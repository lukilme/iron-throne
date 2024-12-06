use validation_macro::Validate;

#[derive(Debug, Validate)]
struct Product {
    #[not_null]
    id: Option<i32>,
    #[max_size = 120]
    name: String,
}

fn main() {
    let product = Product {
        id: Some(11),
        name: "A product name that is way too long and exceeds the limit".to_string(),
    };

    match product.validate() {
        Ok(_) => println!("Product is valid"),
        Err(err) => println!("Validation error: {}", err),
    }
}
