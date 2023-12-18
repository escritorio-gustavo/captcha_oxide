extern crate proc_macro;

mod expansion;
mod util;

trait ToTokenStream {
    fn resolve(self) -> proc_macro::TokenStream;
}

impl ToTokenStream for syn::Result<proc_macro2::TokenStream> {
    fn resolve(self) -> proc_macro::TokenStream {
        match self {
            Ok(x) => x.into(),
            Err(e) => e.into_compile_error().into(),
        }
    }
}

// #[proc_macro_attribute]
// pub fn captcha_solution(
//     _: proc_macro::TokenStream,
//     item: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     expansion::captcha_solution::captcha_solution_expansion(item.into()).resolve()
// }

#[proc_macro_attribute]
pub fn proxy_task(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    expansion::proxy_task::proxy_task_expansion(args.into(), item.into()).resolve()
}

#[proc_macro_derive(CaptchaTask, attributes(task))]
pub fn derive_captcha_task(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expansion::derive_captcha_task::derive_captcha_task_expansion(input.into()).resolve()
}
