mod modules {
    pub mod database; // Agora o Rust sabe onde procurar o módulo `database`
}
mod subpart {
    pub mod reflection; // Agora o Rust sabe onde procurar o módulo `database`
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use modules::database;

    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 11);
    }

    #[test]
    fn database_tests() {
        database::database_tests::test_establish_connection();
    }
}

fn main() {
    println!("testing...");
}
