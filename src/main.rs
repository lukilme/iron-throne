use validator_derive::Validate; 
use validator::Validate;

#[derive(Debug, Validate)]
struct User {
    #[validate(length(min = 3, max = 50))]
    username: String,
}

fn main() {
    let user = User {
        username: "abkkk".to_string(), 
    };

    match user.validate() {
        Ok(_) => println!("Usuário válido!"),
        Err(e) => println!("Erro de validação: {:?}", e),
    }
}