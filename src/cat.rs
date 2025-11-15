use std::fs;

// Lists the contents of a file given as argument.
pub(crate) fn cat(arguments: &[String]){
    //the first argument is sth readable
     let path = arguments.get(0);
     match path{
        Some(path) => {
           let construct = fs::read_to_string(path);
           match construct {
           Ok(strings) => {
              println!("{}", strings);
           } 
           Err(_) => {
              println!("[ERR]: Invalid file path or no rights to read file")
           }
          }
        }
        None => {
           println!("[ERR]: No file was given");
        }
     }
}