use quote::ToTokens;
use syn::{Field, Type};

pub enum TypeInfo {
    Option,
    Vec,
    ToString,
}

impl From<&Field> for TypeInfo {
    fn from(field: &Field) -> Self {
        if let Type::Path(path) = &field.ty {
            let ty = path.to_token_stream().to_string();
            let option = ty.split_once("<").map(|(l, _)| l.trim());
            match option {
                Some(ty) if ty == "Option" => TypeInfo::Option,
                Some(ty) if ty == "Vec" => TypeInfo::Vec,
                _ => TypeInfo::ToString,
            }
        } else {
            TypeInfo::ToString
        }
    }
}
