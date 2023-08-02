use super::TodoMac;
use crate::model::db::init_db;
use crate::model::todo::TodoPatch;

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let data_fx = TodoPatch {
        title: Some("test - model_create_todo".to_string()),
        ..Default::default()
    };

    let todo_created = TodoMac::create(&db, data_fx.clone()).await?;

    println!("\n\n ->> {:?}", todo_created);

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let todos = TodoMac::list(&db).await?;
    assert_eq!(2, todos.len());
    println!("\n\n ->> {:?}", todos);
    Ok(())
}
