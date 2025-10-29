use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, Meta, Type, parse::Parse, parse_macro_input};

#[proc_macro_derive(MpdResponse, attributes(mpd))]
pub fn derive_mpd_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut field_serializers = Vec::new();

    if let Data::Struct(ds) = input.data {
        if let Fields::Named(fields) = ds.fields {
            for field in fields.named.iter() {
                let is_vec = is_type(field, "Vec");
                let is_option = is_type(field, "Option");

                let field_name = field.ident.as_ref().unwrap();
                let field_str = field_name.to_string();

                let is_binary = field.attrs.iter().any(|attr| {
                    if attr.path().is_ident("mpd") {
                        if let Ok(arg) = attr.parse_args_with(Meta::parse) {
                            arg.path().is_ident("binary")
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                let name = field_str
                    .strip_prefix("r#")
                    .map(ToString::to_string)
                    .unwrap_or(field_str);

                if is_binary {
                    field_serializers.push(quote! {
                        write!(w, "binary: {}\n", self.#field_name.len())?;
                        w.write_all(&self.#field_name)?;
                        w.write_all(b"\n")?;
                    });
                } else if is_option {
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            write!(w, "{0}: {1}\n", #name, value)?;
                        }
                    });
                } else if is_vec {
                    field_serializers.push(quote! {
                        for value in &self.#field_name {
                            write!(w, "{0}: {1}\n", #name, value)?;
                        }
                    });
                } else {
                    field_serializers.push(quote! {
                        write!(w, "{0}: {1}\n", #name, self.#field_name)?;
                    });
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

fn is_type(field: &syn::Field, name: &str) -> bool {
    if let Type::Path(path) = &field.ty {
        let ty = path.to_token_stream().to_string();
        let option = ty.split_once("<").map(|(l, _)| l.trim());
        match option {
            Some(ty) if ty == name => true,
            _ => false,
        }
    } else {
        false
    }
}
