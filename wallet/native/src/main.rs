use sydar_cli_lib::{TerminalOptions, sydar_cli};

#[tokio::main]
async fn main() {
    let result = sydar_cli(TerminalOptions::new().with_prompt("$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}
