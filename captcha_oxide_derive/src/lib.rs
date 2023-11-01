extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Solution)]
pub fn derive_solution(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics Solution for #ident #type_generics #where_clause {
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
