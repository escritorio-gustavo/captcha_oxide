use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Brace, Data, DataStruct, DeriveInput, Fields, FieldsNamed,
    GenericParam, Generics, Ident, Lifetime, LifetimeParam, Path, PathSegment, Token, TypeParam,
    Visibility,
};

use crate::util::ClassifiedFields;

use super::{type_state::TypeStatePair, FieldAttr, SkippedField};

pub struct Builder {
    pub declaration: DeriveInput,
    pub impl_build: proc_macro2::TokenStream,
    pub impl_constructor: proc_macro2::TokenStream,
    pub impl_methods: proc_macro2::TokenStream,
    pub path: Path,
}

pub struct BuilderConstructor {
    impl_constructor: proc_macro2::TokenStream,
    path: Path,
}

pub fn create_builder(
    crate_path: &Path,
    ident: &Ident,
    task_generics: &Generics,
    classified_fields: ClassifiedFields,
    skipped_fields: Vec<SkippedField>,
    type_state_pairs: &[TypeStatePair],
) -> Builder {
    let call_site = proc_macro2::Span::call_site();

    let builder_ident = format_ident!("{}Builder", ident);
    let lifetime: Option<LifetimeParam> =
        if classified_fields.optional.iter().any(|x| x.has_lifetime) {
            Some(LifetimeParam {
                attrs: vec![],
                lifetime: Lifetime {
                    apostrophe: call_site,
                    ident: format_ident!("a"),
                },
                colon_token: None,
                bounds: Punctuated::new(),
            })
        } else {
            None
        };

    let builder_decl_generics = Generics {
        lt_token: Some(Token![<](call_site)),
        params: vec![lifetime.as_ref().map(|x| GenericParam::Lifetime(x.clone()))]
            .into_iter()
            .flatten()
            .chain(
                task_generics
                    .type_params()
                    .cloned()
                    .map(|mut x| {
                        x.eq_token = None;
                        x.default = None;

                        x
                    })
                    .map(GenericParam::Type),
            )
            .chain(type_state_pairs.iter().map(|x| x.generic.clone()))
            .collect(),
        gt_token: Some(Token![>](call_site)),
        where_clause: task_generics.where_clause.clone(),
    };

    let decl = DeriveInput {
        attrs: vec![],
        vis: Visibility::Public(Token![pub](call_site)),
        ident: builder_ident.clone(),
        generics: builder_decl_generics.clone(),
        data: Data::Struct(DataStruct {
            struct_token: Token![struct](call_site),
            fields: Fields::Named(FieldsNamed {
                brace_token: Brace(call_site),
                named: type_state_pairs
                    .iter()
                    .map(|x| x.field.clone())
                    .chain(classified_fields.optional.iter().map(|x| x.field.clone()))
                    .collect(),
            }),
            semi_token: None,
        }),
    };

    let BuilderConstructor {
        impl_constructor,
        path,
    } = create_constructor(
        &lifetime,
        task_generics,
        call_site,
        type_state_pairs,
        &classified_fields.optional,
        &builder_ident,
    );

    let impl_build = create_build(
        &lifetime,
        task_generics,
        type_state_pairs,
        &classified_fields,
        skipped_fields,
        ident,
        crate_path,
        &builder_ident,
    );

    let impl_methods = create_methods(
        &lifetime,
        builder_decl_generics,
        task_generics,
        type_state_pairs,
        classified_fields,
        builder_ident,
    );

    Builder {
        declaration: decl,
        impl_build,
        impl_constructor,
        impl_methods,
        path,
    }
}

