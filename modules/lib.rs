mod utils;
mod data;

pub use utils::greet;
pub use data::get_data;

pub fn say_hello() {
    println!("Hello from modules::lib!");
}