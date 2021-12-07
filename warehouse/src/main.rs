use std::net::SocketAddr;
use tonic::transport::Server;

mod connection;
mod entities;
mod logic;
mod routes;
mod utils;

use entities::{
    group::group_server::GroupServer, person::student_server::StudentServer,
    person::teacher_server::TeacherServer, person::user_server::UserServer,
};
use routes::group_routes::GroupCon;
use utils::config::{is_production_mode, is_testing_mode, CONFIG};

use crate::routes::{student_routes::StudentCon, teacher_routes::TeacherCon, person_routes::UserCon};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ADDR: SocketAddr = {
        if is_production_mode() {
            let formatted_addr_string: &str = &format!(
                "{}:{}",
                CONFIG.production.server.host, CONFIG.production.server.port
            );
            formatted_addr_string
                .parse()
                .expect("Couldn't parse addr string")
        } else if is_testing_mode() {
            let formatted_addr_string: &str = &format!(
                "{}:{}",
                CONFIG.testing.server.host, CONFIG.testing.server.port
            );
            formatted_addr_string
                .parse()
                .expect("Couldn't parse addr string")
        } else {
            let formatted_addr_string: &str = &format!(
                "{}:{}",
                CONFIG.development.server.host, CONFIG.development.server.port
            );
            formatted_addr_string
                .parse()
                .expect("Couldn't parse addr string")
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let group_con = GroupCon::default();
    let user_con = UserCon::default();
    let teacher_con = TeacherCon::default();
    let student_con = StudentCon::default();

    println!("gRPC server listening on {}", ADDR.to_owned());

    Server::builder()
        .add_service(GroupServer::new(group_con))
        .add_service(UserServer::new(user_con))
        .add_service(TeacherServer::new(teacher_con))
        .add_service(StudentServer::new(student_con))
        .serve(ADDR.to_owned())
        .await?;

    Ok(())
}
