use std::process;

use backend::run;

#[tokio::main]
async fn main() {
    let code = match run().await {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("An error occurred so we are crashing");
            eprintln!("{error:?}");
            1
        }
    };

    process::exit(code);
}
