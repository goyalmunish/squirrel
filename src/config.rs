/// `Config` struct providing all passed and environmental
/// configurations required for the program run.
#[derive(Debug)]
pub struct Config {
    pub workflow_file_path: String,
    pub webdriver_url: String,
    pub headless_browser: bool,
    pub browser_args: Vec<String>,
    pub temp_dir: String,
}

pub const WEBDRIVER_URL_DEFAULT: &str = "http://localhost:9515";
pub const HEADLESS_BROWSER_DEFAULT: bool = true;
pub const BROWSER_ARGS_DEFAULT: &str = "";
pub const TEMP_DIR_DEFAULT: &str = "temp/";
/// `TAB_SIZE` defines the printing indentation.
pub const TAB_SIZE: usize = 4;
pub const WINDOW_WIDTH_DEFAULT: u32 = 1200;
pub const WINDOW_HEIGHT_DEFAULT: u32 = 1293;
pub const REMOTE_WAIT_FACTOR: f64 = 0.1;
pub const DEBUG_MODE: bool = false;
/// Parse args to construct and return a Config.
pub fn parse_args(args: &Vec<String>) -> Config {
    let workflow_file_path = match args.get(1) {
        None => panic!("Provide configuration file (.yaml) path as the first argument!"),
        Some(v) => v.clone(),
    };
    let webdriver_url = match args.get(2) {
        None => WEBDRIVER_URL_DEFAULT.to_string(),
        Some(v) => v.clone(),
    };
    let headless_browser = match args.get(3) {
        None => HEADLESS_BROWSER_DEFAULT,
        Some(elem) => elem.trim().parse().unwrap_or(HEADLESS_BROWSER_DEFAULT),
    };
    let browser_args = args
        .get(4)
        .unwrap_or(&BROWSER_ARGS_DEFAULT.to_string())
        .clone();
    let browser_args = browser_args
        .split_ascii_whitespace()
        .map(|c| c.to_string())
        .collect();

    return Config {
        workflow_file_path,
        webdriver_url,
        headless_browser,
        browser_args,
        temp_dir: TEMP_DIR_DEFAULT.to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Provide configuration file (.yaml) path as the first argument!")]
    fn parse_args_with_missing_workflow_path() {
        let args = vec![String::from("program_name")];
        parse_args(&args);
    }

    #[test]
    fn parse_args_with_workflow_path() {
        let args = vec![
            String::from("program_name"),
            String::from("./sample_workflow.yaml"),
        ];
        let config = parse_args(&args);

        assert_eq!(config.workflow_file_path, "./sample_workflow.yaml");
        assert_eq!(config.headless_browser, HEADLESS_BROWSER_DEFAULT);
        assert_eq!(config.webdriver_url, WEBDRIVER_URL_DEFAULT);
        assert_eq!(config.temp_dir, TEMP_DIR_DEFAULT.to_string());
    }

    #[test]
    fn parse_args_with_all_arguments() {
        let args = vec![
            String::from("program_name"),
            String::from("./sample_workflow.yaml"),
            String::from("http://localhost:9515"),
            String::from("false"),
        ];
        let config = parse_args(&args);

        assert_eq!(config.workflow_file_path, "./sample_workflow.yaml");
        assert_eq!(config.headless_browser, false);
        assert_eq!(config.webdriver_url, WEBDRIVER_URL_DEFAULT);
        assert_eq!(config.temp_dir, TEMP_DIR_DEFAULT.to_string());
    }
}
