#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn sandbox() {
        println!("Welcome to the playground!");
        // println!("The time is {}", SystemTime::now());
        // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("The time is {}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    }
}
