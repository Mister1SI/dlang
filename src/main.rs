// dlang bin crate root
// Contains logic for processing cl arguments and invoking the compiler

use std::env;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_help();
        return Ok(());
    } else {
        for arg in &args[1..] { 
            if arg.starts_with("--") {
                println!("Option: {}", arg);
            } else if arg.starts_with('-') {
                for flag in arg[1..].chars() {
                    match flag {
                        'h' => print_help(),
                        'v' => println!("dlang version {}", env::var("CARGO_PKG_VERSION").unwrap()),
                        _ => println!("Unknown flag {}.", flag),
                    }
                }
            } else {
                println!("File supplied: {}", arg);
            }
            
        }
    } // End args processing

    Ok(())
}

fn print_help() {
    let help_text = r#"help text"#;
    println!("{}", help_text);
}
