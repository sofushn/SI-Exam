tonic::include_proto!("group");


#[derive(sqlx::FromRow)]
pub struct FullGroup{
    pub name: String
}