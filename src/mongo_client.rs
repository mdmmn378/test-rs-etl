use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};

// works only json, not json list
pub fn convert_serde_to_document(any: serde_json::Value) -> Document {
    let any_str = any.to_string();
    let serde_map =
        serde_json::from_str(&any_str).expect("Failed to convert serde_json::Value to Document");
    let doc = Document::from(serde_map);
    doc
}

pub async fn get_connection(url: &str) -> Client {
    let client_options = ClientOptions::parse(url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client
}

#[allow(dead_code)]
pub async fn get_collection(
    client: Client,
    collection_name: &str,
) -> Result<mongodb::Collection<Document>, mongodb::error::Error> {
    let db = client.database("test");
    let collection = db.collection(collection_name);
    Ok(collection)
}

pub async fn insert_document(
    client: &Client,
    collection_name: &str,
    doc: Document,
) -> Result<(), mongodb::error::Error> {
    let db = client.database("test");
    let collection = db.collection(collection_name);
    collection.insert_one(doc, None).await?;
    Ok(())
}

#[tokio::test]
async fn test_mongo() {
    use mongodb::bson::doc;
    let doc =
        doc! { "name": "MongoDB", "type": "database", "count": 1, "info": { "x": 203, "y": 102 } };
    let client = get_connection("mongodb://localhost:27017").await;
    let result = insert_document(&client, "test", doc).await;
    assert_eq!(result.is_ok(), true);
}

#[tokio::test]
async fn test_convert_any_serde_to_document() {
    use mongodb::bson;
    use mongodb::bson::doc;
    let any = serde_json::json!({"name": "MongoDB", "type": "database", "count": 1, "info": { "x": 203, "y": 102 }});
    let doc = convert_serde_to_document(any);
    assert_eq!(
        doc.get("name").unwrap(),
        &bson::Bson::String("MongoDB".to_string())
    );
    assert_eq!(
        doc.get("type").unwrap(),
        &bson::Bson::String("database".to_string())
    );
    assert_eq!(doc.get("count").unwrap(), &bson::Bson::Int32(1));
    assert_eq!(
        doc.get("info").unwrap(),
        &bson::Bson::Document(doc! { "x": 203, "y": 102 })
    );
    let any = serde_json::json!({"vals": [1, 2, 3]});
    let doc = convert_serde_to_document(any);
    assert_eq!(
        doc.get("vals").unwrap(),
        &bson::Bson::Array(vec![
            bson::Bson::Int32(1),
            bson::Bson::Int32(2),
            bson::Bson::Int32(3)
        ])
    );
}

#[tokio::test]
async fn test_get_connection() {
    let client = get_connection("mongodb://localhost:27017").await;
    let any = serde_json::json!({"name": "MongoDB", "type": "database", "count": 1, "info": { "x": 203, "y": 102 }});
    let doc = convert_serde_to_document(any);
    let collection = get_collection(client, "test").await.unwrap();
    let result = collection.insert_one(doc, None).await;
    assert_eq!(result.is_ok(), true);
}
