// make an empty directory from its relative path

use std::fs;

pub(crate) fn mkdir(arguments: &[String]){
    let path = arguments.get(0);
     match path{
        Some(path) => {
            let res = fs::create_dir(path);
            match res{
                Ok(_) => {
        
                }
                Err(_) => {
                    println!("[ERR]: filepath inaccessible");
                }
            }
        }
        None => {
           println!("[ERR]: No path was given");
        }
    }
}