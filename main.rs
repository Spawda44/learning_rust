use std::io::{self, Write};

fn input(msg:&str) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("Не удалось.");

    let mut input_value:String = String::new();
    io::stdin().read_line(&mut input_value).expect("Не удалось.");

    return input_value.trim().to_string();
}

fn main() {
    let name:String = input("Как вас зовут: ");

    println!("Здравствуйте {}", name);
}
