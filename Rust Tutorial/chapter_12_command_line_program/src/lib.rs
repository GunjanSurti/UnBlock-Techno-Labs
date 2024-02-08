use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        // => returns 1st commandline argument
        // we are discarding it as 1st arg is path of file

        // query and filename are taking ownership of their string
        let query = match args.next() {
            Some(arg) => arg, // string inside Some() is owned string
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// original before modifying
// impl Config {
//     pub fn build(args: &[String]) -> Result<Config, &str> {
//         // Result<...> bcz to give clear error
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         // args[0] => target\\debug\\chapter_12_command_line_program.exe
//         let query = args[1].clone();
//         // clone bcz we dont want to take ownership
//         let filename = args[2].clone();
//         //Config cannot have &str without lifetime which makes complicated
//         // let case_sensitive: "Result<String, env::VarError>" = env::var("CASE_INSENSITIVE");
//         let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();
//         // "var" => takes an key to environment variable and  returns Result type,
//         // if key exists and set then the result will be Ok containing set value
//         // otherwise it results in error
//         // now "is_err" returns boolean
//         // if key is not set then "var" returns error,Returns true if the result is Err.

//         /* command to run in "bash" */
//         //$ export CASE_INSENSITIVE=true
//         // $ cargo run -- to poem.txt
//         //  unset CASE_INSENSITIVE

//         Ok(Config {
//             query,
//             filename,
//             case_sensitive,
//         })
//     }
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //we used the "trait object" Box<dyn Error>
    let contents = fs::read_to_string(config.filename)?;
    // here the "?" operator, the error type received will be converted to return (error) type of function
    // the new statement "fs::read_to_string" takes the file_path, opens that file, and returns a "std::io::Result<String>" of the file’s contents.
    // println!("With text:\n {}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // for line in search(&config.query, &contents) {
    //     // search return vector of strings and each string is a line which contains line
    //     // which we are looping over and printing them
    //     println!("{}", line);
    // }
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query)) // adapter, takes and return iterator
        .collect() // consumer, takes iterator and return  different data type
                   // here rust knows which collection should be return as it is specified in return type of function
}

// original before modifing
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     // we have only given lifetime to "contents" because "query" is always there
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         // looping through each in contents
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // this is case case_insensitive search
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        //line() => line-by-line iteration of strings
        // The lines method returns an iterator
        // to_lowercase returns new string so we are not modifing old string
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct"; // duct comes in pro"duct"ive
        let contents = "\
Rust : 
safe, fast ,productive.
Pick three.:
Duct Tape.";
        //(Note that the backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal)
        assert_eq!(vec!["safe, fast ,productive."], search(query, contents));
    }
    //     fn one_result() {
    //         let query = "duct"; // duct comes in pro"duct"ive
    //         let contents = "\
    // Rust :
    // safe, fast ,productive.
    // Pick three.:";
    //         assert_eq!(vec!["safe, fast ,productive."], search(query, contents));
    //     }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast ,productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
