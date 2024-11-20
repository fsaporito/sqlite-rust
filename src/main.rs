use std::io::{self, Write};

fn print_help() {
    println!("#### SQLITE Rust Clone ###\n");
    println!("Helper \n");
    println!("Available Commands:\n");
    println!("\t .help, .h => this prompt\n");
    println!("\t .quit, .q => quit program prompt\n");
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush stdout")
}

fn main() {
    println!("Hello I am an SQLITE Rust Clone :)\n");

    loop {

        print_prompt();

        let mut input_command = String::new();

        io::stdin()
        .read_line(&mut input_command)
        .expect("Error reading input command");

        let input_command = input_command.trim();

        println!("Received command is: {}\n", input_command);

        match input_command {
            ".quit" | ".q" => {
                    println!("Closing down application. Bye\n");
                    break;
                },
            ".help" | ".h" => {
                print_help();
                continue;
                },
            _ => println!("Received command \"{}\" is not supported\n", input_command)
        }

    }
}