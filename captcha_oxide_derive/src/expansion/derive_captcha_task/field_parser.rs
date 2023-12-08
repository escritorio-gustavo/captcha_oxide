use quote::ToTokens;
use syn::{Field, Type, Visibility};

use super::{FieldAttr, TaskFieldAttribute, TaskParseField};

pub(crate) fn parse_field_data(mut field: Field) -> deluxe::Result<Option<FieldAttr>> {
    let attr: TaskFieldAttribute = deluxe::extract_attributes(&mut field)?;

    if attr.default.is_some() {
        return Ok(None);
    }

    let original_ident = field.ident.clone().unwrap();

    let builder_type = match attr.builder_type {
        Some(ref x) => x.clone(),
        None => field.ty.clone(),
    };

    let parse_with = attr.parse_with.as_ref().map(|x| match x {
        TaskParseField::Fallible(parser) => parser.clone(),
        TaskParseField::Infallible(parser) => parser.clone(),
    });

    let is_fallible = match attr.parse_with {
        Some(ref x) => match x {
            TaskParseField::Fallible(_) => true,
            TaskParseField::Infallible(_) => false,
        },
        None => false,
    };

    let impl_into_type: Type = syn::parse_str(&{
        let mut ty = builder_type.to_token_stream().to_string();

        if ty.starts_with("Option") {
            ty = ty.replace("Option", "Option<impl Into");
            ty.push('>');
        } else {
            ty = format!("impl Into<{ty}>");
        }

        ty
    })?;

    let has_lifetime = builder_type.to_token_stream().to_string().contains('\'');

    field.ident = match attr.rename {
        Some(ref x) => Some(x.clone()),
        None => field.ident,
    };
    field.attrs = field
        .attrs
        .iter()
        .filter(|x| match x.meta {
            syn::Meta::NameValue(ref x) => x.path.is_ident("doc"),
            _ => false,
        })
        .cloned()
        .collect();
    field.ty = builder_type.clone();
    field.vis = Visibility::Inherited;

    Ok(Some(FieldAttr {
        field,
        attr,
        builder_type,
        impl_into_type,
        parse_with,
        is_fallible,
        has_lifetime,
        original_ident,
    }))
}
