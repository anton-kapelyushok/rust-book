#[derive(Debug)]
struct Ipv4Addr(u8, u8, u8, u8);

#[derive(Debug)]
struct Ipv6Addr;

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    test();

    let ipv4addr = IpAddr::V4(Ipv4Addr(127, 0, 0, 1));
    dbg!(&ipv4addr);

    Message::Quit.print();  
    (Message::Move { x: 4, y: 2 }).print();
    (Message::Write(String::from("poupa"))).print();
    (Message::ChangeColor(255, 255, 0)).print();


    let some_number = Some(5);
    let some_char = Some('e');

    dbg!(sum_options(&Some(5), &Some(4)));
    dbg!(sum_options(&Some(5), &None));
    dbg!(sum_options(&None, &Some(4)));
    dbg!(sum_options(&None, &None));

    dbg!(mul_options(&Some(5), &Some(4)));
    dbg!(mul_options(&None, &Some(4)));
}

fn sum_options(x: &Option<i32>, y: &Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,        
    }
}

fn mul_options(x: &Option<i32>, y: &Option<i32>) -> Option<i32> {
    if let (Some(x), Some(y)) = (x, y) {
        Some(x * y)
    } else {
        None
    }
}

fn handle_message(msg: &Message) {
    match msg {
        _ => todo!(),
    };
}

impl Message {
    fn print(&self) {
        use Message::*;
        use Message::Quit as PoupaQuit;
        use Message::Move as PoupaMove;
        match self {
            PoupaQuit => println!("Quit message"),
            Move { x, y } => println!("Move {x} {y}"),
            Message::Write(s) => println!("Write {s}"),
            Message::ChangeColor(r, g, b) => println!("ChangeColor {r} {g} {b}"),
        }
    }

    // fn print1(&self) {
    //     match self {
    //         PoupaQuit => println!("Quit message"),
    //         Move { x, y } => println!("Move {x} {y}"),
    //         Message::Write(s) => println!("Write {s}"),
    //         Message::ChangeColor(r, g, b) => println!("ChangeColor {r} {g} {b}"),
    //     }
    // }
}



fn test() {
    #[ derive(Debug) ]
    struct Ip4Addr(u8, u8);
    
    #[ derive(Debug) ]
    struct Ip6Addr(u8, u8, u8);

    #[ derive(Debug) ]
    enum Ip {
        V4(Ip4Addr),
        V6(Ip6Addr),
    }
    println!("{:?}", Ip::V4(Ip4Addr(192, 168)));
}
