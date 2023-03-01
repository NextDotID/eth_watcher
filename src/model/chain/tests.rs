use entities::chain::ActiveModel;
use sea_orm::ActiveModelTrait;

use crate::model::tests::init_test_db;

use super::*;

async fn generate_data(db: &DatabaseConnection) -> Result<ChainModel> {
    use sea_orm::ActiveValue;
    let chain_am = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set("polygon".into()),
        rpc: ActiveValue::Set("https://polygon-rpc.com".into()),
        network_id: ActiveValue::Set(Some("137".into())),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
    };

    let chain = chain_am.insert(db).await?;
    Ok(chain)
}

#[tokio::test]
async fn test_find_by_name() -> Result<()> {
    let db = init_test_db().await?;
    let chain = generate_data(&db).await?;

    let chain_found = find_by_name(&db, &chain.name).await?.unwrap();
    assert_eq!(chain_found.id, chain.id);

    Ok(())
}

#[tokio::test]
async fn test_find_by_network_id() -> Result<()> {
    let db = init_test_db().await?;
    let chain = generate_data(&db).await?;

    let chain_found = find_by_network_id(&db, &chain.network_id.unwrap()).await?.unwrap();
    assert_eq!(chain_found.id, chain.id);

    Ok(())
}
