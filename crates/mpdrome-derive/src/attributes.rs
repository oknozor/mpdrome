use syn::{
    Token,
    parse::{Parse, ParseStream},
};

#[derive(Default)]
pub struct MpdAttributes {
    pub binary: bool,
    pub case: CaseAttribute,
    pub rename: Option<String>,
}

#[derive(Default)]
pub enum CaseAttribute {
    PascalCase,
    UpperSnake,
    #[default]
    Unchanged,
}

impl Parse for MpdAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = MpdAttributes::default();

        while let Ok(ident) = input.parse::<syn::Ident>() {
            match ident.to_string().as_str() {
                "case" => attributes.case = Self::parse_case(input)?,
                "binary" => attributes.binary = true,
                "rename" => attributes.rename = Self::parse_rename(input)?,
                other => panic!(
                    "unexpected attribute {}, supported attributes are : ['case', 'rename', 'binary']",
                    other
                ),
            }

            if input.peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            }
        }

        Ok(attributes)
    }
}

impl MpdAttributes {
    pub fn parse_case(input: ParseStream) -> syn::Result<CaseAttribute> {
        input.parse::<Token!(=)>()?;
        let case = input.parse::<syn::LitStr>()?;
        match case.value().as_str() {
            "PascalCase" => Ok(CaseAttribute::PascalCase),
            "UPPER_SNAKE" => Ok(CaseAttribute::UpperSnake),
            _ => Ok(CaseAttribute::Unchanged),
        }
    }

    pub fn parse_rename(input: ParseStream) -> syn::Result<Option<String>> {
        input.parse::<Token!(=)>()?;
        Ok(input
            .parse::<syn::LitStr>()
            .ok()
            .map(|lit| lit.value().as_str().to_string()))
    }
}
