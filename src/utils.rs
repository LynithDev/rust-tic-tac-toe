use std::io::{self, Write};

pub fn read_input(s: &str) -> String {
    print!("{s}");
    io::stdout().flush().expect("Could not flush stdout");
    let buf = &mut String::new();
    loop {
        match io::stdin().read_line(buf) {
            Ok(_) => break,
            Err(_) => {
                println!("Could not get input. Please try again");
            }
        };
    };
    buf.trim().to_owned()
}