fn create_methods(
    lifetime: &Option<LifetimeParam>,
    builder_decl_generics: Generics,
    task_generics: &Generics,
    type_state_pairs: &[TypeStatePair],
    classified_fields: ClassifiedFields,
    builder_ident: Ident,
) -> proc_macro2::TokenStream {
    let impl_generics = Generics {
        lt_token: builder_decl_generics.lt_token.clone(),
        params: task_generics
            .lifetimes()
            .cloned()
            .map(GenericParam::Lifetime)
            .chain(
                builder_decl_generics
                    .type_params()
                    .cloned()
                    .map(GenericParam::Type),
            )
            .collect(),
        gt_token: builder_decl_generics.gt_token.clone(),
        where_clause: builder_decl_generics.where_clause.clone(),
    };
    let impl_generics = impl_generics.split_for_impl().0;

    let (_, ty_generics, where_clause) = builder_decl_generics.split_for_impl();

    let task_generic_count =
        task_generics.type_params().count() + builder_decl_generics.lifetimes().count();
    let required_methods = type_state_pairs.iter().enumerate().map(|(i, x)| {
        let ident = x.field.ident.as_ref().unwrap();
        let field_attr = classified_fields.required[i].clone();
        let ty = field_attr.impl_into_type;
        let doc_attr = &classified_fields.required[i].field.attrs;
        let mut ty_generics = vec![lifetime.as_ref().map(ToTokens::to_token_stream)]
            .into_iter()
            .flatten()
            .chain(
                task_generics
                    .type_params()
                    .cloned()
                    .map(|mut t| {
                        t.eq_token = None;
                        t.default = None;

                        t
                    })
                    .map(|x| ToTokens::to_token_stream(&x)),
            )
            .chain(
                type_state_pairs
                    .iter()
                    .map(|pair| pair.generic.to_token_stream()),
            )
            .collect::<Vec<_>>();
        ty_generics[i + task_generic_count] = x.provided.path.to_token_stream();

        let mut required_set = classified_fields
            .required
            .iter()
            .map(|x| {
                let ident = x.field.ident.as_ref().unwrap();

                quote! {#ident: self.#ident}
            })
            .collect::<Vec<_>>();

        required_set[i] = {
            let ty = &x.provided.ident;
            quote! {#ident: #ty(#ident.into()) }
        };

        let optional_set = classified_fields.optional.iter().map(|x| {
            let ident = x.field.ident.as_ref().unwrap();

            quote! {#ident: self.#ident}
        });

        quote! {
            #(#doc_attr)*
            pub fn #ident(self, #ident: #ty) -> #builder_ident<#(#ty_generics),*> {
                #builder_ident {
                    #(#required_set,)*
                    #(#optional_set,)*
                }
            }
        }
    });

    let optional_methods = classified_fields.optional.iter().map(|x| {
        let ident = x.field.ident.as_ref().unwrap();
        let ty = &x.impl_into_type;
        let doc_attr = &x.field.attrs;

        quote! {
            #(#doc_attr)*
            pub fn #ident(mut self, #ident: #ty) -> Self {
                self.#ident = #ident.map(Into::into);
                self
            }
        }
    });

    quote! {
        impl #impl_generics #builder_ident #ty_generics #where_clause {
            #(#required_methods)*

            #(#optional_methods)*
        }
    }
}

