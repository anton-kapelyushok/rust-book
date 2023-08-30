use std::thread;
use std::time::Duration;


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red, 
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}", 
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}", 
        user_pref2, giveaway2
    );

    let mut _expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        println!("Finished calculating");
        num
    };

    // this does not compile:
    // _expensive_closure = cannot_be_assigned_to_closure;

    _print_type_of(&_expensive_closure);

    // println!("Result: {}", _expensive_closure(4));


    println!("Immutable borrow");
    let list = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    println!("");



    println!("Mutable borrow");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // nope, value is in closure
    // println!("{:?}", list); 
    borrows_mutably();

    println!("After calling closure: {:?}", list);
    println!("");


    println!("Move");
    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let _value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push("poupa");
        r.width
    });
    println!("{:#?}", list);


    // closure is struct + function around the data
    let mut o = 1;
    let switch = true;
    let mut my_closure: Box<dyn FnMut(i32) -> i32> = if switch {
        let c = |x: i32| -> i32 { o += x; o };
        Box::new(c)
    } else {
        let c = |x: i32| -> i32 { o += x; o += x; o };
        Box::new(c)
    };

    _print_type_of(&my_closure);

    my_closure(1);
    // my_closure(2);

    drop(my_closure);

    println!("{o}");
}

fn _accept_closure<F>(mut f: F)
where F: FnMut(i32) -> i32,
{
    f(4);
    f(4);
}


fn _cannot_be_assigned_to_closure(num: u32) -> u32 {
    num
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}











