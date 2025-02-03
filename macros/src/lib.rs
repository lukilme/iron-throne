use inflector::Inflector;
use syn::{parse_macro_input, DeriveInput, Attribute, Meta, Lit, Expr, ExprLit};
use quote::quote;
use proc_macro::TokenStream; // Correct proc_macro type
//use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(ActiveRecord, attributes(table_name, primary_key))]
pub fn derive_active_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let table_name = get_table_name(&input.attrs, &struct_name);

    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) = &input.data {
        &fields.named
    } else {
        panic!("Apenas structs com campos nomeados são suportadas");
    };

    let (columns, placeholders) = prepare_sql_parts(fields);

    let expanded = quote! {
        impl iron_throne_v2::prelude::active_record::ActiveRecord for #struct_name {
            fn save(&self) -> Result<(), iron_throne_v2::erro::DbError> {
                let query = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    #table_name,
                    #columns,
                    #placeholders
                );
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let mut db = iron_throne_v2::database::SINGLETON_INSTANCE.lock().await;
                    db.execute_query(query.as_str()).await.map(|_| ())
                })
            }
        }
    };
    expanded.into()
}

fn get_table_name(attrs: &[Attribute], struct_name: &syn::Ident) -> String {
    for attr in attrs {
        if attr.path().is_ident("table_name") {
            if let Ok(Meta::NameValue(nv)) = attr.parse_args::<Meta>() {
                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                    return s.value();
                }
            }
            panic!("table_name attribute must be a string literal");
        }
    }
    struct_name.to_string().to_snake_case()
}

fn prepare_sql_parts(fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>) -> (String, String) {
    let columns: Vec<String> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    let placeholders: Vec<String> = (1..=columns.len())
        .map(|i| format!("${}", i))
        .collect();

    (columns.join(", "), placeholders.join(", "))
}