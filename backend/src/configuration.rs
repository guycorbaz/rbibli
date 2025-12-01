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
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration(config_path: Option<String>) -> Result<Settings, config::ConfigError> {
    let mut builder = config::Config::builder();
    
    if let Some(path) = config_path {
        builder = builder.add_source(config::File::with_name(&path));
    } else {
        // Check for configuration file in current directory or backend/ directory
        // This allows running from both the workspace root and the backend directory
        if std::path::Path::new("configuration.toml").exists() {
            builder = builder.add_source(config::File::with_name("configuration"));
        } else if std::path::Path::new("backend/configuration.toml").exists() {
            builder = builder.add_source(config::File::with_name("backend/configuration"));
        } else {
            builder = builder.add_source(config::File::with_name("configuration"));
        }
    }

    // Add support for environment variables (e.g. APP_APPLICATION__PORT=5001)
    builder = builder.add_source(
        config::Environment::with_prefix("APP")
            .separator("__")
    );

    let settings = builder.build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_configuration() {
        let config = get_configuration(None);
        assert!(config.is_ok());
        let settings = config.unwrap();
        assert_eq!(settings.application.port, 8000);
        assert_eq!(settings.application.host, "127.0.0.1");
        assert_eq!(settings.database.username, "user");
        assert_eq!(settings.database.password, "password");
        assert_eq!(settings.database.port, 3306);
        assert_eq!(settings.database.host, "127.0.0.1");
        assert_eq!(settings.database.database_name, "rbibli");
    }
}
