use crate::config;

/// Returns web driver client capabilities.
pub fn client_capabilities(config: &config::Config) -> serde_json::Map<String, serde_json::Value> {
    let mut browser_args: Vec<String> = Vec::new();
    if config.headless_browser {
        browser_args.push("--headless".to_string());
    }
    for browser_arg in config.browser_args.clone() {
        browser_args.push(browser_arg);
    }

    let options = serde_json::json!({ "args": browser_args });

    let mut capabilities = serde_json::map::Map::new();
    capabilities.insert("goog:chromeOptions".to_string(), options);

    return capabilities;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    fn client_capabilities_with_headless_browser() {
        // Create a Config with headless_browser set to true
        let config = config::Config {
            workflow_file_path: String::from("config.yaml"),
            headless_browser: true,
            webdriver_url: config::WEBDRIVER_URL_DEFAULT.to_string(),
            browser_args: vec!["".to_string()],
            temp_dir: String::from(config::TEMP_DIR_DEFAULT),
        };

        let capabilities = client_capabilities(&config);

        // Check if the capabilities include "headless" argument
        assert!(capabilities.contains_key("goog:chromeOptions"));
        let options = capabilities["goog:chromeOptions"].as_object().unwrap();
        let args = options["args"].as_array().unwrap();
        assert!(args.contains(&serde_json::Value::String(String::from("--headless"))));
    }

    #[test]
    fn client_capabilities_with_browser_args() {
        // Create a Config with headless_browser set to true
        let config = config::Config {
            workflow_file_path: String::from("config.yaml"),
            headless_browser: true,
            browser_args: vec![
                "--no-sandbox".to_string(),
                "--disable-dev-shm-usage".to_string(),
                "--disable-popup-blocking".to_string(),
                "--disable-gpu".to_string(),
                "--blink-settings=imagesEnabled=false".to_string(),
            ],
            webdriver_url: config::WEBDRIVER_URL_DEFAULT.to_string(),
            temp_dir: String::from(config::TEMP_DIR_DEFAULT),
        };

        let capabilities = client_capabilities(&config);

        // Check if the capabilities include "headless" argument
        assert!(capabilities.contains_key("goog:chromeOptions"));
        let options = capabilities["goog:chromeOptions"].as_object().unwrap();
        let args = options["args"].as_array().unwrap();
        println!("{:#?}", args);
        assert!(args.contains(&serde_json::Value::String(String::from("--headless"))));
        assert!(args.contains(&serde_json::Value::String(String::from("--no-sandbox"))));
        assert!(args.contains(&serde_json::Value::String(String::from(
            "--disable-dev-shm-usage"
        ))));
        assert!(args.contains(&serde_json::Value::String(String::from(
            "--disable-popup-blocking"
        ))));
        assert!(args.contains(&serde_json::Value::String(String::from("--disable-gpu"))));
    }

    #[test]
    fn client_capabilities_without_headless_browser() {
        // Create a Config with headless_browser set to false
        let config = config::Config {
            workflow_file_path: String::from("config.yaml"),
            headless_browser: false,
            browser_args: vec!["".to_string()],
            webdriver_url: config::WEBDRIVER_URL_DEFAULT.to_string(),
            temp_dir: String::from(config::TEMP_DIR_DEFAULT),
        };

        let capabilities = client_capabilities(&config);

        // Check if the capabilities do not include "headless" argument
        assert!(capabilities.contains_key("goog:chromeOptions"));
        let options = capabilities["goog:chromeOptions"].as_object().unwrap();
        let args = options["args"].as_array().unwrap();
        assert!(!args.contains(&serde_json::Value::String(String::from("--headless"))));
    }
}
