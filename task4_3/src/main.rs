
use std::io;
use std::collections::HashMap;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        println!("Введите команду (или 'exit' для выхода):");
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let words: Vec<&str> = input.split_whitespace().collect();

        if words.len() < 4 {
            println!("Ошибка: недостаточно аргументов. Введите команду в формате: 'добавить имя в отдел'");
            continue;
        }

        let name = words[1];
        let department = words[3];

        departments.entry(department.to_string()).or_insert_with(Vec::new).push(name.to_string());

    }
    // println!("Словарь: {:?}", departments);
    let mut sorted_departments: Vec<_> = departments.iter().collect();
        sorted_departments.sort_by(|a, b| a.0.cmp(b.0));

    println!("Отсортированный список отделов:");
    for (dept, members) in sorted_departments {
        println!("{}: {:?}", dept, members);
    }
}

