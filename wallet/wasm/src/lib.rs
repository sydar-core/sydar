use sydar_cli_lib::sydar_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_sydar_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    sydar_cli(options, None).await?;
    Ok(())
}
