use std::{env, error::Error, fs, process};

use minigrep::{search, search_case_insensitive, search_case_insensitive_iter, search_iter};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config = Config::build(env::args()).unwrap_or_else(|err| {
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
    pub iter: bool,
    pub query: String,
    pub file_path: String,
}

impl Config {
    // fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //
    //     let ignore_case;
    //     let query;
    //     let file_path;
    //
    //     if args[1] == "-i" {
    //         ignore_case = true;
    //
    //         if args.len() < 4 {
    //             return Err("not enough arguments");
    //         }
    //
    //         query = args[2].clone();
    //         file_path = args[3].clone();
    //     } else {
    //         ignore_case = env::var("IGNORE_CASE").is_ok_and(|val| val == "1");
    //         query = args[1].clone();
    //         file_path = args[2].clone();
    //     }
    //
    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }

    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let _program_name = args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok_and(|val| val == "1");
        let iter = env::var("ITER").is_ok_and(|val| val == "1");

        Ok(Config {
            query,
            file_path,
            ignore_case,
            iter,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    match config.ignore_case {
        true => match config.iter {
            true => {
                for line in search_case_insensitive_iter(&config.query, &contents) {
                    println!("{line}");
                }
            }
            false => {
                for line in search_case_insensitive(&config.query, &contents) {
                    println!("{line}");
                }
            }
        },
        false => match config.iter {
            true => {
                for line in search_iter(&config.query, &contents) {
                    println!("{line}");
                }
            }
            false => {
                for line in search(&config.query, &contents) {
                    println!("{line}");
                }
            }
        },
    }

    Ok(())
}
