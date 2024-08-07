use std::fs;
use std::error::Error;
use std::env;

pub fn run (config :Config)-> Result<(), Box<dyn Error>>{

    let content = fs::read_to_string(config.fileName)?;    
    // println!("{:?}", content);

    let results= if config.case_sensitive{
            search(&config.query,&content)
    }
    else{
        search_case_insensitive(&config.query, &content)
    };

    for line in results { 
            println!("{}", line);

    }

    Ok(())
}

pub struct Config{
    pub query: String,
    pub fileName: String,
    pub case_sensitive: bool,
}

impl Config{

    pub fn new(args :&[String])-> Result<Config, &str>{

        if args.len()<3{
         return Err("NoT enough Arguments");
        }
    
        let query = args[1].clone();
        let fileName =args[2].clone();
    
        let case_sensitive= env::var("CASE_SENSITIVE").is_err();


        Ok(Config{ query, fileName, case_sensitive})
    }
    
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    let mut resultVec:Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            resultVec.push(line);
        }
    }

    resultVec
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    let query = query.to_lowercase();

    let mut resultVec:Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            resultVec.push(line);
        }
    }

    resultVec


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
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
}