use sellershut_core::{
    category::{
        query_categories_server::{QueryCategories, QueryCategoriesServer},
        ResponseCategories
    },
    common::{Empty, SearchQuery, Status},
};
use tonic::{transport::Server, Response};

#[derive(Default)]
pub struct CategoryService;

#[tonic::async_trait]
impl QueryCategories for CategoryService {
    #[doc = " gets all categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn categories(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<ResponseCategories>, tonic::Status> {
        println!("Got a request: {request:?}");

        // let req = request.into_inner();

        let reply = ResponseCategories {
            status: Some(Status {
                code: 0,
                message: "Ok".into(),
            }),
            categories: vec![],
        };

        Ok(Response::new(reply))
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn category_by_id(
        &self,
        _request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<ResponseCategories>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn sub_categories(
        &self,
        _request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<ResponseCategories>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn search(
        &self,
        _request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<ResponseCategories>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(QueryCategoriesServer::new(CategoryService))
        .serve(addr)
        .await?;

    Ok(())
}
