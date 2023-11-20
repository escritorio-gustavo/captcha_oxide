extern crate proc_macro;

use darling::{ast::NestedMeta, FromMeta};
use quote::quote;
use syn::{parse::Parser, parse_macro_input, parse_quote, ItemStruct};

#[proc_macro_attribute]
pub fn captcha_solution(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ast = parse_macro_input!(item as ItemStruct);

    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    ast.attrs.splice(
        0..0,
        vec![parse_quote! {#[derive(Debug, serde::Deserialize)]}],
    );
    ast.attrs
        .push(parse_quote! {#[serde(rename_all = "camelCase")]});

    match ast.fields {
        syn::Fields::Named(ref mut fields) => fields.named.push(
            syn::Field::parse_named
                .parse2(
                    quote! {
                        #[serde(default = "Default::default")]
                        task_id: u64
                    }
                    .into(),
                )
                .unwrap(),
        ),
        _ => {
            return quote! {
                compile_error!("This attribute can only be used in structs with named fields")
            }
            .into()
        }
    }

    quote! {
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

    ast.attrs.splice(
        0..0,
        vec![
            parse_quote! {#[derive(Debug, serde::Serialize)]},
            parse_quote! {#[serde(rename_all = "camelCase")]},
        ],
    );

    if lifetime.is_none() {
        return quote! {
            compile_error!("This attribute requires the struct to have a lifetime param")
        }
        .into();
    }

    match ast.fields {
        syn::Fields::Named(ref mut fields) => fields.named.push(
            syn::Field::parse_named
                .parse2(
                    quote! {
                        #[serde(flatten)]
                        pub(super) task_type: TaskType<#lifetime>
                    }
                    .into(),
                )
                .unwrap(),
        ),
        _ => {
            return quote! {
                compile_error!("This attribute can only be used in structs with named fields")
            }
            .into()
        }
    }

    quote! {
        #ast

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
    .into()
}
