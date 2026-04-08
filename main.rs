/*
Импорты:
- Ввод/Вывод, Обработка строк
- Словари с сортировкой по порядку
- Неявная аннотация типов
*/

use std::io::{self, Write};
use std::collections::BTreeMap;
use std::fmt::Display;


fn input(msg: &str) -> String {
    /*
    Функция ввода от пользователя:
    - Возможность ввести print перед запросом
    - Ввод пользователя
    - Обработка ввода
    */

    print!("{}", msg);
    io::stdout().flush().expect("Не удалось.");

    let mut input_val:String = String::new();
    io::stdin().read_line(&mut input_val).expect("Не удалось.");

    return input_val.trim().to_string();
}


fn output_btreemap<K, V>(btreemap_type: i8, btreemap: &BTreeMap<K, V>)
where
    K: Display,
    V: Display
{
    /*
    Вывод словаря в консоль:
    - Неявная аннотация типов для словарей с разными типами в ключах
    - Разные типы вывода
    */

    match btreemap_type {
        1 => {
            for (k, v) in btreemap {
                println!("{}: {}", k, v);
            }
        }
        2 => {
            for (k, v) in btreemap {
                println!("[{}] - {}", k, v);
            }
        }
        _ => {println!("Некорректный тип BTreeMap.");}
    }
}


fn input_text(text_type: &str) -> String {
    /*
    Запрос однострочных простых данных у пользователя:
    - "t": Запрос заголовка заметки
    - "c": Запрос наполнения заметки
    - Обработка на правильность аргумента в функцию
    - Обработка на пустую строку
    */
    
    match text_type {
        "t" => {
            loop {
                let input_value:String = input("Введите название заметки: ").trim().to_string();
                if input_value.is_empty() {
                    println!("Строка не может быть пустой.")
                } else {return input_value;}
            }
        }
        "c" => {
            loop {
                let input_value:String = input("Введите текст заметки: ").trim().to_string();
                if input_value.is_empty() {
                    println!("Строка не может быть пустой.")
                } else {return input_value;}
            }
        }
        _ => {
            println!("Ошибка.");
            return String::new();
        }
    } 
}


fn input_status() -> String {
    /*
    Запрос статуса заметки у пользователя:
    - Таблица статусов, и вывод в консоль
    - Проверка ввода на пустую строку и неверный пункт
    */

    let choices: BTreeMap<i8, String> = BTreeMap::from([
        (1, String::from("Выполнено")),
        (2, String::from("В процессе")),
        (3, String::from("Отложено")),
    ]);

    loop {
        println!("=====================");
        output_btreemap(2, &choices);
        println!("=====================");
        
        let choice: String = input("Выберите статус: ");
        match choice.trim().parse::<i8>() {
            Ok(choice) => {
                match choices.get(&choice) {
                    Some(status) => return status.clone(),
                    None => {println!("Некорректный выбор.\n");}
                }
            }
            Err(e) => {
                println!("Ошибка \"{}\". Ожидается номер статуса.", e);
            }
        }
    }
}


fn create_note() -> BTreeMap<String, String> {
    /*
    Сборка заметки:
    title: Название заметки
    content: Текст заметки
    status: Статус заметки
    */

    let mut note: BTreeMap<String, String> = BTreeMap::new();

    note.insert("Название".to_string(), input_text("t"));
    note.insert("Текст".to_string(), input_text("c"));
    note.insert("Статус".to_string(), input_status());

    return note;
}


fn delete_notes(notes: &mut Vec<BTreeMap<String, String>>) {
    /*
    Удаление заметки по индексу:
    - Запрос номера заметки для удаления
    - Проверка на корректность номера и пустоту строки
    */

    loop {
        let choice: String = input(&format!("Какую заметку хотите удалить? (1-{}) >> ", notes.len()));

        match choice.trim().parse::<usize>() {
            Ok(index) => {
                if index > 0 && index <= notes.len() {
                    notes.remove(index - 1);
                    println!("Заметка #{} была удалена.", index);
                    break;
                } else {
                    println!("Некорректный номер. Введите от 1 до {}", notes.len());
                }
            }
            Err(e) => {
                println!("Ошибка \"{}\". Ожидался номер заметки.", e);
            }
        }
    }
}


fn output_notes(notes: &Vec<BTreeMap<String, String>>) {
    for (i, note) in notes.iter().enumerate() {
        println!("=====================");
        println!("Заметка #{}", i + 1);
        println!("=====================");
        output_btreemap(1, &note);
        println!("=====================\n");
    }
}


fn main() {
    let mut notes: Vec<BTreeMap<String, String>> = Vec::new(); 

    let main_choices: BTreeMap<i8, String> = BTreeMap::from([
        (1, String::from("Создать заметку")),
        (2, String::from("Посмотреть заметки")),
        (3, String::from("Удалить заметку")),
        (4, String::from("Сохранить и выйти")),
    ]);

    loop {
        println!("=====================");
        output_btreemap(2, &main_choices);
        println!("=====================");

        let choice: String = input("Выберите пункт: ");
        match choice.trim().parse::<i8>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        notes.push(create_note());
                    },
                    2 => {
                        output_notes(&notes);
                    },
                    3 => {
                        delete_notes(&mut notes);
                    },
                    4 => {
                        break;
                    },
                    _ => {
                        println!("Некорректный выбор.")
                    },
                }
            }
            Err(e) => {
                println!("Ошибка \"{}\". Ожидается номер пункта.", e);
            }
        }
    }
}