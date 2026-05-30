/// Cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na < 1e-8 || nb < 1e-8 {
        0.0
    } else {
        dot / (na * nb)
    }
}

/// Encode vector as compact little-endian f32 bytes.
pub fn encode_vector(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}

/// Decode little-endian f32 bytes back to vector.
pub fn decode_vector(data: &[u8]) -> Vec<f32> {
    data.chunks(4)
        .map(|c| {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(c);
            f32::from_le_bytes(buf)
        })
        .collect()
}

/// Call OpenAI embedding API to get a vector for text.
pub async fn embed_openai(text: &str, api_key: &str) -> Result<Vec<f32>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.openai.com/v1/embeddings")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&serde_json::json!({"input": text, "model": "text-embedding-3-small"}))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("API error: {}", resp.status()));
    }

    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    body["data"][0]["embedding"]
        .as_array()
        .ok_or("no embedding".into())
        .map(|arr| {
            arr.iter()
                .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                .collect()
        })
}

/// Auto-detect provider and get embedding for text.
pub async fn get_embedding(text: &str) -> Result<Vec<f32>, String> {
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            return embed_openai(text, &key).await;
        }
    }
    Err("Set OPENAI_API_KEY for embedding support".into())
}
