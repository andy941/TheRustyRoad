use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;

fn main() {
    // Mode and median
    let _v = [1, 8, 5, 3, 5, 6, 7, 8];
    let _median = median(&_v);
    let _mode = mode(&_v);
    println!("Median  = {_median}");
    println!("Mode    = {_mode:?}");

    let _v = [1, 8, 5, 5, 6, 7, 8];
    let _median = median(&_v);
    let _mode = mode(&_v);
    println!("Median  = {_median}");
    println!("Mode    = {_mode:?}");

    // Pig latin
    let phrase = "ASCII strings and こんにちは";
    println!("Normal -> `{phrase}'");
    let phrase_latin = convert_to_pig_latin(phrase);
    println!("Latin  -> `{phrase_latin}'");

    // Employee database
    let mut data: EmployeeDatabase = Default::default();
    let mut line = String::new();
    println!("INPUT: <NAME> <DEPARTMENT>, LIST: ls, LIST BY DEPARTMENT: ls <DEPARTMENT>");
    loop {
        let _ = std::io::stdout().flush();
        line.clear();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => {
                println!("ERROR: The input string is not UTF-8");
                continue;
            }
        }
        let mut iter = line.split_whitespace();
        let first_word = iter.next().unwrap_or_default();
        if first_word == "q" {
            break;
        }
        if first_word.is_empty() {
            continue;
        }

        let second_word = iter.next().unwrap_or_default();
        if first_word == "ls" {
            match second_word.is_empty() {
                true => data.list_all(),
                false => data.list_department(second_word),
            }
        }

        let third_word = iter.next().unwrap_or_default();
        let fourth_word = iter.next().unwrap_or_default();
        if first_word.to_ascii_lowercase() != "add" || third_word != "to" {
            continue;
        }
        if !fourth_word.is_empty() {
            data.add(second_word, fourth_word);
        }
    }
}

// Employee database ----------------------------------------------------------
#[derive(Default, Debug)]
struct EmployeeDatabase {
    database: HashMap<String, Vec<String>>,
}

impl EmployeeDatabase {
    fn add(&mut self, emp: &str, dep: &str) {
        match self.database.entry(String::from(dep)) {
            Entry::Occupied(o) => {
                let o = o.into_mut();
                o.sort();
                match o.binary_search(&String::from(emp)) {
                    Ok(_) => (),
                    Err(_) => o.push(String::from(emp)),
                }
            }
            Entry::Vacant(v) => {
                v.insert(vec![String::from(emp)]);
            }
        }
    }
    fn list_department(&mut self, dep: &str) {
        match self.database.entry(String::from(dep)) {
            Entry::Occupied(o) => {
                println!("{:#?}", o.into_mut())
            }
            Entry::Vacant(_) => println!("ERROR: {dep} is not a department or the department is empty"),
        }
    }
    fn list_all(&self) {
        println!("{:#?}", self.database);
    }
}

// Pig latin ------------------------------------------------------------------

fn convert_to_pig_latin(word: &str) -> String {
    let mut result = String::new();
    let vowels = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
    for x in word.split_whitespace() {
        if !x.is_ascii() {
            result.push_str(&format!("{x} "));
            continue;
        }
        match vowels.iter().any(|&v| x.starts_with(v)) {
            true => result.push_str(&format!("{}-hay ", x)),
            false => {
                let mut iter = x.chars();
                let c = iter.next().unwrap_or(' ');
                let s: String = iter.collect();
                result.push_str(&format!("{s}-{c}ay "));
            }
        }
    }
    result.trim_end().to_string()
}

// Mode and median ------------------------------------------------------------

fn mode(v: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut max_count: i32 = 0;
    for x in v {
        let count = map.entry(x).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
        }
    }
    let mut result: Vec<i32> = Vec::new();
    for (x, y) in map {
        if y == max_count {
            result.push(*x);
        }
    }
    result.sort();
    result
}

fn median(v: &[i32]) -> f32 {
    let mut v2 = v.to_owned();
    v2.sort();
    println!("{v2:?}");
    let remainder = v2.len() % 2;
    let middlepoint = v2.len() / 2;
    match remainder {
        0 => {
            let first = v2[middlepoint - 1];
            let second = v2[middlepoint];
            let delta = (second - first) as f32 / 2.0;
            let result = delta + first as f32;
            result
        }
        _ => v2[middlepoint] as f32,
    }
}
