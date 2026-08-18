pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    //
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_iter<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    //
    // results
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search_case_insensitive_iter<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive_iter() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert!(search_iter(query, contents).eq(["safe, fast, productive."].into_iter()))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_iter() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert!(
            search_case_insensitive_iter(query, contents).eq(["Rust:", "Trust me."].into_iter())
        )
    }
}
