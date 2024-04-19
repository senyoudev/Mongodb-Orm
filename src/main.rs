use std::env;
use dotenv::dotenv;
use ORM_RUST::db::connect;
use ORM_RUST::create_collection::create_collection;

fn main() {
    // load env vars
    dotenv().ok();

    // get the connection string from the environment
    let conn_str = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // create a collection
    let collection_name = "users";
    let schema = r#"
        {
            "username": "string",
            "email": "string",
            "age": "number"
        }
    "#;
    create_collection(collection_name, conn_str);
    println!("Collection created successfully");

    
}
