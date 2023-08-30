fn main() {
    let mut s = String::from("poupa loupa");


    let i = first_word(&s);

    println!("{i}");
    s.clear();

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn test() -> (String, usize) {
    let s = String::from("poupa");

    (s, s.len())
}