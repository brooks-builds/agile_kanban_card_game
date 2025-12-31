use std::process;

use anathema_frontend::run;

fn main() {
    match run() {
        Ok(()) => println!("thanks for playing"),
        Err(error) => {
            eprintln!("{error:?}");
            process::exit(1);
        }
    }
}
