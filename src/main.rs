use validation_macro::Validate;

#[derive(Debug, Validate)]
struct Product {
    #[not_null]
    id: Option<i32>,

    #[min_size = 5]
    #[max_size = 11]
    name: String,
}

impl Product {
    pub fn new(id: Option<i32>, name: String) -> Result<Self, String> {
        let product = Product { id, name };
        product.validate().map_err(|e| e.to_string())?;
        Ok(product)
    }
}

fn main() {
    let biscoit = Product::new(Some(11), "12345674289fjhgfdfgdfdf0".to_string());
    println!("{:?}", biscoit);
}
