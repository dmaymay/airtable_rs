#[derive(Serialize)]
struct PostData {
    title: String,
    body: String,
    userId: u32,
}

#[tokio::main]
async fn get_request() -> Result<(), reqwest::Error> {
    let url = "https://httpdump.app/dumps/c2f0c182-9a93-418a-bdcc-fb96b3006b92";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn post_request() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "https://httpdump.app/dumps/c2f0c182-9a93-418a-bdcc-fb96b3006b92";

    let new_post = PostData {
        title: "Hello Rust".to_string(),
        body: "This is a test post.".to_string(),
        userId: 1,
    };

    let response = client.post(url).json(&new_post).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

