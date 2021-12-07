use crate::{entities::person, logic::handlers::teacher_handler};
use person::teacher_server::Teacher;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct TeacherCon {}

#[tonic::async_trait]
impl Teacher for TeacherCon {
    async fn generate_code_and_start(
        &self,
        request: Request<person::GenerateCodeAndStartRequest>,
    ) -> Result<Response<person::GenerateCodeAndStartResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            teacher_handler::generate_code_and_start(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
    async fn edit_countdown(
        &self,
        request: Request<person::EditCountdownRequest>,
    ) -> Result<Response<person::EditCountdownResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            teacher_handler::edit_countdown(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }

    async fn delete_code(
        &self,
        request: Request<person::DeleteCodeRequest>,
    ) -> Result<Response<person::DeleteCodeResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            teacher_handler::delete_code(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
}
