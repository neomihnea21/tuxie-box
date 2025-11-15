use std::{fs::{self, Permissions}, os::unix::fs::PermissionsExt};

//this changes permissions for a  file
//and expects 2 arguments: a 3-digit string and a path
pub(crate) fn chmod(arguments :&[String]){
   let code = arguments.get(0).unwrap();
   let path=arguments.get(1).unwrap();
   let updated_perms :Permissions;
   if code == "ugrw" {
      updated_perms = Permissions::from_mode(0o664);
   }
   else if code == "+xa" {
      let perms = fs::metadata(path).unwrap().permissions();
      let mut current_mode = perms.mode();
      current_mode |= 0o111;
      updated_perms=Permissions::from_mode(current_mode);
   }
   else{
     let new_perms = u32::from_str_radix(code, 8);
     updated_perms = Permissions::from_mode(new_perms.unwrap());
   }
   let _ = fs::set_permissions(path, updated_perms);
}