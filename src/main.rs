use std::env;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let _path: &'static str = env!("CARGO");

    println!("**** Y'llo. Me be Frevis. ***");
    println!("**** My dreams, they aren't as empty as my conscience seems to be! ***");
    println!("{}", {_path});

}

