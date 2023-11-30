extern crate proc_macro;

use convert_case::{Case, Casing};
use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parser, parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Field,
    FieldsNamed, ItemStruct, Visibility,
};

#[proc_macro_attribute]
pub fn captcha_solution(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ast = parse_macro_input!(item as ItemStruct);

    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    match ast.fields {
        syn::Fields::Named(ref mut fields) => fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    #[serde(default = "Default::default")]
                    task_id: u64
                })
                .unwrap(),
        ),
        _ => {
            return quote! {
                compile_error!("This attribute can only be used in structs with named fields");
            }
            .into()
        }
    }

    quote! {
        #[derive(Debug, serde::Deserialize)]
        #ast

        impl #impl_generics crate::captcha_types::CaptchaSolution for #ident #type_generics #where_clause {
            fn get_task_id(&self) -> u64 {
                self.task_id
            }

            fn set_task_id(&mut self, task_id: u64) {
                self.task_id = task_id
            }
        }
    }
    .into()
}

#[derive(FromMeta)]
struct ProxyAttribute {
    with_proxy: String,
    proxyless: String,
}

#[proc_macro_attribute]
pub fn proxy_task(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ast = parse_macro_input!(item as ItemStruct);
    let lifetime = ast.generics.lifetimes().next();

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return darling::Error::from(e).write_errors().into();
        }
    };

    let ProxyAttribute {
        with_proxy,
        proxyless,
    } = match ProxyAttribute::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    if lifetime.is_none() {
        return quote! {
            compile_error!("This attribute requires the struct to have a lifetime param");
        }
        .into();
    }

    match ast.fields {
        syn::Fields::Named(ref mut fields) => fields.named.push(
            syn::Field::parse_named
                .parse2(
                    quote! {
                        #[serde(flatten)]
                        pub(super) task_type: task_type::TaskType<#lifetime>
                    }
                    .into(),
                )
                .unwrap(),
        ),
        _ => {
            return quote! {
                compile_error!("This attribute can only be used in structs with named fields");
            }
            .into()
        }
    }

    quote! {
        #[derive(Debug, serde::Serialize)]
        #ast

        mod task_type {

            #[derive(serde::Serialize, Debug)]
            #[serde(tag = "type")]
            pub enum TaskType<'a> {
                #[serde(rename = #proxyless)]
                ProxyLess,

                #[serde(rename = #with_proxy)]
                WithProxy(crate::proxy::Proxy<'a>),
            }

            impl<'a> From<Option<crate::proxy::Proxy<'a>>> for TaskType<'a> {
                fn from(value: Option<crate::proxy::Proxy<'a>>) -> Self {
                    value.map_or(Self::ProxyLess, Self::WithProxy)
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(CaptchaTask, attributes(task))]
pub fn derive_captcha_task(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match derive_captcha_task2(input.into()) {
        Ok(x) => x.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(task))]
struct TaskAttribute {
    timeout: u64,
}

fn derive_captcha_task2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: syn::DeriveInput = syn::parse2(input)?;

    let data = parse_task_ast_data(ast.data.clone())?;
    let ident = ast.ident.clone();
    let where_clause = ast.generics.where_clause.clone();
    let builder_ident = format_ident!("{}Builder", ident);
    let solution_ident = format_ident!("{}Solution", ident);
    let fields = parse_task_fields(data.fields, data.struct_token.span)?.named;

    let TaskAttribute { timeout } = deluxe::extract_attributes(&mut ast)?;

    let fallible = fields
        .iter()
        .any(|x| x.ident.as_ref().unwrap().to_string().ends_with("_url"));

    let build_return = if fallible {
        quote! { Ok(captcha) }
    } else {
        quote! { captcha }
    };

    let build_return_type = if fallible {
        quote! { crate::prelude::Result<#ident<'a>> }
    } else {
        quote! { #ident<'a> }
    };

    let (mandatory_fields, mandatory_field_idents, mandatory_field_types, mandatory_field_build) =
        generate_mandatory_fields(fields.clone());

    let (optional_fields, optional_field_idents, optional_field_types, optional_field_build) =
        generate_optional_fields(fields.clone());

    let lifetime = optional_fields
        .iter()
        .find(|x| x.ty.to_token_stream().to_string().contains("'a"))
        .map(|_| quote! { 'a, });

    let missing_type_decls = mandatory_fields
        .iter()
        .map(|x| x.ident.as_ref().unwrap())
        .map(|x| format_ident!("{}Missing", x.to_string().to_case(Case::Pascal)))
        .map(|x| {
            syn::parse2::<syn::DeriveInput>(quote! {
                pub struct #x;
            })
            .unwrap()
        })
        .collect::<Vec<_>>();

    let missing_type_idents = missing_type_decls
        .iter()
        .map(|x| x.ident.clone())
        .collect::<Vec<_>>();

    let (satisfied_type_decls, satisfied_type_idents, satisfied_type_lifetimes) =
        generate_types_satisfied(&mandatory_fields);

    let mandatory_self = mandatory_field_idents
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            mandatory_field_idents
                .iter()
                .enumerate()
                .map(|(i, y)| {
                    if i == idx {
                        let ty = satisfied_type_idents[idx].clone();
                        quote! {#x: #ty(#x.into())}
                    } else {
                        quote! {#y: self.#y}
                    }
                })
                .chain(optional_field_idents.iter().map(|x| quote! {#x: self.#x}))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let generics = ('T'..='Z')
        .chain('A'..'T')
        .skip(ast.generics.type_params().count())
        .take(mandatory_fields.len())
        .map(|x| x.to_string().parse().unwrap())
        .collect::<Vec<proc_macro2::TokenStream>>();

    let type_params = ('T'..='Z')
        .chain('A'..'T')
        .take(ast.generics.type_params().count())
        .map(|x| x.to_string().parse().unwrap())
        .collect::<Vec<proc_macro2::TokenStream>>();

    let mandatory_setter_return = mandatory_fields
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let mut generics = ('T'..='Z')
                .chain('A'..'T')
                .take(mandatory_fields.len())
                .map(|x| x.to_string().parse().unwrap())
                .collect::<Vec<proc_macro2::TokenStream>>();

            let ty = satisfied_type_idents[idx].clone();
            let lifetime = satisfied_type_lifetimes[idx].clone();
            generics[idx + ast.generics.type_params().count()] = quote! { #ty #lifetime };

            generics
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        mod type_state {
            use super::*;

            #(#missing_type_decls)*
            #(#satisfied_type_decls)*
        }

        mod builder {
            use super::{*, type_state::*};

            pub struct #builder_ident<#lifetime #(#type_params,)* #(#generics),*> #where_clause {
                #(#mandatory_field_idents: #generics,)*
                #(#optional_fields,)*
            }

            impl<'a, #(#type_params),*> #builder_ident<#lifetime #(#type_params,)* #(#satisfied_type_idents #satisfied_type_lifetimes),*> #where_clause {
                pub fn build(self) -> #build_return_type {
                    let captcha = #ident {
                        #(#mandatory_field_build)*
                        #(#optional_field_build)*
                    };

                    #build_return
                }
            }

            impl<#lifetime #(#type_params),*> #builder_ident<#lifetime #(#type_params,)* #(#missing_type_idents),*> #where_clause {
                pub const fn new() -> Self {
                    Self {
                        #(#mandatory_field_idents: #missing_type_idents,)*
                        #(#optional_field_idents: None,)*
                    }
                }
            }

            impl<#lifetime #(#type_params),*> Default for #builder_ident<#lifetime #(#type_params,)* #(#missing_type_idents),*> #where_clause {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<'a, #(#type_params,)* #(#generics),*> #builder_ident<#lifetime #(#type_params,)* #(#generics),*> #where_clause {
                #(
                    pub fn #mandatory_field_idents(self, #mandatory_field_idents: #mandatory_field_types) -> #builder_ident<#lifetime #(#mandatory_setter_return),*> {
                        #builder_ident {
                            #(#mandatory_self,)*
                        }
                    }
                )*

                #(
                    pub fn #optional_field_idents(mut self, #optional_field_idents: #optional_field_types) -> Self {
                        self.#optional_field_idents = #optional_field_idents.map(Into::into);
                        self
                    }
                )*
            }
        }

        use type_state::*;
        impl<'a #(, #type_params)*> crate::captcha_types::CaptchaTask for #ident<'a #(, #type_params)*> #where_clause {
            type Solution = super::solution::#solution_ident<'a>;
            type Builder = builder::#builder_ident<#lifetime #(#type_params,)* #(#missing_type_idents),*>;

            fn get_timeout(&self) -> std::time::Duration {
                std::time::Duration::from_secs(#timeout)
            }
        }
    })
}

