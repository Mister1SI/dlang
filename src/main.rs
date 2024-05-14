// dlang bin crate root
// Contains logic for processing cl arguments and invoking the compiler

use std::env;
use dlang::compile;

fn main() -> Result<(), std::io::ErrorKind> {
    let args: Vec<String> = env::args().collect();
    let mut input_files: Vec<String> = Vec::new();
    let mut options: Vec<String> = Vec::new();
    let mut flags: String = String::new();
    if args.len() == 1 {
        print_help();
        return Ok(());
    } else {
        for arg in &args[1..] { 
            if arg.starts_with("--") {
                options.push(arg[2..].to_string());
            } else if arg.starts_with('-') {
                for flag in arg[1..].chars() {
                    match flag {
                        'h' => print_help(),
                        'v' => println!("dlang version {}", env::var("CARGO_PKG_VERSION").unwrap()),
                        'e'|'l' => {
                            flags.push(flag);
                        }
                        _ => println!("Unknown flag {}.", flag),
                    }
                }
            } else {
                input_files.push(arg.to_string());
            }
            
        }
    } // End args processing
    match compile(input_files, options, flags) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.kind());
        }
    };
    
    Ok(())
}

fn print_help() {
    let help_text = r#"help text"#;
    println!("{}", help_text);
}
