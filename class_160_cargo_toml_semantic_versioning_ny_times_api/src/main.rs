
/* 
API key is required to access The New York Times API.
The API key is used to authenticate your requests to the New York Times servers
*/

use serde_json::Value;
use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // match dotenvy::dotenv() {
    //     Ok(_) => println!("Successfully loaded .env file"),
    //     Err(e) => println!("Failed to load .env file: {}", e),
    // }

    dotenvy::dotenv().expect("Failed to load .env file");
    
    // Replace with your New York Times API key
    let api_key = env::var("NY_TIMES_API_KEY").expect("NY_TIMES_API_KEY must be set");

    let url = format!("https://api.nytimes.com/svc/topstories/v2/home.json?api-key={}", api_key);

    let response = reqwest::get(&url).await?.json::<Value>().await?;

    // println!("{}", response);
	

    let headlines = response["results"].as_array().unwrap();

    println!("\n\n-----------------------\n\n");

    for (i, article) in headlines.iter().take(5).enumerate() {
        let title = article["title"].as_str().unwrap_or("No title");
        println!("Headline {}: {}", i + 1, title);
    }

    println!("\n\n-----------------------\n\n");



    Ok(())
}