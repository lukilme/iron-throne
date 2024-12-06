#[cfg(test)]
pub mod database_tests {
    use std::collections::HashMap;
    use std::any::type_name;
    
    #[derive(Debug)]
    struct Product {
        id: i32,
        name: String,
    }
    
    macro_rules! to_hashmap {
        ($struct_name:ident { $($field:ident : $type:ty),* }) => {{
            let mut map = HashMap::new();
            $(
                map.insert(
                    stringify!($field),
                    type_name::<$type>(), 
                );
            )*
            map
        }};
    }
    
    #[test]
    fn main() {
        let product = Product {
            id: 123,
            name: String::from("Gadget"),
        };
    
        let field_types = to_hashmap!(Product { id: i32, name: String });
    
        for (field, field_type) in field_types {
            println!("Campo: {}, Tipo: {}", field, field_type);
        }
    }
    


}