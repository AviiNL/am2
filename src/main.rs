use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let sse = web::start(3000).await?;

    arma::start(sse);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
