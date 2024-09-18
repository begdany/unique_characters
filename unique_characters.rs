use std::collections::HashSet; // Импортируем HashSet из стандартной библиотеки

// Функция проверки уникальности символов в строке
fn unique_characters(s: &str) -> bool {
    let mut chars_set = HashSet::new(); // Создаем множество для хранения уникальных символов
    
    // Приводим строку к нижнему регистру и проходим по каждому символу
    for c in s.to_lowercase().chars() {
        if !chars_set.insert(c) {
            return false; // Если символ уже есть в множестве, то возвращаем отрицательный ответ
        }
    }
    
    true // Если все символы уникальные, возвращаем положительный ответ
}

fn main() {
    let test_str1 = "Outleap";
    let test_str2 = "Salsa";
    
    println!("Строка \"{}\" содержит уникальные символы: {}", test_str1, unique_characters(test_str1));
    println!("Строка \"{}\" содержит уникальные символы: {}", test_str2, unique_characters(test_str2));
}
