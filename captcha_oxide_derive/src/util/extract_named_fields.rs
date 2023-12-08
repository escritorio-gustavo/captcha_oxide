use syn::{token::Comma, Field};

pub fn extract_named_fields(
    fields: syn::Fields,
    span: proc_macro2::Span,
) -> deluxe::Result<syn::punctuated::Punctuated<Field, Comma>> {
    match fields {
        syn::Fields::Named(x) => Ok(x.named),
        _ => Err(syn::Error::new(
            span,
            "The CaptchaTask can only be derived by structs with named fields",
        )),
    }
}
