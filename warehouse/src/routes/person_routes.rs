use crate::{entities::person, logic::handlers::person_handler};
use tonic::{Request, Response, Status};
use person::user_server::User;

#[derive(Default)]
pub struct UserCon {}

#[tonic::async_trait]
impl User for UserCon {
    async fn login(
        &self,
        request: Request<person::LoginRequest>,
    ) -> Result<Response<person::LoginResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            person_handler::login(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
    async fn create_user(
        &self,
        request: Request<person::CreateUserRequest>,
    ) -> Result<Response<person::CreateUserResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            person_handler::create_user(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
}
