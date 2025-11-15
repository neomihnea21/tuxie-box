use std::env;

//Prints the current working directory. Takes no arguments.
pub(crate) fn pwd(){ 
   let path = env::current_dir();
   match path{
      Ok(path_buf) => {
         println!("{}", path_buf.display()); 
      }
      Err(_) => {
         println!("[ERR]: We are lsot")
      }
   }
}