use std::error::Error;

use axum_learn::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await?;

    Ok(())
}
