use std::{
    env,
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub cors_allow_origin: String,
    pub jwt_secret: String,
    pub server_host: IpAddr,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Self {
        // let cors_allow_origin = Config::env_var("CORS_ALLOW_ORIGIN");
        let jwt_secret = Config::env_var("JWT_SECRET");
        let server_host =
            Config::env_var_opt("HOST").unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        let server_port = Config::env_var_opt("PORT").unwrap_or(1131);

        Self {
            cors_allow_origin: String::from(""),
            jwt_secret,
            server_host,
            server_port,
        }
    }

    fn env_var<T: FromStr>(key: &str) -> T {
        let value = env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {key}"));

        if let Ok(parsed) = str::parse::<T>(&value) {
            return parsed;
        }

        panic!("Failed to parse environment variable from key: {key}");
    }

    #[allow(dead_code)]
    fn env_var_opt<T: FromStr>(key: &str) -> Option<T> {
        let value = env::var(key).ok()?;

        str::parse::<T>(&value).ok()
    }
}
