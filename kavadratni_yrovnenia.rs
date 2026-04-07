use std::io::{self, Write};

fn input(msg:&str) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("Не удалось.");

    let mut input_value:String = String::new();
    io::stdin().read_line(&mut input_value).expect("Не удалось.");

    return input_value.trim().to_string();
}

fn main() {
    /* 
    aX^2 + bX + c = 0
    D = b^2 - 4(a * c)
    
    НЕТ КОРНЕЙ: a = 1, b = 1, c = 1
    ОДИН КОРЕНЬ: a = 1, b = 12, c = 36
    ДВА КОРНЯ: a = 1, b = -2, c = -3
    */

    println!("Решить квадратное уравнение.\n");

    let a:f64 = input("Введите a >> ").parse().unwrap();
    let b:f64 = input("Введите b >> ").parse().unwrap();
    let c:f64 = input("Введите c >> ").parse().unwrap();

    let d:f64 = (b * b) - 4.0 * (a * c);

    if d > 0.0 {
        let x1:f64 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2:f64 = ((-b) - d.sqrt()) / (2.0 * a);

        println!(
            "\nРЕШЕНО У УРАВНЕНИЯ 2 КОРНЯ\nD = {}\n1 корень = {}\n2 корень = {}",
            d, x1, x2
        );
    } else if d < 0.0 {
        println!("\nУ УРАВНЕНИЯ НЕТ КОРНЕЙ.");
    } else {
        let x:f64 = (-b) / (2.0 * a);

        println!(
            "\nРЕШЕНО У УРАВНЕНИЯ 1 КОРЕНЬ\nD = {}\nКорень = {}",
            d, x
        );
    }

    input("\nНажми на любую клавишу, что бы выйти... ");
}


