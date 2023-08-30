fn main() {
    exhaustive_match(&Point { x: 4, y: 5 } );


    let point = Point { x: 4, y: 2 };
    match point {
        Point { x, .. } if x % 2 == 0 => { 
            println!("leg 1"); 
            print_type_of(&point);
        },
        Point { x, .. } if x % 2 != 0 => { println!("leg 2"); },
        Point => panic!("Never happens"),
    }

    println!("{:?}", point.x);


    let v = vec![1, 2, 3];
    for i in &v {
        print_type_of(&i);
    }
    // sink(&v[0]);

    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result = longest(&string1, string2);
    println!("result 1 is {result}");



    let novel = "Call me Ishmael. Some years ago...";
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i1 = ImportantExcerpt { part: first_sentence };

    let i2 = produces_citation(first_sentence);

    let s1 = S1 { data: "poupa".into() };
    let s2 = S2 { data1: 4, data2: 5 };

    s1.mark1();
    s2.mark1();

    match left_or_right(true, &s1, &s2) {
        Left(ss1) => {
            println!("SS1: {}", ss1.data);
            ss1.mark1();
        },
        Right(ss2) => {
            println!("SS2: {}, {}", ss2.data1, ss2.data2);
            ss2.mark1();
        },
    }


    match left_or_right(false, &s1, &s2) {
        Left(ss1) => {
            println!("{}", ss1.data);
            ss1.mark1();
        },
        Right(S2 { data1, data2: 5 }) => {
            println!("Right is 5!");
        }
        Right(ss2) => {
            println!("{}, {}", ss2.data1, ss2.data2);
            ss2.mark1();
        },
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = "poupa".to_string();
    result.as_str(); // does not work

    "loupa" // works because 'static is > 'a
}

struct ImportantExcerpt<'a> {
    part: &'a str, // & means borrowed, borrowed requires lifetime
}

use std::sync::Arc;
struct ImportantExcerpt2 {
    part: Arc<str>,
}


fn produces_citation<'a>(s: &'a str) -> ImportantExcerpt<'a> {
    let first_sentence = s.split('.').next().expect("Could not find a '.'");
    ImportantExcerpt { part: first_sentence }
}


fn produces_citation1(s: &str) -> ImportantExcerpt2 {
    let first_sentence = s.split('.').next().expect("Could not find a '.'");
    ImportantExcerpt2 { part: first_sentence.into() } // copying here :(
}


fn mut_string_and_arc() {
    let a: String = String::from("poupa");
    let b: Arc<str> = a.into(); 

    // println!("{a}");   // a moved, not borrowed
    println!("{b}");

    let a: &str = "poupa";
    let b: Arc<str> = a.into();

    println!("{a}");
    println!("{b}");


    let a: &str = "poupa";
    let b: Arc<str> = a.into();

    println!("{a}");
    println!("{b}");
}

struct SomeStruct {
    data: String,
}


fn from_borrowed_struct<'a, 'b> (s1: &'a SomeStruct, s2: &'b SomeStruct) -> (&'a String, &'b String) {
    (&s1.data, &s2.data)
}

trait Mark1 {
    fn mark1(&self);
}

trait Mark2 {
    fn mark2(&self);
}

fn longest_sized<'a, T: Mark1>(x: &'a T, y: &'a T) -> &'a T 
where T: Mark2
{
    x.mark1();
    x.mark2();
    x
}

enum Either<L, R> {
    Left(L), Right(R)
}
use Either::*;
fn left_or_right<'a, T1: Mark1, T2: Mark1>(switch: bool, l: &'a T1, r: &'a T2) -> Either<&'a T1, &'a T2> {
    if switch { Left(l) } else { Right(r) }
}

struct S1 {
    data: Arc<str>,
}

struct S2<T> {
    data1: T,
    data2: T,
}


impl Mark1 for S1 {
    fn mark1(&self) {
        println!("S1: {}", self.data);
    }
}

impl Mark1 for S2<i32> {
    fn mark1(&self) {
        println!("S2: {}", self.data1 + self.data2);
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn sink(i: &i32) {

}

fn exhaustive_match(point: &Point) {
    print_type_of(&point);
    match point {
        Point { x, .. } if x % 2 == 0 => { 
            println!("leg 1"); 
            print_type_of(&point);
        },
        Point { x, .. } if x % 2 != 0 => { println!("leg 2"); },
        Point => panic!("Never happens"),
    }
}

pub fn test(num: &i32) {
    sink1(num + num);
}

fn sink1(num: i32) {}
