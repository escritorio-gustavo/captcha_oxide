use quote::quote;
use syn::{Field, Ident, Path, Type};

use crate::util::{classify_fields, extract_named_fields, extract_struct_data};

use builder::create_builder;
use type_state::create_type_state;

use self::builder::Builder;

mod builder;
mod field_parser;
mod type_state;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(task))]
struct TaskStructAttribute {
    timeout: u64,
    solution: Type,

    #[deluxe(rename = crate, default = syn::parse2(quote!{captcha_oxide}).unwrap())]
    crate_path: Path,
}

#[derive(deluxe::ExtractAttributes, Clone)]
#[deluxe(attributes(task))]
pub struct TaskFieldAttribute {
    pub rename: Option<Ident>,
    pub builder_type: Option<Type>,

    pub default: Option<Path>,

    #[deluxe()]
    pub parse_with: Option<TaskParseField>,
}

#[derive(deluxe::ParseMetaItem, Clone)]
pub enum TaskParseField {
    Fallible(Parser),
    Infallible(Parser),
}

#[derive(deluxe::ParseMetaItem, Clone)]
pub struct Parser {
    pub path: Path,
    pub parse_ref: deluxe::Flag,
}

#[derive(Clone)]
pub struct FieldAttr {
    pub field: Field,
    pub builder_type: Type,
    pub impl_into_type: Type,
    pub attr: TaskFieldAttribute,
    pub parse_with: Option<Parser>,
    pub is_fallible: bool,
    pub has_lifetime: bool,
    pub original_ident: Ident,
}

#[derive(Clone)]
pub struct SkippedField {
    pub ident: Ident,
    pub default_path: Path,
}

pub fn derive_captcha_task_expansion(
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut ast: syn::DeriveInput = syn::parse2(input)?;

    let TaskStructAttribute {
        timeout,
        crate_path,
        solution,
    } = deluxe::extract_attributes(&mut ast)?;

    let data_struct = extract_struct_data(ast.data)?;

    let fields =
        extract_named_fields(data_struct.fields, data_struct.struct_token.span)?.into_iter();

    let skipped_fields = fields
        .clone()
        .filter_map(|mut field| {
            let attr: TaskFieldAttribute = deluxe::extract_attributes(&mut field).ok()?;

            Some(SkippedField {
                ident: field.ident.unwrap(),
                default_path: attr.default?,
            })
        })
        .collect::<Vec<_>>();

    let fields = fields
        .map(field_parser::parse_field_data)
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>, _>>()?;

    let classified_fields = classify_fields(fields);

    let ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let type_state_pairs = create_type_state(
        &classified_fields.required,
        ast.generics.type_params().count(),
    );

    let type_state_decls = type_state_pairs
        .iter()
        .flat_map(|pair| vec![&pair.missing.declaration, &pair.provided.declaration]);

    let Builder {
        declaration: builder_decl,
        impl_build,
        impl_constructor,
        impl_methods,
        path,
    } = create_builder(
        &crate_path,
        ident,
        &ast.generics,
        classified_fields,
        skipped_fields,
        &type_state_pairs,
    );

    Ok(quote! {
        mod type_state {
            use super::*;

            #(#type_state_decls)*
        }
        use type_state::*;

        mod builder {
            use super::*;

            #builder_decl

            #impl_build

            #impl_constructor

            #impl_methods
        }

        use builder::*;

        impl #impl_generics #crate_path::CaptchaTask for #ident #ty_generics #where_clause {
            type Solution = #solution;
            type Builder = #path;

            fn get_timeout(&self) -> std::time::Duration {
                std::time::Duration::from_secs(#timeout)
            }
        }

        // pub mod testing {
        pub struct Test;

        impl Test {
            pub fn foo() -> usize {
                42
            }
        }
        // }
    })
}
