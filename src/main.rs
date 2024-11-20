use std::io::{self, Write};

fn print_help() {
    println!("#### SQLITE Rust Clone ###\n");
    println!("Helper \n");
    println!("Available Meta Commands: [Metacommands start with a dot] \n");
    println!("\t .help, .h => this prompt\n");
    println!("\t .quit, .q => quit program prompt\n");
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn is_metacommand(input_command: &str) -> bool {
    return input_command.starts_with('.');
}

fn sql_parser(sql_command: &str) {
    println!("Received SQL command: {}\n", sql_command);

    if sql_command.starts_with("insert") {
        println!("The received SQL command is an INSERT statement");
    } else if sql_command.starts_with("delete") {
        println!("The received SQL command is an DELETE statement");
    } else if sql_command.starts_with("select") {
        println!("The received SQL command is an SELECT statement");
    } else {
        println!("Unknown SQL command: {}\n", sql_command);
    }
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

        println!("Received input is: {}\n", input_command);

        if is_metacommand(input_command) {
            match input_command {
                ".quit" | ".q" => {
                        println!("Closing down application. Bye\n");
                        break;
                    },
                ".help" | ".h" => {
                    print_help();
                    continue;
                    },
                _ => println!("Received metacommand \"{}\" is not supported\n", input_command)
            }
        } else {
            sql_parser(input_command)
        }

    }
}