//! Loads the smart embedding pack and reports why it fails, if it does.
//!
//!   cargo run -p kensaku --example smart_probe --features smart-embeddings -- <models/smart>
fn main() {
    let dir = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .expect("usage: smart_probe <pack-dir>");

    println!("pack dir: {}", dir.display());
    for name in ["model.onnx", "tokenizer.json", "config.json"] {
        let path = dir.join(name);
        println!(
            "  {name}: {} ({} bytes)",
            path.is_file(),
            std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
        );
    }

    let config = kensaku_lib::config::KensakuConfig::default();
    let registry = kensaku_lib::embed::ModelRegistry::from_config_and_dir(&config, &dir);
    println!(
        "default config -> provider {:?}, model_id {}, name {}, fallback {:?}",
        registry.provider, registry.model_id, registry.name, registry.fallback_reason
    );

    let mut config = kensaku_lib::config::KensakuConfig::default();
    config.semantic_enabled = true;
    config.semantic_model = "smart".to_string();
    let registry = kensaku_lib::embed::ModelRegistry::from_config_and_dir(&config, &dir);
    println!(
        "smart config   -> provider {:?}, model_id {}, name {}, fallback {:?}",
        registry.provider, registry.model_id, registry.name, registry.fallback_reason
    );

    match kensaku_lib::embed::smart::SmartEmbedder::load(&dir) {
        Some(embedder) => {
            let vector = embedder.embed_f32("quarterly sales by region");
            println!(
                "SmartEmbedder::load -> ok, dim={}, vector_len={}",
                embedder.dim,
                vector.len()
            );
        }
        None => println!("SmartEmbedder::load -> None"),
    }
}
