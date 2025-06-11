use twodo::Result;
use twodo::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
