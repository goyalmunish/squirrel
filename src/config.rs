/// Config struct providing all passed and environmental
/// configurations required for the program run.
#[derive(Debug)]
pub struct Config {
    pub workflow_file_path: String,
    pub headless_browser: bool,
    pub webdriver_port: usize,
    pub temp_dir: String,
}

pub const HEADLESS_BROWSER_DEFAULT: bool = true;
pub const WEBDRIVER_PORT_DEFAULT: usize = 9515;
pub const TEMP_DIR_DEFAULT: &str = "temp/";
/// TAB_SIZE defines the printing indentation.
pub const TAB_SIZE: usize = 4;
/// Parse args to construct and return a Config.
pub fn parse_args(args: &Vec<String>) -> Config {
    let workflow_file_path = match args.get(1) {
        None => panic!("Provide configuration file (.yaml) path as the first argument!"),
        Some(v) => v.clone(),
    };
    let headless_browser = match args.get(2) {
        None => HEADLESS_BROWSER_DEFAULT,
        Some(elem) => match elem.trim().parse() {
            Err(_) => HEADLESS_BROWSER_DEFAULT,
            Ok(v) => v,
        },
    };
    return Config {
        workflow_file_path,
        headless_browser,
        webdriver_port: WEBDRIVER_PORT_DEFAULT,
        temp_dir: TEMP_DIR_DEFAULT.to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_with_workflow_path() {
        let args = vec![
            String::from("program_name"),
            String::from("./sample_workflow.yaml"),
        ];
        let config = parse_args(&args);

        assert_eq!(config.workflow_file_path, "./sample_workflow.yaml");
        assert_eq!(config.headless_browser, HEADLESS_BROWSER_DEFAULT);
        assert_eq!(config.webdriver_port, WEBDRIVER_PORT_DEFAULT);
        assert_eq!(config.temp_dir, TEMP_DIR_DEFAULT.to_string());
    }

    #[test]
    fn parse_args_with_all_arguments() {
        let args = vec![
            String::from("program_name"),
            String::from("./sample_workflow.yaml"),
            String::from("false"),
        ];
        let config = parse_args(&args);

        assert_eq!(config.workflow_file_path, "./sample_workflow.yaml");
        assert_eq!(config.headless_browser, false);
        assert_eq!(config.webdriver_port, WEBDRIVER_PORT_DEFAULT);
        assert_eq!(config.temp_dir, TEMP_DIR_DEFAULT.to_string());
    }

    #[test]
    #[should_panic(expected = "Provide configuration file (.yaml) path as the first argument!")]
    fn parse_args_with_missing_workflow_path() {
        let args = vec![String::from("program_name")];
        parse_args(&args);
    }
}
