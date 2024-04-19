// A model is a struct that represents a collection in no-sql database(It is the equivalent of a table in a relational database).
// The model is used to interact with the database. 

#[derive(Debug, Serialize, Deserialize)]
struct Document<T> {
    data: T,
}