/// Display the given arguments.
pub(crate) fn echo(arguments: &[String]) {
    let mut start_index = 0;
    // Check if the argument no-newline has been specified. We assume it is always the first
    // argument.
    if let Some(argument) = arguments.get(0) {
        if argument == "-n" || argument == "--no-newline" {
            // An argument has been specified. Update the index of the first argument to be
            // displayed.
            start_index = 1;
        }
    }

    // Pass over the arguments.
    for argument in arguments[start_index..].iter() {
        // Print the given argument
        print!("{}", argument);
        // If the start_index is 0, then no-newline option has not been provided. Print a new line
        // after each argument.
        if start_index == 0{
            println!();
        }
    }
}
