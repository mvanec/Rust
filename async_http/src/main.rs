mod webcrawl;

use reqwest::Error;
use async_http::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Make an asynchronous GET request
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?  // Wait for the request to complete and propagate any errors
        .text()  // Convert the response body to text
        .await?; // Wait for the text conversion and propagate any errors
    println!("Response: {}", response);

    // let my_future = MyFuture { state: 0 };
    // let result = my_future.await;
    // println!("Result: {}", result);

    match read_file("nonexistent.txt").await {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    Ok(())
}
