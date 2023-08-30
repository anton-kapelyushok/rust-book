#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 13,
                style: "sandal".to_string(),
            },
            Shoe {
                size: 10,
                style: "boot".to_string(),
            },
        ];

        let in_my_size = shoes_in_size(&shoes, 10);

        let styles: Vec<&String> = in_my_size.iter().map(|x| &x.style).collect();

        assert_eq!(
            styles,
            vec!["sneaker", "boot"],
        );
    }

    #[test]
    fn map_iterator() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        assert_eq!(v1_iter.next(), None);

        let mut v1 = vec![1, 2, 3];
        let mut v1_iter_mut = v1.iter_mut();

        let first = v1_iter_mut.next().unwrap();
        assert_eq!(_type_of(&first), "&mut i32");

        *first += 1;

        assert_eq!(v1, vec![2, 2, 3]);
    }

    #[test]
    fn mutable_references() {
        let a: &mut u32 = &mut 4;

        *a += 1;

        let s = "poupa".to_string();
        assert_eq!(_type_of(&s), "alloc::string::String");
        _consumes(s);
    }
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn _type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}


fn _consumes(s: String) {
    let mut s = s;
    s.push_str("poupa")
}
