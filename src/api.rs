// This function get data from an API using the libraries tokio and reqwest
// In the next step we will be using the library serde to parse the JSON 

#[tokio::main]

async fn main() -> Result<(), reqwest::Error>{
    let todos = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .text()
        .await?;

        println!("{:#?}", todos);
    Ok(())
}