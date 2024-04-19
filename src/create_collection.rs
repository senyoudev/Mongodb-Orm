use mongodb::error::Result;

use crate::connect;


pub async fn create_collection(collection_name: &str,conn_str:String) -> Result<()> {
    // String to &str
    let conn_str = &conn_str;
    let client = connect(conn_str).await?;
    let db = client.database("users");
    db.create_collection(collection_name, None).await?;
    Ok(())
}