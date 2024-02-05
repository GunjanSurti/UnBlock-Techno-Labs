use std::error::Error;
use std::{env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    // used to take arguments from command line

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }); // this is to reduce noise which is shown in terminal in case of error

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n {}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone(); // clone bcz we dont want to take ownership
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
