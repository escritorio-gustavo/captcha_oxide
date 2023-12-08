use darling::{ast::NestedMeta, FromMeta};
use quote::quote;
use syn::{parse::Parser, AttrStyle, Error, Field, Fields, ItemStruct, Path};

#[derive(FromMeta)]
struct ProxyAttribute {
    with_proxy: String,
    proxyless: String,

    #[darling(rename = "crate")]
    crate_path: Option<Path>,
}

pub fn proxy_task_expansion(
    args: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut ast = syn::parse2::<ItemStruct>(item)?;

    let derives_captcha_task = ast.attrs.iter().any(|x| {
        x.style == AttrStyle::Outer
            && match x.meta {
                syn::Meta::List(ref list) => {
                    let tokens = list.tokens.to_string();
                    list.path.is_ident("derive") && tokens.contains("CaptchaTask")
                }
                _ => false,
            }
    });

    let attr_args = NestedMeta::parse_meta_list(args.into())?;

    let ProxyAttribute {
        with_proxy,
        proxyless,
        crate_path,
    } = ProxyAttribute::from_list(&attr_args)?;
    let crate_path = crate_path.unwrap_or(syn::parse2(quote! {captcha_oxide}).unwrap());

    let Some(lifetime) = ast.generics.lifetimes().next() else {
        return Err(Error::new(
            proc_macro2::Span::call_site(),
            "This attribute requires the struct to have a lifetime param",
        ));
    };

    let task_attr = if derives_captcha_task {
        Some(
            quote! {#[task(rename = proxy, builder_type = Option<#crate_path::proxy::Proxy<#lifetime>>)]},
        )
    } else {
        None
    };

    let Fields::Named(ref mut fields) = ast.fields else {
        return Err(Error::new(
            proc_macro2::Span::call_site(),
            "This attribute can only be used in structs with named fields",
        ));
    };

    let task_type_field = quote! {
        #[serde(flatten)]
        #task_attr
        pub(super) task_type: task_type::TaskType<#lifetime>
    };

    fields
        .named
        .push(Field::parse_named.parse2(task_type_field)?);

    Ok(quote! {
        #[derive(Debug)]
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
    })
}
