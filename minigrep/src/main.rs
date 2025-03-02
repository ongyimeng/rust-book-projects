use std::env;
use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing config: {}", err);
        process::exit(1);
    });
    
    let file_content = match run(&config) {
        Err(e) => {
            eprintln!("Error reading file content: {}", e);
            process::exit(1);
        }

        Ok(file_content) => {
            file_content
        }
    };

    let search_results = search(&config.query, &file_content, &config.case_sensitive);   

    for result in search_results.iter() {
        println!("{}", result);
    }
    
}