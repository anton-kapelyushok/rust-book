mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[ignore]
    #[test]
    fn closure_must_be_mutable() {
        let mut o = 0;


        // 
        let mut closure: Box<dyn FnMut(i32) -> i32> = 
            Box::new(|x: i32| -> i32 { o += x; o });

        closure(1);
        closure(2);
        drop(closure);

        assert_eq!(o, 3);

        let closure: &mut dyn FnMut(i32) -> i32 = 
            &mut |x: i32| -> i32 { o += x; o };
        closure(1);
        closure(2);

        assert_eq!(o, 6);

        println!("{o}");

        let mut binding = |x: i32| -> i32 { o += x; o };

        let mut closure: Box<&mut dyn FnMut(i32) -> i32> = Box::new(&mut binding);
        _print_type_of(&closure);

        closure(1);
        closure(2);
        assert_eq!(o, 9);
    }

    #[ignore]
    #[test]
    fn again() {
        let mut o = 0;
        let mut closure = |x: i32| -> i32 { o += x; o };
        closure(1);
        closure(2);
        assert_eq!(o, 3);

        let mut o = 0;
        let mut closure = |x: i32| -> i32 { o += x; o };
        let closure_ref: &mut dyn FnMut(i32) -> i32 = &mut closure;
        closure_ref(1);
        closure_ref(2);
        assert_eq!(o, 3);
    }

    #[ignore]
    #[test]
    fn and_once_more() {
        let switch = false;
        let mut o = 0;
        let mut my_closure: Box<dyn FnMut(i32) -> i32> = if switch {
            let c = |x: i32| -> i32 { o += x; o };
            Box::new(c)
        } else {
            let c = |x: i32| -> i32 { o -= x; o };
            Box::new(c)
        };

        my_closure(1);
        my_closure(2);
    }

    #[ignore]
    #[test]
    fn integers_are_fine() {
        let i = &mut 4;
        *i += 1;
    }


    #[ignore]
    #[test]
    fn integer_box() {
        let mut i = Box::new(0);
        *i += 1;

        struct Poupa (i32, i32);
        let mut p = Box::new(Poupa( 14, 88 ));
        p.0 += 1;
        (*p).1 += 1;

        let p = &mut Poupa(14, 88);
        p.0 += 1;

        let mut binding = Poupa(100, 200);
        let p = Box::new(&mut binding);
        p.0 += 1;
        assert_eq!(p.0, 101);

        let mut o: i32 = 0;
        let mut c = |x: i32| -> i32 { o += x; o };
        c(0);

        let mut cp = Box::new(&mut c);
        cp(0);
    }


    #[test]
    fn wtf() {
        struct Poupa (i32, i32);
        let mut binding = Poupa(100, 200);
        let p = Box::new(&mut binding);
        p.0 += 1;
        assert_eq!(p.0, 101);

        _print_type_of(&p);

        {
            let mut o = 0;
            let mut c = |x: i32| -> i32 { o += x; o };
            c(0);
            let cp = Box::new(&mut c);
            (*cp)(0);
            // cp(0) - requires for cp to be mut, why?
            // because it calls (box.call_mut()) instead of box.deref().call_mut() 

            use std::ops::Deref;
            let mut df = cp.deref();
            // df(0);
            _print_type_of(&df);
            // cp(0);
            _print_type_of(&cp);
        }

        {

            let mut o = 0;
            let mut c = |x: i32| -> i32 { o += x; o };
            let cp = &mut c;
            (*cp)(0);
            cp(0);
        }

        {   
            let mut o = 0;
            let mut c = |x: i32| -> i32 { o += x; o };
            println!("jere!");
            _print_type_of(&c);
            let mut b = Box::new(c);
            // let mut b = Box::new(c);
            _print_type_of(&b);
            *b(0);
        }
        // pp(0);

        // cp(0);

        struct Foo<F>
        where
            F: FnMut(i32) -> i32,
        {
            pub foo: F,
        }

        impl <F> Foo<F>
        where
            F: FnMut(i32) -> i32,
        {
            fn new(foo: F) -> Self {
                Self { foo }
            }
        }

        let mut foo = Foo { foo: |a| a + 1 };
        (foo.foo)(42);

        let mut b = Box::new(foo.foo);
        _print_type_of(&b);
    
        (Foo::new(|a| a + 1).foo)(42);

    }

    #[test]
    fn wrapper() {
        let p = Poupa(14, 88);
        p.print();
        print(&p);

        let b = Box::new(p);
        b.print();

        print(&*b);
        print::<Poupa>(&b);
        print_poupa(&b);
    }


    fn print<T: MyPrint>(i: &T) {
        i.print();
    }

    fn print_poupa(i: &Poupa) {
        i.print()
    }
}

pub struct Poupa (i32, i32);

trait MyPrint {
    fn print(&self);
}

impl MyPrint for Poupa {
    fn print(&self) {
        println!("{} {}", self.0, self.1);
    }
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn _foo() -> i32 {
    let mut o: i32 = 0;
    let mut my_closure: Box<dyn FnMut(i32) -> i32> = {
    let c = |x: i32| -> i32 { o += x; o };
        Box::new(c)
    };

    my_closure(1);
    my_closure(2);

    drop(my_closure);
 
    return o;
}
