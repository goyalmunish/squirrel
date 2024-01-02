mod config;
mod error;
mod utils;
mod web_driver;
mod wf;

/// Squirrel revolutionizes browser automation by simplifying the process
/// through YAML based workflow definition. With Squirrel, you effortlessly
/// automate tasks without getting bogged down by technical intricacies, as
/// the library handles all the underlying complexities for you.
///
/// Here's how you can run:
///
/// ```sh
/// // with default values for all, but first argument
/// cargo run ./src/sample_workflow.yaml
///
/// // with default values (but provided explicitly) for webdriver_url
/// // and headless_browser
/// cargo run ./src/sample_workflow.yaml http://localhost:9515 true
///
/// // directly using build executable
/// ./target/debug/squirrel ./src/sample_workflow.yaml http://localhost:9515 false
///
/// // as IDE run configuration
/// RustRover: `run --package squirrel --bin squirrel -- ./src/sample_workflow.yaml http://localhost:9515 false
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cnf: config::Config = config::parse_args(&args);
    println!("Squirrel executing with configuration: {:#?}", cnf);
    match wf::invoke_workflow(&cnf) {
        Ok(_) => println!("Successfully closed the WebDriver session!"),
        Err(error) => panic!(
            "Error closing the WebDriver session: {error}",
            error = error
        ),
    }
}
