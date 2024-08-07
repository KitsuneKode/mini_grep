use std::env;
use std::process;
use colored::*;
use mini_grep::Config;

fn main() {

    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(| err | {
            eprintln!("Problem with parsing the arguments: {}", err);
            process::exit(1);
    }); 

    
   
    println!("Searching query: \"{}\" in the file: {}", config.query.red(), config.fileName.bright_green());

    if let Err(e) = mini_grep::run(config){
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
 
}

