#[test]
fn pointers() {
    let (mut i, mut j) = (42, 2701);

    // Can only have one mutable reference at a time, so need p to Drop before passing i to
    // println!()
    {
        let p = &mut i;
        println!("{}", *p);
        *p = 21;
    }
    println!("{}", i);

    {
        let p = &mut j;
        *p = *p / 37;
    }
    println!("{}", j);
}

#[cfg(test)]
mod structs {
    // println!() doesn't know how to display new structs unless Display has been implemented.
    // However, Debug can be automatically implemented, which is simpler to use.
    #[derive(Debug)]
    struct Vertex {
        x: i32,
        y: i32,
    }

    #[test]
    fn structs() {
        // {:?} means use the Debug trait
        println!("{:?}", Vertex{x: 1, y: 2});
    }

    #[test]
    fn struct_fields() {
        let mut v = Vertex{x: 1, y: 2};
        v.x = 4;
        println!("{:?}", v.x);
    }

    #[test]
    fn struct_pointers() {
        let mut v = Vertex{x: 1, y: 2};
        {
            let p = &mut v;
            // Rust "references" are more like C++ references than true pointers.
            p.x = 1e9 as i32;
        }
        println!("{:?}", v);
    }

    #[test]
    fn struct_literals() {
        // Can't omit field names, but the order can be changed
        let v1 = Vertex{y: 2, x: 1};
        // I think this is basically the same thing..
        let v2 = &mut Vertex{y: 2, x: 1};
        println!("{:?} {:?}", v1, v2);
    }
}

#[test]
fn arrays() {
    let a = vec!["Hello", "World"];
    println!("{} {}", a[0], a[1]);
    // Vectors don't implement Display, but do implement Debug
    println!("{:?}", a);

    let primes = vec![2, 3, 5, 7, 11, 13];
    println!("{:?}", primes);
}

#[test]
fn slices() {
    let primes = vec![2, 3, 5, 7, 11, 13];
    let s = &primes[1..4];
    println!("{:?}", s);
}

#[test]
fn slices_pointers() {
    let mut names = vec!["John", "Paul", "George", "Ringo"];
    println!("{:?}", names);

    // Can borrow immutably multiple times
    {
        let a = &names[0..2];
        let b = &names[1..3];
        println!("{:?} {:?}", a, b);
    }

    // Can only borrow mutably once and no other borrows may be in scope
    {
        let b = &mut names[1..3];
        b[0] = "XXX";
    }

    println!("{:?}", names);
}