fn generate_optional_fields(
    mut optional_fields: Punctuated<Field, Comma>,
) -> (
    Vec<Field>,
    Vec<Ident>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
) {
    let optional_fields = optional_fields
        .iter_mut()
        .filter(|x| {
            let ty = x.ty.to_token_stream().to_string();
            let ident = x.ident.as_ref().unwrap().to_string();
            ty.starts_with("Option") || ident == "task_type"
        })
        .map(|x| {
            let ident = x.ident.as_ref().unwrap().to_string();

            if ident == "task_type" {
                syn::Field {
                    attrs: vec![],
                    vis: Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: Some(format_ident!("proxy")),
                    colon_token: syn::parse2(quote! {:}).unwrap(),
                    ty: syn::parse2(quote! {Option<crate::proxy::Proxy<'a>>}).unwrap(),
                }
            } else {
                x.vis = Visibility::Inherited;
                x.attrs = vec![];
                x.clone()
            }
        })
        .collect::<Vec<_>>();

    let optional_field_idents = optional_fields
        .iter()
        .map(|x| x.ident.as_ref().unwrap())
        .cloned()
        .collect::<Vec<_>>();

    let optional_field_types = optional_fields
        .iter()
        .map(|x| x.ty.clone())
        .map(|x| {
            let mut ty_str = x
                .to_token_stream()
                .to_string()
                .replace("Option", "Option<impl Into");
            ty_str.push('>');

            ty_str.parse().unwrap()
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let optional_field_build = optional_field_idents
        .iter()
        .map(|x| {
            if x.to_string() == "proxy" {
                quote! {task_type: self.proxy.into(),}
            } else {
                quote! {#x: self.#x,}
            }
        })
        .collect::<Vec<_>>();

    (
        optional_fields,
        optional_field_idents,
        optional_field_types,
        optional_field_build,
    )
}

fn generate_mandatory_fields(
    mut mandatory_fields: Punctuated<Field, Comma>,
) -> (
    Vec<Field>,
    Vec<Ident>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
) {
    let mandatory_fields = mandatory_fields
        .iter_mut()
        .filter(|x| !x.ty.to_token_stream().to_string().starts_with("Option"))
        .filter(|x| x.ident.as_ref().unwrap().to_string() != "task_type")
        .map(|x| {
            x.vis = Visibility::Inherited;
            x.attrs = vec![];
            x.clone()
        })
        .collect::<Vec<_>>();

    let mandatory_field_idents = mandatory_fields
        .iter()
        .map(|x| x.ident.as_ref().unwrap())
        .cloned()
        .collect::<Vec<_>>();

    let mandatory_field_types = mandatory_fields
        .iter()
        .map(|x| x.ty.clone())
        .map(|x| {
            let mut ty_str = x.to_token_stream().to_string();

            if ty_str == "Url" || ty_str == "url::Url" {
                quote! {&'a str}
            } else {
                ty_str = format!("impl Into<{}>", ty_str);

                ty_str.parse().unwrap()
            }
        })
        .collect::<Vec<_>>();

    let mandatory_field_build = mandatory_fields
        .iter()
        .map(|x| x.ident.as_ref().unwrap())
        .map(|x| {
            let ident_str = x.to_string();

            if ident_str.ends_with("_url") {
                quote! { #x: url::Url::parse(self.#x.0)?, }
            } else {
                quote! { #x: self.#x.0, }
            }
        })
        .collect::<Vec<_>>();

    (
        mandatory_fields,
        mandatory_field_idents,
        mandatory_field_types,
        mandatory_field_build,
    )
}

fn generate_types_satisfied(
    mandatory_fields: &Vec<Field>,
) -> (
    Vec<DeriveInput>,
    Vec<Ident>,
    Vec<Option<proc_macro2::TokenStream>>,
) {
    let types_satisfied = mandatory_fields
        .iter()
        .map(|x| (x.ident.as_ref().unwrap(), x.ty.clone()))
        .map(|(ident, ty)| {
            (
                format_ident!("{}Provided", ident.to_string().to_case(Case::Pascal)),
                ty,
            )
        })
        .map(|(ident, ty)| {
            let ty_str = ty.to_token_stream().to_string();
            let lifetime = if ty_str.contains("'a") || ty_str.contains("Url") {
                Some(quote! {<'a>})
            } else {
                None
            };

            let ty_param = if ty_str.contains("Url") {
                quote! { &'a str }
            } else {
                ty.to_token_stream()
            };

            syn::parse2::<syn::DeriveInput>(quote! {
                pub struct #ident #lifetime(pub #ty_param);
            })
            .unwrap()
        })
        .collect::<Vec<_>>();

    let types_satisfied_ident = types_satisfied
        .iter()
        .map(|x| x.ident.clone())
        .collect::<Vec<_>>();

    let types_satisfied_lifetime = types_satisfied
        .iter()
        .map(|x| x.generics.lifetimes().next().map(|y| quote! {<#y>}))
        .collect::<Vec<_>>();

    (
        types_satisfied,
        types_satisfied_ident,
        types_satisfied_lifetime,
    )
}

fn parse_task_fields(fields: syn::Fields, span: proc_macro2::Span) -> deluxe::Result<FieldsNamed> {
    match fields {
        syn::Fields::Named(x) => Ok(x),
        _ => Err(syn::Error::new(
            span,
            "The CaptchaTask can only be derived by structs with named fields",
        )),
    }
}

fn parse_task_ast_data(data: syn::Data) -> deluxe::Result<syn::DataStruct> {
    match data {
        syn::Data::Struct(x) => Ok(x),
        syn::Data::Enum(x) => Err(syn::Error::new(
            x.enum_token.span,
            "The CaptchaTask trait can only be derived by structs",
        )),
        syn::Data::Union(x) => Err(syn::Error::new(
            x.union_token.span,
            "The CaptchaTask trait can only be derived by structs",
        )),
    }
}
