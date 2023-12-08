use convert_case::Casing;
use quote::format_ident;
use syn::{
    punctuated::Punctuated, token::Paren, AngleBracketedGenericArguments, Data, DataStruct,
    DeriveInput, Field, FieldMutability, Fields, FieldsUnnamed, GenericArgument, GenericParam,
    Generics, Ident, Lifetime, LifetimeParam, Path, PathArguments, PathSegment, Token, Type,
    TypeParam, TypePath, Visibility,
};

use super::FieldAttr;

pub struct TypeState {
    pub declaration: DeriveInput,
    pub ident: Ident,
    pub path: Path,
}

pub struct TypeStatePair {
    pub missing: TypeState,
    pub provided: TypeState,
    pub field: Field,
    pub generic: GenericParam,
}

fn create_missing_type(field: &FieldAttr) -> TypeState {
    let call_site = proc_macro2::Span::call_site();

    let ident = field.field.ident.as_ref().unwrap();
    let ident = format_ident!(
        "{}Missing",
        ident.to_string().to_case(convert_case::Case::Pascal)
    );

    TypeState {
        declaration: DeriveInput {
            attrs: vec![],
            vis: Visibility::Public(Token![pub](call_site)),
            ident: ident.clone(),
            generics: Generics {
                lt_token: None,
                params: Punctuated::new(),
                gt_token: None,
                where_clause: None,
            },
            data: Data::Struct(DataStruct {
                struct_token: Token![struct](call_site),
                fields: Fields::Unit,
                semi_token: Some(Token![;](call_site)),
            }),
        },
        ident: ident.clone(),
        path: Path {
            leading_colon: None,
            segments: vec![PathSegment {
                ident: ident.clone(),
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        },
    }
}

fn create_provided_type(field: &FieldAttr) -> TypeState {
    let call_site = proc_macro2::Span::call_site();

    let ident = field.field.ident.as_ref().unwrap();
    let ident = format_ident!(
        "{}Provided",
        ident.to_string().to_case(convert_case::Case::Pascal)
    );

    let lifetime = if field.has_lifetime {
        Some(Lifetime {
            apostrophe: call_site,
            ident: format_ident!("a"),
        })
    } else {
        None
    };

    TypeState {
        declaration: DeriveInput {
            attrs: vec![],
            vis: Visibility::Public(Token![pub](call_site)),
            ident: ident.clone(),
            generics: Generics {
                lt_token: None,
                params: match lifetime {
                    Some(ref x) => vec![GenericParam::Lifetime(LifetimeParam {
                        attrs: vec![],
                        lifetime: x.clone(),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    })]
                    .into_iter()
                    .collect(),
                    None => Punctuated::new(),
                },
                gt_token: None,
                where_clause: None,
            },
            data: Data::Struct(DataStruct {
                struct_token: Token![struct](call_site),
                fields: Fields::Unnamed(FieldsUnnamed {
                    paren_token: Paren(call_site),
                    unnamed: vec![Field {
                        attrs: vec![],
                        vis: Visibility::Public(Token![pub](call_site)),
                        mutability: FieldMutability::None,
                        ident: None,
                        colon_token: None,
                        ty: field.builder_type.clone(),
                    }]
                    .into_iter()
                    .collect(),
                }),
                semi_token: Some(Token![;](call_site)),
            }),
        },
        ident: ident.clone(),
        path: Path {
            leading_colon: None,
            segments: vec![PathSegment {
                ident: ident.clone(),
                arguments: match lifetime {
                    Some(ref x) => PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Token![<](call_site),
                        args: vec![GenericArgument::Lifetime(x.clone())]
                            .into_iter()
                            .collect(),
                        gt_token: Token![>](call_site),
                    }),
                    None => PathArguments::None,
                },
            }]
            .into_iter()
            .collect(),
        },
    }
}

pub fn create_type_state(
    required_fields: &[FieldAttr],
    task_type_param_count: usize,
) -> Vec<TypeStatePair> {
    required_fields
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let generic_ident = format_ident!(
                "{}",
                ('T'..'Z')
                    .chain('A'..'T')
                    .skip(task_type_param_count + i)
                    .next()
                    .unwrap()
            );

            TypeStatePair {
                missing: create_missing_type(x),
                provided: create_provided_type(x),
                field: Field {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: x.field.ident.clone(),
                    colon_token: x.field.colon_token.clone(),
                    ty: Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: vec![PathSegment {
                                ident: generic_ident.clone(),
                                arguments: PathArguments::None,
                            }]
                            .into_iter()
                            .collect(),
                        },
                    }),
                },
                generic: GenericParam::Type(TypeParam {
                    attrs: vec![],
                    ident: generic_ident.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                    eq_token: None,
                    default: None,
                }),
            }
        })
        .collect()
}
