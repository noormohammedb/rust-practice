use std::{env, error::Error, fs, result};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query.as_str(), &contents)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query = match args.next() {
            Some(data) => data,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(data) => data,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // println!("case_sensitive: {:?}", case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let foo: String = contents.lines().map(|line| line.to_lowercase()).collect();
    // println!("foo , lower lines: {}", foo);

    // let bar: Vec<String> = contents
    //     .lines()
    //     .map(|line| line.to_lowercase())
    //     .filter(|line| line.contains(query))
    // .filter(|line| 1 == 1)
    // .collect();
    // .collect()

    // println!("bar: {:?}", bar);

    // let result: Vec<_> = contents
    // contents
    //     .lines()
    //     .map(|line| {
    //         if (line.to_lowercase().contains(query)) {
    //             return line;
    //         } else {
    //             return "";
    //         }
    //     })
    //     .filter(|line| {
    //         let f = line.to_lowercase();
    //         // println!("lower lines: {}", f);
    //         line.contains(query)
    //     })
    //     // .map(|item| item.as_str())
    //     .collect()

    // result.iter().map(|item| item.as_str()).collect()
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me..";

        assert_eq!(
            vec!["Rust:", "Trust me.."],
            search_case_insensitive(query, contents)
        );
    }
}
