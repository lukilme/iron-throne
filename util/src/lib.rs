use syn::DeriveInput;
use syn::{Data, Meta, LitStr, ExprLit, Expr};
use syn::punctuated::Punctuated;
use syn::Token;
use quote::ToTokens;

#[derive(Debug)]
pub struct FieldInfo {
    pub column_name: String,
    pub is_primary_key: bool,
    pub rust_type: String,
}

pub fn parse_attributes(input: &DeriveInput) -> (String, Vec<FieldInfo>) {
    let table_name = extract_table_name(input);
    let fields_info = extract_fields_info(input);
    (table_name, fields_info)
}

fn extract_table_name(input: &DeriveInput) -> String {
    for attr in &input.attrs {
        if attr.path().is_ident("table_name") {
            return attr.parse_args::<LitStr>()
                .expect("table_name deve ser uma string")
                .value();
        }
    }
    
    panic!("Atributo #[table_name(\"nome_da_tabela\")] não encontrado");
}

fn extract_fields_info(input: &DeriveInput) -> Vec<FieldInfo> {
    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => panic!("Apenas structs são suportadas"),
    };

    fields.iter()
        .map(|field| {
            let mut column_name = field.ident.as_ref().unwrap().to_string();
            let mut is_primary_key = false;

            for attr in &field.attrs {
                if attr.path().is_ident("column") {
                    let meta = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                        .expect("Formato inválido para atributo column");

                    for m in meta {
                        match m {
                            Meta::NameValue(nv) if nv.path.is_ident("name") => {
                                if let Expr::Lit(ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                                    column_name = s.value();
                                }
                            }
                            Meta::Path(p) if p.is_ident("primary_key") => {
                                is_primary_key = true;
                            }
                            _ => {}
                        }
                    }
                }
            }

            FieldInfo {
                column_name,
                is_primary_key,
                rust_type: field.ty.to_token_stream().to_string(),
            }
        })
        .collect()
}