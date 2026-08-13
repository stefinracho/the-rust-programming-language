use std::{env, error::Error, fs, process};

use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub ignore_case: bool,
    pub query: String,
    pub file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case;
        let query;
        let file_path;

        if args[1] == "-i" {
            ignore_case = true;

            if args.len() < 4 {
                return Err("not enough arguments");
            }

            query = args[2].clone();
            file_path = args[3].clone();
        } else {
            ignore_case = env::var("IGNORE_CASE").is_ok_and(|val| val == "1");
            query = args[1].clone();
            file_path = args[2].clone();
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
