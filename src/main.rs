use std::env;
mod aoc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass a problem to solve.");
    }
    
    else {
        println!("Got arg {}", args[1]);

    let main_arg = &args[1];
    let foo = [1,2,3]

        match &main_arg[..] {
            "day1" => aoc::day1::calculate_chronal_frequency(&foo)
            "foobar" => println!("got dat foobar"),
            "baz" => println!("got dat baz"),
            _ => println!("got something else"),
        };
    }
}
