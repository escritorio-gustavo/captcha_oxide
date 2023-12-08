macro_rules! declare {
    ($x: ident) => {
        mod $x;
        pub use $x::*;
    };
}

declare!(extract_named_fields);
declare!(extract_struct_data);
declare!(classify_fields);
