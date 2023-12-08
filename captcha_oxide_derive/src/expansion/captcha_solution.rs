use quote::quote;
use syn::{parse::Parser, Field, Fields, ItemStruct};

pub fn captcha_solution_expansion(
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut ast = syn::parse2::<ItemStruct>(item)?;

    let Fields::Named(ref mut fields) = ast.fields else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "This attribute can only be used with structs that contain named fields",
        ));
    };

    let task_id_field = quote! {
        #[serde(default = "Default::default")]
        task_id: u64
    };

    fields.named.push(Field::parse_named.parse2(task_id_field)?);

    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    Ok(quote! {
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
    })
}