fn create_build(
    lifetime: &Option<LifetimeParam>,
    task_generics: &Generics,
    type_state_pairs: &[TypeStatePair],
    classified_fields: &ClassifiedFields,
    skipped_fields: Vec<SkippedField>,
    ident: &Ident,
    crate_path: &Path,
    builder_ident: &Ident,
) -> proc_macro2::TokenStream {
    let build_ty_generics = vec![lifetime.as_ref().map(ToTokens::to_token_stream)]
        .into_iter()
        .flatten()
        .chain(
            task_generics
                .type_params()
                .cloned()
                .map(|mut x| {
                    x.eq_token = None;
                    x.default = None;

                    x
                })
                .map(|x| ToTokens::to_token_stream(&x)),
        )
        .chain(
            type_state_pairs
                .iter()
                .map(|x| x.provided.path.clone())
                .map(|x| ToTokens::to_token_stream(&x)),
        )
        .collect::<Vec<_>>();

    let (task_impl_generics, task_ty_generics, where_clause) = task_generics.split_for_impl();

    let is_fallible = classified_fields
        .required
        .iter()
        .chain(classified_fields.optional.iter())
        .any(|x| x.is_fallible);

    let (return_type, return_value) = if is_fallible {
        (
            quote! { Result<#ident #task_ty_generics, #crate_path::Error> },
            quote! { Ok(captcha) },
        )
    } else {
        (quote! { #ident #task_ty_generics }, quote! { captcha })
    };

    let required_build = classified_fields.required.iter().map(|x| {
        let ident = x.original_ident.clone();
        let builder_ident = x.field.ident.as_ref().unwrap();
        let question_mark = if x.is_fallible {
            Some(quote! {?})
        } else {
            None
        };
        let value = match x.parse_with {
            Some(ref parser) => {
                let path = &parser.path;

                if parser.parse_ref.is_set() {
                    quote! { #path(&self.#builder_ident.0) }
                } else {
                    quote! { #path(self.#builder_ident.0) }
                }
            }
            None => quote! { self.#builder_ident.0 },
        };

        quote! { #ident: #value #question_mark.into() }
    });

    let optional_build = classified_fields.optional.iter().map(|x| {
        let ident = x.original_ident.clone();
        let builder_ident = x.field.ident.as_ref().unwrap();

        let question_mark = if x.is_fallible {
            Some(quote! {.transpose()?})
        } else {
            None
        };
        let value = match x.parse_with {
            Some(ref parser) => {
                let path = &parser.path;
                if parser.parse_ref.is_set() {
                    quote! { self.#builder_ident.as_ref().map(#path) }
                } else {
                    quote! { self.#builder_ident.map(#path) }
                }
            }
            None => quote! { self.#builder_ident },
        };

        quote! { #ident: #value #question_mark.map(Into::into).into() }
    });

    let skipped_build = skipped_fields.iter().map(|x| {
        let ident = &x.ident;
        let path = &x.default_path;

        quote! { #ident: #path() }
    });

    quote! {
        impl #task_impl_generics #builder_ident <#(#build_ty_generics),*> #where_clause {
            pub fn build(self) -> #return_type {
                let captcha = #ident {
                    #(#required_build,)*
                    #(#optional_build,)*
                    #(#skipped_build,)*
                };

                #return_value
            }
        }
    }
}

fn create_constructor(
    lifetime: &Option<LifetimeParam>,
    task_generics: &Generics,
    call_site: proc_macro2::Span,
    type_state_pairs: &[TypeStatePair],
    optional_fields: &[FieldAttr],
    builder_ident: &Ident,
) -> BuilderConstructor {
    let constructor_impl_has_generics =
        lifetime.is_some() || task_generics.type_params().count() > 0;

    let constructor_impl_generics = Generics {
        lt_token: if constructor_impl_has_generics {
            Some(Token![<](call_site))
        } else {
            None
        },
        params: vec![lifetime.as_ref().map(|x| GenericParam::Lifetime(x.clone()))]
            .into_iter()
            .flatten()
            .chain(task_generics.type_params().cloned().map(GenericParam::Type))
            .collect(),
        gt_token: if constructor_impl_has_generics {
            Some(Token![>](call_site))
        } else {
            None
        },
        where_clause: task_generics.where_clause.clone(),
    };
    let constructor_impl_generics = constructor_impl_generics.split_for_impl().0;

    let constructor_ty_generics = Generics {
        lt_token: Some(Token![<](call_site)),
        params: vec![lifetime.as_ref().map(|x| GenericParam::Lifetime(x.clone()))]
            .into_iter()
            .flatten()
            .chain(task_generics.type_params().cloned().map(GenericParam::Type))
            .chain(type_state_pairs.iter().map(|x| {
                GenericParam::Type(TypeParam {
                    attrs: vec![],
                    ident: x.missing.ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                    eq_token: None,
                    default: None,
                })
            }))
            .collect(),
        gt_token: Some(Token![>](call_site)),
        where_clause: task_generics.where_clause.clone(),
    };
    let constructor_ty_generics = constructor_ty_generics.split_for_impl().1;

    let where_clause = task_generics.where_clause.clone();

    let path = Path {
        leading_colon: None,
        segments: vec![
            PathSegment {
                ident: format_ident!("builder"),
                arguments: syn::PathArguments::None,
            },
            PathSegment {
                ident: builder_ident.clone(),
                arguments: syn::PathArguments::AngleBracketed(
                    syn::parse2(constructor_ty_generics.to_token_stream()).unwrap(),
                ),
            },
        ]
        .into_iter()
        .collect(),
    };

    let required_initializers = type_state_pairs.iter().map(|x| {
        let ident = x.field.ident.as_ref().unwrap();
        let value = x.missing.ident.clone();

        quote! { #ident: #value }
    });

    let optional_initializers = optional_fields.iter().map(|x| {
        let ident = x.field.ident.as_ref().unwrap();
        quote! { #ident: None }
    });

    let impl_constructor = quote! {
        impl #constructor_impl_generics #builder_ident #constructor_ty_generics #where_clause {
            pub const fn new() -> Self {
                Self {
                    #(#required_initializers,)*
                    #(#optional_initializers,)*
                }
            }
        }

        impl #constructor_impl_generics Default for #builder_ident #constructor_ty_generics #where_clause {
            fn default() -> Self {
                Self::new()
            }
        }
    };

    BuilderConstructor {
        impl_constructor,
        path,
    }
}
