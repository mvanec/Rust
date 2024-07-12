use reqwest::Error;
use tokio::sync::mpsc;

async fn fetch_url(url: String) -> Result<String, Error> {
    // Implement the logic to fetch a single URL
    Ok("Url".to_string())
}

async fn process_url(url: String, tx: mpsc::Sender<String>) {
    // Implement the logic to process a URL and send results through the channel
}

pub async fn crawl() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://example.com".to_string(),
        "https://example.org".to_string(),
        // Add more URLs here
    ];
    let (tx, mut rx) = mpsc::channel(100);
    for url in urls {
        let tx = tx.clone();
        tokio::spawn(async move {
            process_url(url, tx).await;
        });
    }
    drop(tx); // Close the sending end in the main task
    while let Some(result) = rx.recv().await {
        println!("Processed: {}", result);
    }
    Ok(())
}
