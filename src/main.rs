const API: &str = "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd";
const CUSTOM_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
use reqwest::header::USER_AGENT;
use reqwest::Client;


#[tokio::main]
async fn main() {
    get_and_print_data().await;
}


async fn get_and_print_data() {
    let data = Client::new()
        .get(API)
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .send()
        .await
        .expect("Error")
        .text()
        .await
        .expect("Error");

    println!("{:#?}", data)
}
