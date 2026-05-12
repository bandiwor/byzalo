use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    println!("Морозов К.О. 090301-ПОВа-о25");
    let file_path = "./war_and_peace.txt";

    println!("Читаем файл...");
    let text = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error.Reading: {}: {}", file_path, e);
            return;
        }
    };

    print!("Индексация...");
    let start_index = Instant::now();

    let freq_map = text
        .par_split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(
            || HashMap::new(),
            |mut map, word| {
                *map.entry(word).or_insert(0) += 1;
                map
            },
        )
        .reduce(
            || HashMap::new(),
            |mut map1, map2| {
                for (k, v) in map2 {
                    *map1.entry(k).or_insert(0) += v;
                }
                map1
            },
        );

    let mut sorted_words: Vec<(String, usize)> = freq_map.into_iter().collect();

    sorted_words.par_sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let index_time = start_index.elapsed();
    println!(" завершена за {:?}", index_time);
    println!("Уникальных слов: {}", sorted_words.len());
    println!("--------------------------------------------------");

    loop {
        print!("\nВведите слово для поиска (от 3 символов, или 'exit' для выхода): ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let query = query.trim().to_lowercase();

        if query == "exit" {
            println!("выход...");
            break;
        }

        if query.chars().count() < 3 {
            println!("[X] Запрос должен содержать не менее 3 символов.");
            continue;
        }

        let start_query = Instant::now();

        let results: Vec<&(String, usize)> = sorted_words
            .iter()
            .filter(|(word, _)| word.contains(&query))
            .take(20)
            .collect();

        let query_time = start_query.elapsed();

        println!("Время поиска: {:?}", query_time);

        if results.is_empty() {
            println!("Ничего не найдено.");
        } else {
            println!("Топ совпадений:");
            for (i, (word, count)) in results.iter().enumerate() {
                println!("{:2}. {} ({} раз)", i + 1, word, count);
            }
        }
    }
}
