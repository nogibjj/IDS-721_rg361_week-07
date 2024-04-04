use anyhow::{Result, anyhow};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    CreateCollection, SearchPoints, VectorParams, VectorsConfig,
};
use qdrant_client::qdrant::vectors_config::Config;
use rand::Rng;
use serde_json::json;
use std::convert::TryInto;

#[tokio::main]
async fn main() -> Result<()> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config))?;

    let collection_name = "data_table";
    let _ = client.delete_collection(collection_name).await;

    client.create_collection(&CreateCollection {
        collection_name: collection_name.into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 2, // Adjust this to match the size of your vectors
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }).await?;

    // Initialize a random number generator
    let mut rng = rand::thread_rng();

    // Generate 20 random vectors and insert them into the database
    for i in 0..20 {
        let vector: [f32; 2] = [rng.gen::<f32>(), rng.gen::<f32>()];
        let number = rng.gen_range(1..101); // Randomly generated integer between 1 and 100
        let payload: Payload = json!({
            "number": number,
            "vector": vector
        }).try_into().map_err(|e| anyhow!("Payload conversion error: {:?}", e))?;

        let points = vec![PointStruct::new(i as u64, vector.to_vec(), payload)];
        client.upsert_points_blocking(collection_name, None, points, None).await?;

        println!("Generated vector {}: {:?} with number {}", i+1, vector, number);
    }

    // Generate a new 2D vector and query the 3 closest vectors from the database
    let query_vector: [f32; 2] = [rng.gen::<f32>(), rng.gen::<f32>()];
    println!("Search vector: {:?}", query_vector);

    let search_result = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: query_vector.to_vec(),
        filter: None,
        limit: 3,
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;

    // Visualize the output for each found point
    for (index, point) in search_result.result.iter().enumerate() {
        let payload = &point.payload;
        let vector = payload.get("vector")
            .map(|v| v.to_string())
            .ok_or(anyhow!("Failed to extract vector from payload"))?;
        let number = payload.get("number")
            .map(|v| v.to_string())
            .ok_or(anyhow!("Failed to extract number from payload"))?;
        println!("Result {}, Vector: {}, Number: {}", index + 1, vector, number);
    }

    Ok(())
}