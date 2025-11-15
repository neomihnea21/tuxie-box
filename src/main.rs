

// Declare the echo module used to implement echo command
mod echo;
mod cat;
mod mkdir;
mod pwd;
mod grep;
mod chmod;
fn main() {
    // Retrieve the list of arguments
    let arguments: Vec<String> = std::env::args().collect();

    // Check if a command was specified
    let command = match arguments.get(1) {
        // Instead of panicking, you can display a help message with the list of all possible
        // commands.
        None => panic!("No command provided"),
        Some(command) => command,
    };

    // Check what command the user specified
    match command.as_str() {
        // Call the appropiate function. Since echo() function is defined in another module, we have
        // to use the full path.
        //
        // Also, the echo argument takes some arguments. Pass them to the command.
        "echo" => echo::echo(&arguments[2..]),
        "cat" => cat::cat(&arguments[2..]),
        "pwd" => pwd::pwd(),
        "mkdir" => mkdir::mkdir(&arguments[2..]),
        "grep" => grep::grep(&arguments[2..]),
        "chmod" => chmod::chmod(&arguments[2..]),
        _ => panic!("Unrecognized command"),
    }
}
