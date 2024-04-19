// A model is a struct that represents a collection in no-sql database(It is the equivalent of a table in a relational database).
// The model is used to interact with the database. 

use serde::{Serialize, Deserialize};

// Define a user document model to Test the connection to the database
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub age: u32,
}
