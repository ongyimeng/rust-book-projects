use std::{fs, process};
use std::error::Error;
use std::env;
use rstest::rstest;

pub fn search<'a>(query: &'a str, file_content: &'a str, case_sensitive: &'a bool) -> Vec<&'a str> {
    let mut search_results: Vec<&str> = Vec::new();
    
    let query = if *case_sensitive {
        query
    }else {
        &query.to_lowercase()
    };

    for line in file_content.lines() {
        let local_line = if *case_sensitive {
            line
        } else {
            &line.to_lowercase()
        };
        
        if local_line.contains(query) {
            search_results.push(line);
        }
    }
    search_results
}

// Reads file to get file content
pub fn run(config: &Config) -> Result<String, Box<dyn Error>>{
    let file_content = fs::read_to_string(&config.file_name)?;
    Ok(file_content)
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {
        
        if args.len() < 3 {
            return Err("Not enough args".to_string());
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        // If env var not set then false
        let case_sensitive: bool = match env::var("CASE_SENSITIVE") {
            Ok(case_sensitive) => {
                if case_sensitive == "1" {
                    true
                } else {
                    false
                }
            },
            Err(_e) => false
        };

        Ok(Self{ query, file_name, case_sensitive})
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::ffi::OsStringExt;

    use super::*;

    #[test]
    fn test_config_new_success() {
        let args = vec!["debug".to_string(), "arg1".to_string(), "arg2".to_string()];
        if let Ok(config) = Config::new(&args) {
            assert_eq!(config.query, "arg1");
            assert_eq!(config.file_name, "arg2");
        }
    }

    #[test]
    #[should_panic(expected = "Not enough args")]
    fn test_config_new_panic_no_args() {
        let args = vec!["debug".to_string()];
        let _config = Config::new(&args);
    }

    #[rstest]
    #[case(vec!["debug".to_string(), "arg1".to_string()])]
    #[case(vec!["debug".to_string(), "arg2".to_string()])]
    #[should_panic(expected = "Not enough args")]
    fn test_config_new_panic_one_arg(#[case] args: Vec<String>) {
        let _config = Config::new(&args);
    }

    #[test]
    fn test_search() {
        let query: String  = String::from("duct");
        let contents: String = String::from("\
Rust:
safe, fast, productive.
Pick three.");
        let case_sensitive = true;
        assert_eq!(
            vec!["safe, fast, productive."],
            search(&query, &contents, &case_sensitive)
        );
    }

    #[test]
    fn test_search_case_sensitive() {
        let query= "rUsT".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.".to_string();
        let case_sensitive = false;
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(&query, &contents, &case_sensitive)
        );
    }
}