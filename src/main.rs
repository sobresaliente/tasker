use std::io;

fn main() {
    const PURPLE: &str = "\x1b[38;5;165m";
    const RESET_ANSI: &str = "\x1b[0m";
    const STARTING_TEXT: &str = "Hello, master!";
    let purple_text = format!("{}{}{}", PURPLE, STARTING_TEXT, RESET_ANSI);
    println!("{}", purple_text);
    println!("What is your current task?");

    loop {
        let mut current_task = String::new();

        io::stdin()
            .read_line(&mut current_task)
            .expect("Failed to read line");

        println!("You are working on {}{}{}", PURPLE, current_task, RESET_ANSI);
        println!("Press Ctrl+C to exit");
    }
}
