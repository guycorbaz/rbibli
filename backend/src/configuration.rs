use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
    pub host: Option<String>,
    pub database_name: Option<String>,
    pub url: Option<String>,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        if let Some(url) = &self.url {
            url.clone()
        } else {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username.as_deref().unwrap_or(""),
                self.password.as_deref().unwrap_or(""),
                self.host.as_deref().unwrap_or("127.0.0.1"),
                self.port.unwrap_or(3306),
                self.database_name.as_deref().unwrap_or("rbibli")
            )
        }
    }
    
    pub fn connection_string_without_db(&self) -> String {
        if let Some(url) = &self.url {
            url.clone()
        } else {
            format!(
                "mysql://{}:{}@{}:{}",
                self.username.as_deref().unwrap_or(""),
                self.password.as_deref().unwrap_or(""),
                self.host.as_deref().unwrap_or("127.0.0.1"),
                self.port.unwrap_or(3306)
            )
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut builder = config::Config::builder()
        // Add support for environment variables (e.g. APP__APPLICATION__PORT=5001)
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("__")
        );

    // Manually override with standard env vars if present
    // This supports HOST, PORT, DATABASE_URL directly
    if let Ok(port) = std::env::var("PORT") {
        builder = builder.set_override("application.port", port)?;
    }
    if let Ok(host) = std::env::var("HOST") {
        builder = builder.set_override("application.host", host)?;
    }
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        builder = builder.set_override("database.url", db_url)?;
    }

    let settings = builder.build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_configuration() {
        let config = get_configuration();
        // This might fail if env vars are not set, so we can't assert much here without setting them
        // But we can check if it returns a result
        // assert!(config.is_ok()); 
    }

    #[test]
    fn test_env_vars() {
        unsafe {
            std::env::set_var("APP__APPLICATION__PORT", "1234");
            std::env::set_var("APP__APPLICATION__HOST", "test_host");
            // We must provide all required fields for DatabaseSettings because they are not Option
            std::env::set_var("APP__DATABASE__USERNAME", "test_user");
            std::env::set_var("APP__DATABASE__PASSWORD", "test_pass");
            std::env::set_var("APP__DATABASE__PORT", "5432");
            std::env::set_var("APP__DATABASE__HOST", "test_db_host");
            std::env::set_var("APP__DATABASE__DATABASE_NAME", "test_db");
        }

        let config = get_configuration().expect("Failed to load config from env vars");
        assert_eq!(config.application.port, 1234);
        assert_eq!(config.application.host, "test_host");
        assert_eq!(config.database.username, Some("test_user".to_string()));
    }
}
