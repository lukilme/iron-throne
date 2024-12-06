use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, Fields, Lit, Meta};


#[proc_macro_derive(Accessors)]
pub fn accessors_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;


    let accessors = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(ref fields_named) = data_struct.fields {
            let fields = &fields_named.named; 
            fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_ty = &field.ty;
        
                quote! {
                    pub fn #field_name(&self) -> &#field_ty {
                        &self.#field_name
                    }
        
                    pub fn set_ #field_name(&mut self, value: #field_ty) -> Result<(), String> {
                        self.#field_name = value;
                        self.validate().map_err(|e| e.to_string())
                    }
                }
            }).collect::<Vec<_>>()
        } else {
            unimplemented!("Somente structs com campos nomeados são suportadas.")
        }
    } else {
        unimplemented!("Somente structs são suportadas.")
    };

    let expanded = quote! {
        impl #struct_name {
            #(#accessors)*
        }
    };

    TokenStream::from(expanded)
}



#[proc_macro_derive(Validate, attributes(not_null, max_size, min_size))]
pub fn validate_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            fields.named.iter()
        } else {
            unimplemented!("Somente structs nomeadas são suportadas")
        }
    } else {
        unimplemented!("Somente structs são suportadas")
    };

    let validations = fields.map(|field| {
        let field_name = &field.ident;
        let mut validation_code = quote! {};

        for attr in &field.attrs {
            if let Some(meta) = get_meta(attr) {
                match meta {
                    Meta::Path(path) if path.is_ident("not_null") => {
                        validation_code = quote! {
                            if self.#field_name.is_none() {
                                return Err(format!("Field '{}' cannot be null", stringify!(#field_name)));
                            }
                        };
                    }
                    
                    Meta::NameValue(name_value) if name_value.path.is_ident("max_size") => {
                        if let Expr::Lit(expr_lit) = &name_value.value {
                            if let Lit::Int(size) = &expr_lit.lit {
                                let max_size: usize = size.base10_parse().unwrap();
                                validation_code = quote! {
                                    if self.#field_name.len() > #max_size {
                                        return Err(format!(
                                            "Field '{}' exceeds maximum size of {}",
                                            stringify!(#field_name),
                                            #max_size
                                        ));
                                    }
                                };
                            }
                        }
                    }

                    Meta::NameValue(name_value) if name_value.path.is_ident("min_size") => {
                        if let Expr::Lit(expr_lit) = &name_value.value {
                            if let Lit::Int(size) = &expr_lit.lit {
                            let min_size: usize = size.base10_parse().unwrap();
                            validation_code = quote! {
                                if self.#field_name.len() < #min_size {
                                    return Err(format!(
                                        "Field '{}' is smaller than the minimum size of {}",
                                        stringify!(#field_name),
                                        #min_size
                                    ));
                                }
                            };
                        }
                    }
                }
                    _ => {}
                }
            }
        }
        validation_code
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> Result<(), String> {
                #(#validations)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}




fn get_meta(attr: &Attribute) -> Option<Meta> {
    Some(attr.meta.clone()) 
}