use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please pass a problem to solve.");
    }
    
    else {
        println!("Got arg {}", args[1]);

    let main_arg = &args[1];

        match &main_arg[..] {
            "foobar" => println!("got dat foobar"),
            "baz" => println!("got dat baz"),
            _ => println!("got something else"),
        };
    }
}
