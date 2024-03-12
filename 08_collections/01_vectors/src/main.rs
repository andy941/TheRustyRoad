fn main() {
    let mut v: Vec<i32> = Vec::new();
    /* // Macro to auto infer type
    let v = vec![1,2,3]; */
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
