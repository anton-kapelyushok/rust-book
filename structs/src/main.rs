struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect3 = Rectangle::square(20);
    dbg!(&rect3);

    println!("rect1 is {:?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.username.push_str(", user1");
    print_user(&user1);

    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);

    let user2 = build_user(String::from("poupa@gmail.com"), String::from("poupa"));
    print_user(&user2);

    let user3 = User {
        email: String::from("loupa@gmail.com"),
        ..user2
    };
    print_user(&user3);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let mut black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let white = Color(i32::MAX, i32::MAX, i32::MAX);
    black = white;

    let mut tuple1 = (0, "str1");
    let tuple2 = (1, "str2");

    tuple1 = tuple2;

    let mut tuple3 = ("str3", 2);

    struct AlwaysEqual;
    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}",
            user.active, user.username, user.email, user.sign_in_count)
}
