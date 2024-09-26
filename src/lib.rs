use std::error::Error;
use std::fs;

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    println!("{:?}",config);
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config{
    pub query:String,
    pub filename:String,
}

impl Config {
    pub fn new(mut args:std::env::Args) -> Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("input not enough");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query arg"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no file name"),
        };
        Ok(Config{query,filename})
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result() {
        let query = "name";
        let contents = "\
my age is 18
my name is ml.
thanks";

        assert_eq!(vec!["my name is ml."],search(query,contents));
    }
}
pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    // let mut result = Vec::new();
    // for line  in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result
    contents.lines().filter(|line| line.contains(query)).collect()
}