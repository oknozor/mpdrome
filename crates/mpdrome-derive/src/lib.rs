use convert_case::{Case, Casing};
use proc_macro::TokenStream;

use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, parse::Parse, parse_macro_input};

use crate::{attributes::CaseAttribute, type_info::TypeInfo};

mod attributes;
mod type_info;

#[proc_macro_derive(MpdResponse, attributes(mpd))]
pub fn derive_mpd_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut field_serializers = Vec::new();

    if let Data::Struct(ds) = input.data {
        if let Fields::Named(fields) = ds.fields {
            for field in fields.named.iter() {
                let type_info = TypeInfo::from(field);
                let field_name = field.ident.as_ref().unwrap();
                let field_str = field_name.to_string();

                let name = field_str
                    .strip_prefix("r#")
                    .map(ToString::to_string)
                    .unwrap_or(field_str);

                let attr = get_mpd_attr(field)
                    .and_then(|attr| attr.parse_args_with(attributes::MpdAttributes::parse).ok())
                    .unwrap_or_default();

                let name = attr.rename.unwrap_or_else(|| match attr.case {
                    CaseAttribute::PascalCase => name.to_case(Case::Pascal),
                    CaseAttribute::UpperSnake => name.to_case(Case::UpperSnake),
                    CaseAttribute::Unchanged => name,
                });

                if attr.binary {
                    field_serializers.push(quote! {
                        write!(w, "binary: {}\n", self.#field_name.len())?;
                        w.write_all(&self.#field_name)?;
                        w.write_all(b"\n")?;
                    });
                } else {
                    match type_info {
                        TypeInfo::Option => field_serializers.push(quote! {
                            if let Some(ref value) = self.#field_name {
                                write!(w, "{0}: {1}\n", #name, value)?;
                            }
                        }),
                        TypeInfo::Vec => field_serializers.push(quote! {
                            for value in &self.#field_name {
                                write!(w, "{0}: {1}\n", #name, value)?;
                            }
                        }),
                        TypeInfo::ToString => field_serializers.push(quote! {
                            write!(w, "{0}: {1}\n", #name, self.#field_name)?;
                        }),
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl mpdrome_macro::ToMpdResponse for #name {
            fn write_response<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
                use std::io::Write;
                #(#field_serializers)*
                write!(w, "OK\n")?;
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_mpd_attr(field: &syn::Field) -> Option<&Attribute> {
    field.attrs.iter().find(|attr| attr.path().is_ident("mpd"))
}
