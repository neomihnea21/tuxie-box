use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use regex::Regex;

pub(crate) fn grep(arguments: &[String]){
    let regex = arguments.get(0);
    let start_index = 1; //if there are filepaths at all, they will begin at pposition 1
    match regex{
      Some(regex) => {
         //TODO find some way to cast regex HERE, don't do N checks
         for arg in arguments[start_index..].iter(){
            let file = File::open(arg);
            //this will panic for nonexistent files, TODO fix it
            match file{
              Ok(file) => {
                let buffer = BufReader::new(file);
                for line in buffer.lines(){
                    match line{
                        Ok(line) => {
                            let compiled_regex = Regex::new(regex);
                            match compiled_regex{
                              Ok(compiled_regex) => 
                              if Regex::is_match(&compiled_regex,&line){
                                println!("{}", line);
                              }
                              Err(_) =>{ 
                                //if the pattern is no pattern, we cannot grep with it
                                println!("[ERR] Invalid regex");
                                break;
                              }
                            }
                        }
                        Err(_) => {
                           
                        }
                    }
                }
              }
              Err(_) => {
                 println!("[WARN]: {arg} does not exist")
              }
            }
         }
      }
      None => {
        println!("[ERR] No pattern was provided")
      }
    }
}