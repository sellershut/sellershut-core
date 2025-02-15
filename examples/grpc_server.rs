use sellershut_core::google::protobuf;
use sellershut_core::users::query_users_server::{QueryUsers, QueryUsersServer};
use sellershut_core::users::QueryUserByIdRequest;
use sellershut_core::users::QueryUserByIdResponse;
use sellershut_core::users::QueryUserByNameRequest;
use sellershut_core::users::QueryUserByNameResponse;
use sellershut_core::users::QueryUsersFollowingRequest;
use sellershut_core::users::QueryUsersFollowingResponse;
use sellershut_core::users::QueryUsersResponse;
use tonic::{transport::Server, Request, Response, Status};

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

    async fn query_user_by_id(
        &self,
        request: Request<QueryUserByIdRequest>,
    ) -> Result<Response<QueryUserByIdResponse>, Status> {
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
