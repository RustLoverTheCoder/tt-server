pub use sea_orm_migration::prelude::*;

mod m20230422_024420_create_user;
mod m20230422_041821_create_chat;
mod m20230422_044814_create_message;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230422_024420_create_user::Migration),
            Box::new(m20230422_041821_create_chat::Migration),
            Box::new(m20230422_044814_create_message::Migration),
        ]
    }
}
