use syn::Type;

use crate::expansion::derive_captcha_task::FieldAttr;

pub struct ClassifiedFields {
    pub required: Vec<FieldAttr>,
    pub optional: Vec<FieldAttr>,
}

fn is_required(x: &&FieldAttr) -> bool {
    match x.field.ty.clone() {
        Type::Path(ref ty) => ty.path.segments.first().unwrap().ident != "Option",
        _ => true,
    }
}

pub fn classify_fields(fields: Vec<FieldAttr>) -> ClassifiedFields {
    let required = fields.iter().filter(is_required).cloned().collect();
    let optional = fields.iter().filter(|x| !is_required(x)).cloned().collect();

    ClassifiedFields { required, optional }
}
