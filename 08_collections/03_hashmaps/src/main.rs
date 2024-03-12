use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // NOTE: owned values will be inserted in the hashmap by move and hashmap will own them
    // If Copy it's  implemented the values are coped like i32
    //
    // Also you cna insert references but the reference has to stay valid for the lifetime of the
    // hashmap!

    let team_name = String::from("Blue");
    let score = scores.get("Blue").copied().unwrap_or(0);
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Insert only if not present already
    scores.entry(String::from("Yellow")).or_insert(999);
    scores.entry(String::from("Pink")).or_insert(999);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
