extern crate rand;
use basics::rand::Rng;

#[test]
fn packages() {
    println!("My favorite number is {}", rand::thread_rng().gen_range(1, 10));
}

#[test]
fn imports() {
    let f = 7 as f64;
    println!("Now you have {} problems.", f.sqrt());
}

use std::f64;

#[test]
fn exported_names() {
    println!("{}", f64::consts::PI);
}


#[cfg(test)]
mod functions {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn main() {
        println!("{}", add(42, 13));
    }
}

#[cfg(test)]
mod multiple_results {
    fn swap(x: String, y: String) -> (String, String) {
        (y, x)
    }

    #[test]
    fn main() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let (s1, s2) = swap(s1, s2);
        println!("{} {}", s1, s2);
    }
}

#[cfg(test)]
mod variables {
    // Static variables require explicit initialization
    // Rust throws a warning if globals are not capitalized (but still compiles)
    static C: bool = true;
    static PYTHON: bool = true;
    static JAVA: bool = true;

    #[test]
    fn main() {
        // Rust does not automatically initialize declared variables to any default variables.
        // The compiler will not accept code that attempts to access a potentially unitialized
        // variable.
        let i: i32 = 0;
        println!("{} {} {} {}", i, C, PYTHON, JAVA);
    }
}

#[test]
fn short_variable_declarations() {
    // Pattern matching can be used to unpack a tuple
    let (i, j) = (1, 2);
    // Rust supports type inference when the type is unambiguous
    let k = 3;

    let (c, python, java) = (true, false, "no!");
    println!("{} {} {} {} {} {}", i, j, k, c, python, java);

}

#[cfg(test)]
mod basic_types {
    static TO_BE: bool = false;
    static MAX_INT: u64 = 1<<64 - 1;
    // Rust does not support complex numbers in the standard library

    #[test]
    fn main() {
        // Rust does not support printing the types of variables
        println!("Value: {}", TO_BE);
        println!("Value: {}", MAX_INT);
    }
}

#[cfg(test)]
mod type_conversions {
    #[test]
    fn main() {
        let (x, y): (i32, i32) = (3, 4);
        let f: f64 = ((x*x + y*y) as f64).sqrt();
        let z: u32 = f as u32;
        println!("{} {} {}", x, y, z);
    }
}

#[cfg(test)]
mod constants {
    const PI: f32 = 3.14;

    #[test]
    fn main() {
        const WORLD: &str = "世界";
        println!("Hello {}", WORLD);
        println!("Happy {} Day", PI);

        const TRUTH: bool = true;
        println!("Go rules? {}", TRUTH);
    }
}

// Rust constants are not "magic". They don't get cast automatically and don't have arbitrary
// precision. The following will not compile.
//
// #[cfg(test)]
// mod numeric_constants {
//     const BIG: i64 = 1 << 100;
//     const SMALL: i64 = BIG >> 99;
//
//     fn need_int(x: i64) -> i64 { return x*10 + 1 }
//     fn need_float(x: f64) -> f64 {
//         x * 0.1
//     }
//
//     #[test]
//     fn main() {
//         println!("{}", need_int(SMALL));
//         println!("{}", need_float(SMALL);
//         println!("{}", need_float(BIG);
//     }
// }

#[cfg(test)]
mod for_ {
    #[test]
    fn main() {
        let mut sum = 0;
        for i in 0..9 {
            sum += i;
        }
        println!("{}", sum);
    }
}

#[cfg(test)]
mod for_continued {
    #[test]
    fn main() {
        let mut sum = 1;
        while sum < 1000 {
            sum += sum;
        }
        println!("{}", sum);
    }
}

#[test]
fn forever() {
    loop {
        break;
    }
}

#[cfg(test)]
mod if_ {
    fn sqrt(x: f64) -> String {
        match x {
            x if x < 0_f64 => format!("{}i", (-x).sqrt()),
            _ => format!("{}", x.sqrt()),
        }
    }

    #[test]
    fn main() {
        println!("{} {}", sqrt(2_f64), sqrt(-4_f64));
    }
}

#[cfg(test)]
mod exercise_loops_and_functions {
    fn sqrt(x: f64) -> f64 {
        let mut last_guess = x;
        let mut guess = last_guess - (last_guess * last_guess - x) / (2_f64 * x);
        while (guess - last_guess).abs() > 0.0001 {
            last_guess = guess;
            guess = last_guess - (last_guess * last_guess - x) / (2_f64 * x);
        }
        guess
    }

    #[test]
    fn main() {
        println!("{}", sqrt(2_f64));
        println!("{}", sqrt(3_f64));
        println!("{}", sqrt(4_f64));
    }
}
