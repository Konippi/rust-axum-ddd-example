use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
    server_port: std::env::var("SERVER_PORT")
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| 8080),
    database_url: std::env::var("DATABASE_URL").unwrap(),
    otlp_exporter_endpoint: std::env::var("OTLP_EXPORTER_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:4317".to_string()),
});

pub struct Config {
    pub server_host: String,
    pub server_port: u32,
    pub database_url: String,
    pub otlp_exporter_endpoint: String,
}

impl Config {
    pub fn init() {
        dotenv::from_filename(format!(
            ".env.{}",
            std::env::var("SERVER_ENV").unwrap_or_else(|_| "local".to_string())
        ))
        .expect("Failed to load .env file");
    }
}
