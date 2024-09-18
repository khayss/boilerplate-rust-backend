pub struct Configs {
    pub db_url: String,
    pub ether_api_url: String,
}

impl Configs {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let ether_api_url = dotenv::var("ETHER_API_URL").expect("ETHER_API_URL must be set");
        Configs {
            db_url,
            ether_api_url,
        }
    }
}
