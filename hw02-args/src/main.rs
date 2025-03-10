use slug::slugify;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        match args[1].as_str() {
            "lowercase" => println!("{}", args[2].to_lowercase()),
            "uppercase" => println!("{}", args[2].to_uppercase()),
            "no-spaces" => println!("{}", args[2].replace(" ", "")),
            "slugify" => println!("{}", slugify(&args[2])),
            _ => println!("Unsupported conversion: {}", args[1]),
        }
    } else {
        println!("Missing parameters!");
    }
}
