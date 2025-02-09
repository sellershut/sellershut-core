use sellershut_core::users::query_users_client::QueryUsersClient;
// run the server first
use tonic::IntoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QueryUsersClient::connect("http://[::1]:50051")
        .await
        .expect("is server running?");

    let response = client
        .query_users(sellershut_core::google::protobuf::Empty::default().into_request())
        .await?;

    println!("response={response:?}");

    Ok(())
}
