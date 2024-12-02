mod lib_module {
    include!("../modules/lib.rs");
}

fn main() {
    lib_module::say_hello();
    lib_module::greet();
    let data = lib_module::get_data();
    println!("{}", data);
}
