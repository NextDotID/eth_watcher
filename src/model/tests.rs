use anyhow::Result;
use migration::TableCreateStatement;
use sea_orm::{DatabaseConnection, Database, Schema, DbBackend, ConnectionTrait};

async fn setup_schema(db: &DatabaseConnection) -> Result<()> {
    let schema = Schema::new(DbBackend::Sqlite);

    let stmt: TableCreateStatement = schema.create_table_from_entity(entities::chain::Entity);
    db.execute(db.get_database_backend().build(&stmt)).await?;

    Ok(())
}

pub(crate) async fn init_test_db() -> Result<DatabaseConnection> {
    let db = Database::connect("sqlite::memory:").await?;
    setup_schema(&db).await?;

    Ok(db)
}
