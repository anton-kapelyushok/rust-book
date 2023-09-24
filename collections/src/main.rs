fn main() {
    if true { _hash_map() } 
    if false { _string() }
    if false { _vector() }
}

fn _hash_map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score is {score}");


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    let blue = map.get(&String::from("Favorite color")).unwrap();
    println!("Favorite color is {blue}");
    map.remove(&String::from("Favorite color"));

    if let Some(count) = scores.get_mut(&String::from("Blue")) {
        println!("{count}");
        *count += 1;
        println!("{count}");

    }

}

fn _string() {
    let _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let hello_rus = String::from("Здравствуйте");
    println!("{hello_rus}");

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // println!("{s1}"); // moved
    println!("{s2}");
    println!("{s3}");


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");

    let slice = &hello_rus[0..2];
    println!("{slice}");

    let b = &hello_rus.as_bytes()[0];
    println!("{b}");
}

fn _vector() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v);   

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(5);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];
    let third: i32 = v[2]; // is it move?
    println!("The third element is {third}");

    let third: &mut i32 = &mut v[2];
    println!("The third element is {third}");
    *third = 8;
    println!("The third element is {third} now");

    let third: Option<&i32> = v.get(1000);
    if let Some(third) = third {
        println!("{third}");
    } else {
        println!("None");
    }

    let third: Option<&i32> = v.get(2);
    if let Some(third) = third {
        println!("{third}");
    } else {
        println!("None");
    }

    let v = vec![100, 32, 57];
    for i in v {
        println!("{i}");
    }


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    #[ derive(Debug) ]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for e in &row {
        println!("{:?}", e);
        {
            use SpreadsheetCell::*;
            match &e {
                Int(v) => println!("{v}"),
                Text(s) => println!("{s}"),
                Float(f) => println!("{f}"),
            }
        }
    }

}
