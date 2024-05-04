// run the server first
use sellershut_core::category::query_categories_client::QueryCategoriesClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QueryCategoriesClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(sellershut_core::common::Empty::default());

    let response = client.categories(request).await?;

    println!("response={response:?}");

    Ok(())
}
