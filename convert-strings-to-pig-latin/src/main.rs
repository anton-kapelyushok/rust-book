#[derive(Debug)]
struct Poupa {
    str: String,
    arc: Arc<str>,
}

use std::sync::Arc;

fn main() { 



    let poupa = Poupa { str: "poupa".to_string(), arc: "poupa".into() };

    print_type_of(&poupa.str);
    print_type_of(&poupa.arc);

    println!("{:?}", convert_string_to_pig_latin(""));
    println!("{:?}", convert_string_to_pig_latin("привет"));
    println!("{:?}", convert_string_to_pig_latin("he1llo"));
    println!("{:?}", convert_string_to_pig_latin("hello"));
    println!("{:?}", convert_string_to_pig_latin("apple"));
}

#[ derive(Debug) ]
enum ConvertError {
    StringIsNotLatin,
}
fn convert_string_to_pig_latin(string: &str) -> Result<String, ConvertError> {
    if string.is_empty() {
        return Ok(string.to_string())
    }
    if !string.chars().all(char::is_alphabetic) {
        return Err(ConvertError::StringIsNotLatin);
    }
    let first_char = string.chars().next().unwrap();
    let vowels = "aeiou";

    return Ok(
        if vowels.contains(first_char) {
            format!("{string}-hay")
        } else {
            let from_second = &string[first_char.len_utf8()..];
            format!("{}-{}ay", &from_second, &first_char)
        }
    );
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


