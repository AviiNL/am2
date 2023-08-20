use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let sse = web::start(3000).await?;

    arma::start(sse.clone());

    tokio::signal::ctrl_c().await?;

    Ok(())
}
