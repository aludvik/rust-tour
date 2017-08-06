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

#[cfg(test)]
mod switch {

    #[cfg(target_os="darwin")]
    mod runtime {
        pub const RUSTOS: &str = "darwin";
    }

    #[cfg(target_os="linux")]
    mod runtime {
        pub const RUSTOS: &str = "linux";
    }

    #[cfg(not(any(target_os="darwin", target_os="linux")))]
    mod runtime {
        pub const RUSTOS: &str = "other";
    }

    #[test]
    fn main() {
        print!("Rust runs on ");
        let os = match runtime::RUSTOS {
            "darwin" => "OS X.",
            "linux" => "Linux.",
            os => os,
        };
        println!("{}", os);
    }
}
