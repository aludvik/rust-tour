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

#[test]
fn slice_literals() {
    let q: &[i32] = &[2, 3, 5, 7, 11, 13];
    println!("{:?}", q);

    let r: &[bool] = &[true, false, true, true, false, true];
    println!("{:?}", r);

    // Rust doesn't support anonymous structs, but tuples can be used instead
    let s: &[(i32, bool)] = &[
        (2, true),
        (3, false),
        (5, true),
        (7, true),
        (11, true),
        (13, true),
    ];
    println!("{:?}", s);
}

#[test]
fn slice_bounds() {
    // If we did `&[2, 3, ..., 13]` the compiler would complain because the size of the slice is
    // changing. For some reason doing this instead works...
    let mut s: &[i32] = &(vec![2, 3, 5, 7, 11, 13])[..];

    s = &s[1..4];
    println!("{:?}", s);

    s = &s[..2];
    println!("{:?}", s);

    s = &s[1..];
    println!("{:?}", s);
}

#[cfg(test)]
mod slices_len_cap {
    #[test]
    fn main() {
        // Go's slices are a bit more like Rust's Vec
        let v = vec![2, 3, 5, 7, 11, 13];

        // You can't shrink and extend a slice and have the data still be there in Rust because
        // that is dumb...
        print_vec(v[..0].to_vec());

        print_vec(v[..4].to_vec());

        print_vec(v[2..].to_vec());
    }

    fn print_vec(v: Vec<i32>) {
        println!("len={}, cap={}, {:?}", v.len(), v.capacity(), v);
    }
}

#[cfg(test)]
mod making_slices {
    #[test]
    fn making_slices() {
        let a = vec![0; 5];
        print_vec("a", &a);

        let mut b = Vec::<i32>::with_capacity(5);
        print_vec("b", &b);

        // Rust doesn't do initialization and bounds checking
        b.extend(&[0, 0, 0, 0, 0]);
        let c = b[..2].to_vec();
        print_vec("c", &c);

        let d = b[2..5].to_vec();
        print_vec("d", &d);
    }

    fn print_vec(s: &str, v: &Vec<i32>) {
        println!("{} len={} cap={} {:?}", s, v.len(), v.capacity(), v);
    }
}

#[test]
fn slices_of_slices() {
    let mut board = vec![
        ["_", "_", "_"],
        ["_", "_", "_"],
        ["_", "_", "_"],
    ];

    board[0][0] = "X";
    board[2][2] = "O";
    board[1][2] = "X";
    board[1][0] = "O";
    board[0][2] = "X";

    for row in board {
        println!("{}", row.join(" "))
    }
}

#[test]
fn append() {
    let mut s = Vec::<i32>::new();

    s.extend(&[0]);

    s.extend(&[1]);

    s.extend(&[2, 3, 4]);
}

#[test]
fn range() {
    let pow = vec![1, 2, 4, 8, 16, 32, 64, 128];
    for (i, v) in pow.iter().enumerate() {
        println!("2**{} = {}", i, v);
    }
}

#[test]
fn range_continued() {
    let mut pow = vec![0; 10];

    for i in 0..pow.len() {
        pow[i] = 1 << i;
    }

    for value in pow {
        println!("{}", value);
    }

    // Or with map
    pow = vec![0; 10].into_iter().enumerate().map(|(i, _)| 1 << i).collect();

    for value in pow {
        println!("{}", value);
    }
}

#[cfg(test)]
mod maps {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Vertex {
        lat: f64,
        long: f64,
    }

    // Cannot have heap allocated static variables
    // static m: HashMap<&str, Vertex> = HashMap::new();

    #[test]
    fn maps() {
        let mut m = HashMap::new();
        m.insert("Bell Labs", Vertex{lat: 40.68433, long: -74.39967});
        println!("{:?}", m.get("Bell Labs").unwrap());
    }

    #[test]
    fn mutating_maps() {
        let mut m = HashMap::new();
        m.insert("Answer", 42);
        println!("The value: {}", m.get("Answer").unwrap());
        m.insert("Answer", 48);
        println!("The value: {}", m.get("Answer").unwrap());
        m.remove("Answer");
        // Rust returns Option<T> for this, so if there isn't an entry, it is None
        println!("The value: {:?}", m.get("Answer"));
        let (v, ok) = match m.get("Answer") {
            Some(&v) => (v, true),
            None => (0, false),
        };
        println!("The value: {} Present? {}", v, ok);
    }
}

#[cfg(test)]
mod functions {
    fn compute(f: fn(f64, f64) -> f64) -> f64 {
        return f(3_f64, 4_f64);
    }

    #[test]
    fn function_values() {
        fn hypot(x: f64, y: f64) -> f64 { (x*x + y*y).sqrt() }
        println!("{}", hypot(5_f64, 12_f64));
        println!("{}", compute(hypot));
    }

    fn adder() -> Box<FnMut(i32) -> i32> {
        let mut sum = 0;
        Box::new(move |x| { sum += x; sum })
    }

    #[test]
    fn function_closures() {
        let (mut pos, mut neg) = (adder(), adder());
        for i in 0..9 {
            println!("{} {}", pos(i), neg(-2*i));
        }
    }

    fn fibonacci() -> Box<FnMut() -> i32> {
        let mut a = 0;
        let mut b = 1;
        Box::new(move || {
            let sum = a+b;
            let c = a + b;
            a = b;
            b = c;
            sum
        })
    }

    #[test]
    fn exercise_fibonacci_closure() {
        let mut f = fibonacci();
        for _ in 0..9 {
            println!("{}", f());
        }
    }
}
