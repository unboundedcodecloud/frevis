use::std::env::consts;

fn main() {
    println!("**** Y'llo. Me be Frevis. ***");
    println!("**** My dreams, they aren't as empty as my conscience seems to be! ***");
    // println!("Let's see where I'm...{}", get_os());
}

fn get_os() -> String {
    pub const OS: &str = consts::OS;
    OS.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn setup(){
        assert_eq!(get_os(), "linux");
    }
}