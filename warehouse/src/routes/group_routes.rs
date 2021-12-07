use crate::entities::group;
use crate::logic::handlers::group_handler;
use group::group_server::Group;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GroupCon {}

#[tonic::async_trait]
impl Group for GroupCon {
    async fn create_group(
        &self,
        request: Request<group::CreateGroupRequest>,
    ) -> Result<Response<group::CreateGroupResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            group_handler::create(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
    async fn delete_group(
        &self,
        request: Request<group::DeleteGroupRequest>,
    ) -> Result<Response<group::DeleteGroupResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            group_handler::delete(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
    async fn remove_student_from_group(
        &self,
        request: Request<group::RemoveStudentFromGroupRequest>,
    ) -> Result<Response<group::RemoveStudentFromGroupResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            group_handler::remove_student_from_group(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
    async fn add_student_to_group(
        &self,
        request: Request<group::AddStudentToGroupRequest>,
    ) -> Result<Response<group::AddStudentToGroupResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            group_handler::add_student_to_group(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }

    async fn get_all_groups_by_username(
        &self,
        request: Request<group::GetAllGroupsByUsernameRequest>,
    ) -> Result<Response<group::GetAllGroupsByUsernameResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            group_handler::get_all_groups_by_username(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }

    async fn get_all_students_by_group_name(
        &self,
        request: Request<group::GetAllStudentsByGroupNameRequest>,
    ) -> Result<Response<group::GetAllStudentsByGroupNameResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        Ok(Response::new(
            group_handler::get_all_students_by_group_name(request.into_inner())
                .await
                .expect("Person Update failed"),
        ))
    }
}
