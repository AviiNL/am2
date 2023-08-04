use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let sse = web::start(3000).await?;

    arma::start(sse.clone());

    tokio::spawn(async move {
        let mut test = data::Test { number: 0 };

        loop {
            test.number += 1;
            let _ = sse.push(&test);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
