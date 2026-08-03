use std::{
    collections::{HashMap, hash_map::Entry},
    io::{self, Write},
};

fn main() {
    println!(
        "This program lets you `add` people to a department in your company, or `list` people in a department or company.\n"
    );

    let mut departments = HashMap::from([
        ("Engineering".to_string(), vec![]),
        ("Marketing".to_string(), vec![]),
        ("Finance".to_string(), vec![]),
    ]);
    println!("Your company has 3 departments:");
    for department in departments.keys() {
        println!("\t{department}")
    }
    println!();

    loop {
        print!("Command (add|list|quit): ");
        io::stdout().flush().expect("stdout should flush");

        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line.");
        let cmd = cmd.trim();

        if cmd.eq_ignore_ascii_case("add") {
            // Get name of person
            let mut person = String::new();
            loop {
                print!("Name of person you want to add: ");
                io::stdout().flush().expect("stdout should flush");
                io::stdin()
                    .read_line(&mut person)
                    .expect("Failed to read line.");
                person = person.trim().to_string();

                if !person.is_empty() {
                    break;
                }
                println!("Invalid name. Try again.");
            }

            // Add person to department
            loop {
                print!("Name of department you want {person} to be added to: ");
                io::stdout().flush().expect("stdout should flush");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line.");
                let department = department.trim();

                match departments.entry(department.to_string()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(person.to_string());
                        println!("Added {person} to {department}\n");
                        break;
                    }
                    Entry::Vacant(_) => println!("Department does not exist. Try again."),
                }
            }
        } else if cmd.eq_ignore_ascii_case("list") {
            println!("{:?}", departments)
        } else if cmd.eq_ignore_ascii_case("quit") {
            break;
        } else {
            println!("Invalid Command");
            println!("Acceptable commands are `add`, `list`, or `quit`");
        }
    }
}
