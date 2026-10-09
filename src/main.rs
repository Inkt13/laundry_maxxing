
use reqwest::multipart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let form = multipart::Form::new()
        .text("goodsId", "1100860895");

    let response = client
        .post("https://userapi.qiekj.com/goods/normal/details")
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://h5.qiekj.com")
        .header("Referer", "https://h5.qiekj.com/")
        .header("phoneBrand", "DEFAULT")
        .header("uid", "undefined")
        .multipart(form)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    println!("Status: {status}");
    println!("Response:\n{body}");

    Ok(())
}