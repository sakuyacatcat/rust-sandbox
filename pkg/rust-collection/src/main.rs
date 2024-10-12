use std::collections::HashMap;

fn main() {
    // Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    // Iterating over the values in a vector
    let v = vec![1,2,3,4,5];
    let does_not_exist = v.get(100);

    // vector iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // vector and enums
    #[derive(Debug)]
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
    println!("{:?}", row);

    // string and &str
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let mut s = String::from("lo");
    s.push_str("l");
    println!("{}", s);

    // string and ownership
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    let s4 = format!("{}-{}", s3, s2);
    println!("{}", s4);

    // hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", teams);
    println!("{:?}", initial_scores);
    println!("{:?}", scores);

    // ownership of hash map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());
    println!("{:?}", map);
    println!("{}", field_name);
    println!("{}", field_value);
}
