mod workflow;
mod workflow_step;

use crate::{config, error, utils, web_driver};

// The `invoke_workflow` is an async function marked to be executed by
// tokio::main runtime, which makes it behave as if it was synchronous
// by starting a new runtime each time it is called.
#[tokio::main]
pub async fn invoke_workflow(config: &config::Config) -> Result<(), fantoccini::error::CmdError> {
    // Get workflow object
    let wf = match construct_workflow(config) {
        Ok(value) => value,
        Err(error) => {
            return Err(fantoccini::error::CmdError::NotJson(format!("{error}")));
        }
    };

    // Construct webdriver client
    let capabilities = web_driver::client_capabilities(config);
    println!("Using capabilities: {:#?}", capabilities);
    let conn_webdriver = match fantoccini::ClientBuilder::native()
        .capabilities(capabilities)
        .connect(format!("http://0.0.0.0:{}", config.webdriver_port).as_str())
        .await
    {
        Ok(value) => value,
        Err(error) => {
            return Err(fantoccini::error::CmdError::NotJson(format!(
                "Failed establishing the webdriver connection: {error}"
            )));
        }
    };

    // The `current_elements` would hold elements found in the search
    let mut current_elements: Vec<fantoccini::elements::Element> = Vec::new();
    // The `current_value` would hold the values the users is interested
    // to be provided with
    let mut current_values: Vec<String> = Vec::new();
    // The `depth` represents the depth of the call stack
    let depth: usize = 0;

    // Execute all steps
    for (index, step) in wf.steps.iter().enumerate() {
        println!(
            "Step {index}: {} (timestamp={})",
            step.to_string(),
            utils::timestamp()
        );
        // In case of an error, stop gracefully
        match step
            .execute(
                config,
                &conn_webdriver,
                &mut current_elements,
                &mut current_values,
                depth,
            )
            .await
        {
            Ok(_) => {
                // Step ran successfully!
            }
            Err(error) => {
                println! {"Workflow failed with: {error}"};
                // skipp subsequent steps
                break;
            }
        };
    }
    // Explicitly close webdriver client
    // Although, this is not required as long as the client implements the Drop trait
    // in which case it will be automatically dropped after going out of scope at the
    // end of this function; provided, the function doesn't panic once the client has
    // been created.
    conn_webdriver.close().await
}

fn construct_workflow(config: &config::Config) -> Result<workflow::Workflow, error::CustomError> {
    // Read workflow steps
    let wf = match std::fs::read_to_string(&config.workflow_file_path) {
        Ok(value) => value,
        Err(error) => {
            // converting std::io::Error to CustomError
            return Err(error::CustomError {
                message: format!("{error}"),
            });
        }
    };
    // Parse workflow steps
    let wf: workflow::Workflow = match serde_yaml::from_str(&wf) {
        Ok(value) => value,
        Err(error) => {
            // converting serde_yaml::Error to CustomError
            return Err(error::CustomError {
                message: format!("{error}"),
            });
        }
    };
    return Ok(wf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    const TEST_WORKFLOW_CONTENT: &str = r#"
          name: "sample workflow"
          steps:
              - !PageOpen "https://www.wikipedia.org/"
              - !PageLocateElements
                - "body div"
                - "all"
                - 0
              - !PageLocateElements
                - "body h1 strong"
                - "index"
                - 0
              - !ElementsLoopThrough
                - !ElementSaveHtmlValue true
                - !ElementTakeScreenshot "separate"
                - !ElementPop
                - !PageWait 100
              - !PrintCurrentValues
              - !PageScroll
                - "full"
                - 1.0
              - !PageWait 5000
              - !PageTakeScreenshot "page_stackoverflow_home"
    "#;

    #[test]
    fn construct_workflow_success() {
        // Create a temporary file with valid workflow content
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let workflow_path = temp_dir.path().join("test_workflow.yaml");
        std::fs::write(&workflow_path, TEST_WORKFLOW_CONTENT)
            .expect("Failed to write workflow file");

        // Create a Config with the temporary workflow file
        let config = config::Config {
            workflow_file_path: workflow_path.to_string_lossy().to_string(),
            headless_browser: true,
            webdriver_port: config::WEBDRIVER_PORT_DEFAULT,
            temp_dir: String::from(config::TEMP_DIR_DEFAULT),
        };

        // Perform the workflow construction
        let result = construct_workflow(&config);

        // Check if the workflow is constructed successfully
        assert!(result.is_ok());

        // Clean up: Delete the temporary workflow file
        std::fs::remove_file(&workflow_path).expect("Failed to delete temporary workflow file");
    }

    #[test]
    fn construct_workflow_with_invalid_file() {
        // Create a Config with a nonexistent workflow file
        let config = config::Config {
            workflow_file_path: String::from("/nonexistent/file.yaml"),
            headless_browser: true,
            webdriver_port: config::WEBDRIVER_PORT_DEFAULT,
            temp_dir: String::from(config::TEMP_DIR_DEFAULT),
        };

        // Perform the workflow construction
        let result = construct_workflow(&config);

        // Check if the workflow construction fails as expected
        assert!(result.is_err());
    }
}
