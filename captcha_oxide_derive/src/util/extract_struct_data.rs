use syn::{
    token::{Enum, Union},
    DataEnum, DataUnion,
};

pub fn extract_struct_data(data: syn::Data) -> deluxe::Result<syn::DataStruct> {
    match data {
        syn::Data::Struct(x) => Ok(x),
        syn::Data::Enum(DataEnum {
            enum_token: Enum { span },
            ..
        })
        | syn::Data::Union(DataUnion {
            union_token: Union { span },
            ..
        }) => Err(syn::Error::new(
            span,
            "The CaptchaTask trait can only be derived by structs",
        )),
    }
}
