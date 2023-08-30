use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![ 34, 50, 25, 100, 65 ];

    let result = largest(&number_list);
    println!("The largest number is {}", result);


    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("points: {:?}, {:?}", integer_point, float_point);

    // println!("{}", integer_point.distance_from_origin());
    println!("{}", float_point.distance_from_origin());
}


impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}