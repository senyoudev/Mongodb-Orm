use mongodb::error::Result;

use crate::connect;


pub async fn create_collection(db_name:&str,collection_name: &str,conn_str:String) -> Result<()> {
    // String to &str
    let client = connect(&conn_str).await?;
    let db = client.database(db_name);
    let collection = db.create_collection(collection_name, None).await?;
    println!("Collection created successfully , here is it{:?}", collection);
    Ok(())
}