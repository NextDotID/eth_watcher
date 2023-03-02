use entities::chain::ActiveModel;
use sea_orm::ActiveModelTrait;

use crate::model::tests::init_test_db;

use super::*;

async fn generate_data(db: &DatabaseConnection) -> Result<ChainModel> {
    let chain_am = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set("polygon".into()),
        rpc: ActiveValue::Set("https://polygon-rpc.com".into()),
        chain_id: ActiveValue::Set(Some("137".into())),
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
async fn test_find_by_chain_id() -> Result<()> {
    let db = init_test_db().await?;
    let chain = generate_data(&db).await?;

    let chain_found = find_by_chain_id(&db, &chain.chain_id.unwrap()).await?.unwrap();
    assert_eq!(chain_found.id, chain.id);

    Ok(())
}

#[tokio::test]
async fn test_update_chain_id() -> Result<()> {
    let db = init_test_db().await?;
    let chain_am = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set("polygon".into()),
        rpc: ActiveValue::Set("https://polygon-rpc.com".into()),
        chain_id: ActiveValue::Set(None),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
    };

    let chain = chain_am.insert(&db).await?;
    let chain_updated = update_chain_id(&db, chain).await?;
    assert_eq!(chain_updated.name, "polygon".to_string());
    assert_eq!(chain_updated.chain_id, Some("137".to_string()));

    Ok(())
}
