use sellershut_core::google::protobuf;
use sellershut_core::users::QueryUserByApIdRequest;
use sellershut_core::users::QueryUserByApIdResponse;
use sellershut_core::users::QueryUserByNameRequest;
use sellershut_core::users::QueryUserByNameResponse;
use sellershut_core::users::QueryUsersFollowingRequest;
use sellershut_core::users::QueryUsersFollowingResponse;
use sellershut_core::users::QueryUsersResponse;
use sellershut_core::users::query_users_server::{QueryUsers, QueryUsersServer};
use tonic::{Request, Response, Status, transport::Server};

#[derive(Default)]
pub struct UserService;

#[tonic::async_trait]
impl QueryUsers for UserService {
    async fn query_users(
        &self,
        _request: Request<protobuf::Empty>,
    ) -> Result<Response<QueryUsersResponse>, Status> {
        todo!()
    }

    async fn query_user_by_name(
        &self,
        request: Request<QueryUserByNameRequest>,
    ) -> Result<Response<QueryUserByNameResponse>, Status> {
        todo!()
    }

    async fn query_user_by_ap_id(
        &self,
        request: Request<QueryUserByApIdRequest>,
    ) -> Result<Response<QueryUserByApIdResponse>, Status> {
        todo!()
    }

    async fn query_user_following(
        &self,
        request: Request<QueryUsersFollowingRequest>,
    ) -> Result<Response<QueryUsersFollowingResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("starting server on {addr:?}");
    Server::builder()
        .add_service(QueryUsersServer::new(UserService))
        .serve(addr)
        .await?;

    Ok(())
}
