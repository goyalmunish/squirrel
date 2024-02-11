use crate::{config, utils};

/// `WorkflowStep` enum defines individual step of `Workflow`.
/// Its variants define supported operations.
#[derive(serde::Serialize, serde::Deserialize, Debug, strum_macros::Display)]
pub enum WorkflowStep {
    /// Find elements on current page.
    ///
    /// Use given css as selector.
    /// If running in "all" mode, search all elements, ignoring the index.
    /// If running in "index" mode, then only select the element at given index.
    /// If no matching element is found, return an error.
    ///
    /// Logic: Calling this means that there is going to be a beginning of new loop
    /// using `ElementsLoopThrough`. So,
    /// - New `current_elements` will be populated and appended to `current_elements_stack`
    ///
    /// Arguments: `css` (`String`), `mode` ("all"/"index") (`String`), `index` (`usize`)
    PageLocateElements(String, String, usize),
    /// Loop through latest set of current_elements (from current_elements_set).
    ///
    /// Logic: Calling this means, we wish to iterate over recently set of `current_elements`
    /// from `current_elements_stack`. Iterate through collection at the top of the stack.
    /// Also, remove the collection from top of the stack, once it's empty.
    /// Note: In each iteration from the collection on the top of the stack, the element
    /// should be removed using `ElementPop`.
    ///
    /// Arguments: `sub_steps` (`Vec<WorkflowStep>`)
    ElementsLoopThrough(Vec<WorkflowStep>),
    /// Run an infinite loop, with given sub-steps. Break the loop if any of
    /// its sub-steps returns error (such as "NEXT" button is not more active).
    ///
    /// For example, this is helpful in the conditions where you want to
    /// keep clicking on "Next" button in pagination, until all pages are
    /// exhausted and so the "Next" button couldn't be found.
    ///
    /// Logic: Within every PageLoop, one of the first step is to look for
    /// certain elements to process, and so there is always expected a
    /// PageLocateElement somewhere at the beginning of this loop.
    /// Also, this elements itself doesn't touch any of the saved values
    /// such as current_elements_stack, current_values.
    /// These will be handled by `PageLocateElement` as its sub-step.
    ///
    /// Arguments: `sub_steps` (`Vec<WorkflowStep>`)
    PageLoop(Vec<WorkflowStep>),
    /// Remove current element from the currently selected page elements.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Note: It is mandatory last sub-step for sub steps under `ElementsLoopThrough`.
    ElementPop,
    /// Save HTML value of the currently selected element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Arguments: `name` (`String`), `is_inner` (`bool`)
    ElementSaveHtmlValue(String, bool),
    /// Open given url.
    ///
    /// Arguments: `url` (`String`)
    PageOpen(String),
    /// Refresh the current page.
    PageRefresh,
    /// Perform Go Back operation within the same window/tab.
    PageBack,
    /// Close the current window and go back to previous window.
    ///
    /// Note: ElementClickNewWindow always needs to be paired with PageBackWindow.
    PageBackWindow,
    /// Scroll current page.
    ///
    /// Refer CSS Selectors: https://www.w3schools.com/cssref/css_selectors.php
    /// If running in "page" mode, provide desired `page_size`.
    /// If running in "full" mode, provide `page_size` as 1.0 for scroll to
    /// the bottom, or `page_size` as -1.0 to scroll to the top of the page.
    ///
    /// Arguments: `mode` (`String`), `page_size` (`f64`)
    PageScroll(String, f64),
    /// Take screenshot of current page.
    ///
    /// Arguments: `file_prefix` (`String`)
    PageTakeScreenshot(String),
    /// Wait for given milliseconds.
    ///
    /// Arguments: `duration_ms` (`String`)
    PageWait(u64),
    /// Click the currently selected page element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    /// If `check_enabled` is `true`, then it raises error if the element is not enabled.
    /// If `check_url` is `true`, then it raised error if the URL of the current page didn't change.
    /// Arguments: `check_enabled` (`bool`), `check_url` (`bool`)
    ElementClick(bool, bool),
    /// Click the currently selected page element (must be `a`) and open it in new browser tab.
    ///
    /// Use URL of currently selected element and open it in new window.
    /// Note: ElementClickNewWindow always needs to be paired with PageBackWindow.
    ElementClickNewWindow,
    /// Send keys to currently selected page element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Arguments: `keys` (`String`)
    ElementSendKeys(String),
    /// Take screenshot of the current element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Arguments: `file_prefix` (`String`)
    ElementTakeScreenshot(String),
    PrintCurrentValues,
}

impl WorkflowStep {
    /// Execute a WorkflowStep
    #[async_recursion::async_recursion]
    pub async fn execute(
        &self,
        config: &config::Config,
        conn_webdriver: &fantoccini::Client,
        current_elements_stack: &mut Vec<Vec<fantoccini::elements::Element>>, // whole current_elements stack
        current_values: &mut Vec<String>, // whole current_values stack
        depth: usize,
        wf_name: &String,
    ) -> Result<(), fantoccini::error::CmdError> {
        let depth = depth + 1;
        match self {
            WorkflowStep::PageLocateElements(css, mode, index) => {
                // Let's create new set of current_elements from found elements.
                let mut elements = conn_webdriver
                    .find_all(fantoccini::Locator::Css(css))
                    .await?;
                if mode == "all" {
                    current_elements_stack.push(elements);
                } else if mode == "index" {
                    // running in "index" mode, so index value must be provided
                    if elements.len() == 0 {
                        // No element could be found with given selector.
                        // Must raise error to signal the loop (if running)
                        // in loop.
                        return Err(fantoccini::error::CmdError::NotJson(
                            "CustomError: Element not found!".to_string(),
                        ));
                    }
                    let mut current_elements_local = vec![];
                    current_elements_local.push(elements.remove(*index));
                    current_elements_stack.push(current_elements_local);
                } else {
                    println!(
                        "{:>width$}Incorrect arguments!",
                        "",
                        width = depth * config::TAB_SIZE
                    )
                }
                let current_elements_len = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!")
                    .len();
                println!(
                    "{:>width$}Current Elements Stack Size: {}, Current Elements Size: {}",
                    "",
                    current_elements_stack.len(),
                    current_elements_len,
                    width = depth * config::TAB_SIZE
                );
                if config::DEBUG_MODE {
                    println!(
                        "{:>width$}[DEBUG] Current Elements Stack: {:?}",
                        "",
                        current_elements_stack,
                        width = depth * config::TAB_SIZE
                    );
                }
                Ok(())
            }
            WorkflowStep::ElementsLoopThrough(sub_steps) => {
                // Get reference to current elements
                let mut current_elements_len = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!")
                    .len();
                // Start the loop
                while current_elements_len > 0 {
                    let index_elem = current_elements_len - 1;
                    println!(
                        "{:>width$}Element index {index_elem}:",
                        "",
                        width = depth * config::TAB_SIZE
                    );
                    for (index_sub_step, sub_step) in sub_steps.iter().enumerate() {
                        println!(
                            "{:>width$}SubStep {index_sub_step}: {} (timestamp={})",
                            "",
                            sub_step.to_string(),
                            utils::timestamp(),
                            width = (depth + 1) * config::TAB_SIZE,
                        );
                        sub_step
                            .execute(
                                config,
                                conn_webdriver,
                                current_elements_stack,
                                current_values,
                                depth + 1,
                                wf_name,
                            )
                            .await?;
                    }
                    current_elements_len -= 1;
                }
                // The top of the stack is now empty; remove it
                current_elements_stack.pop();
                Ok(())
            }
            WorkflowStep::PageLoop(sub_steps) => {
                let mut index_loop = 0;
                // Run infinite loop until a subcommand fails, in which case
                // exist out of this whole workflow step.
                loop {
                    println!(
                        "{:>width$}Loop No. {index_loop}",
                        "",
                        width = depth * config::TAB_SIZE
                    );
                    index_loop += 1;
                    for (index_sub_step, sub_step) in sub_steps.iter().enumerate() {
                        println!(
                            "{:>width$}SubStep {index_sub_step}: {} (timestamp={})",
                            "",
                            sub_step.to_string(),
                            utils::timestamp(),
                            width = (depth + 1) * config::TAB_SIZE,
                        );
                        // Note that if the sub-step raised an error (such as no
                        // longer able to find "Next" element in pagination
                        // while keep on clicking it), it is signal that this
                        // infinite loop must be stopped.
                        let result = sub_step
                            .execute(
                                config,
                                conn_webdriver,
                                current_elements_stack,
                                current_values,
                                depth + 1,
                                wf_name,
                            )
                            .await;
                        match result {
                            Err(error) => {
                                // time to end the loop
                                println!(
                                    "{:>width$}Ending the infinite loop due to error: {error}",
                                    "",
                                    width = (depth + 2) * config::TAB_SIZE
                                );
                                return Ok(());
                            }
                            Ok(_) => {
                                // all good
                            }
                        }
                    }
                }
                // this is unreachable code
            }
            WorkflowStep::ElementPop => {
                // Get reference to current elements
                let current_elements_stack_len = current_elements_stack.len();
                let current_elements = current_elements_stack
                    .get_mut(current_elements_stack_len - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!");

                if config::DEBUG_MODE {
                    println!(
                        "{:>width$}[DEBUG] Current Elements Size (before): {}",
                        "",
                        current_elements.len(),
                        width = depth * config::TAB_SIZE
                    );
                    println!(
                        "{:>width$}[DEBUG] Current Elements: {:?}",
                        "",
                        current_elements,
                        width = depth * config::TAB_SIZE
                    );
                }
                current_elements.pop();
                if config::DEBUG_MODE {
                    println!(
                        "{:>width$}[DEBUG] Current Elements Size (after): {}",
                        "",
                        current_elements.len(),
                        width = depth * config::TAB_SIZE
                    );
                    println!(
                        "{:>width$}[DEBUG] Current Elements: {:?}",
                        "",
                        current_elements,
                        width = depth * config::TAB_SIZE
                    );
                }
                Ok(())
            }
            WorkflowStep::ElementSaveHtmlValue(name, is_inner) => {
                // Get reference to the current element
                let current_elements = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!");
                let current_elem = current_elements
                    .get(current_elements.len() - 1)
                    .expect("Workflow Definition Error: `current_elements` is empty!");
                // Get its HTML value
                let elem_html = current_elem.html(*is_inner).await?;
                let elem_value = format!("{name}::{elem_html}");
                println!(
                    "{:>width$}HTML Value: {}",
                    "",
                    elem_value,
                    width = depth * config::TAB_SIZE
                );
                // Save element's value
                current_values.push(elem_value);
                Ok(())
            }
            WorkflowStep::PageOpen(url) => {
                conn_webdriver
                    .set_window_size(config::WINDOW_WIDTH_DEFAULT, config::WINDOW_HEIGHT_DEFAULT)
                    .await?;
                conn_webdriver.goto(url).await?;
                let size = conn_webdriver.get_window_size().await?;
                println!(
                    "{:>width$}Window Size: {:?}",
                    "",
                    size,
                    width = depth * config::TAB_SIZE
                );
                Ok(())
            }
            WorkflowStep::PageRefresh => {
                conn_webdriver.refresh().await?;
                let size = conn_webdriver.get_window_size().await?;
                println!(
                    "{:>width$}Window Size: {:?}",
                    "",
                    size,
                    width = depth * config::TAB_SIZE
                );
                Ok(())
            }
            WorkflowStep::PageBack => {
                conn_webdriver.back().await?;
                Ok(())
            }
            WorkflowStep::PageBackWindow => {
                conn_webdriver.close_window().await?;
                // Note: ElementClickNewWindow always needs to be paired with PageBackWindow
                let prev_window = conn_webdriver.windows().await?.pop().ok_or(
                    fantoccini::error::CmdError::NotJson(
                        "ElementClickNewWindow always needs to be paired with PageBackWindow"
                            .to_string(),
                    ),
                )?;
                conn_webdriver.switch_to_window(prev_window).await?;
                Ok(())
            }

            WorkflowStep::PageScroll(mode, page_size) => {
                // Note: Not making use of mode yet
                let mut arguments = Vec::new();
                let argument = serde_json::json!(page_size);
                arguments.push(argument);
                if (mode == "full") && (*page_size == 1.0) {
                    conn_webdriver
                        .execute("window.scrollTo(0, document.body.scrollHeight)", arguments)
                        .await?;
                } else if (mode == "full") && (*page_size == -1.0) {
                    conn_webdriver
                        .execute("window.scrollTo(0, 0)", arguments)
                        .await?;
                } else if mode == "page" {
                    conn_webdriver
                        .execute(
                            "window.scrollBy(0, arguments[0] * window.innerHeight)",
                            arguments,
                        )
                        .await?;
                } else {
                    println!(
                        "{:>width$}Incorrect arguments!",
                        "",
                        width = depth * config::TAB_SIZE
                    )
                }
                Ok(())
            }
            WorkflowStep::PageTakeScreenshot(file_prefix) => {
                let file_name = format!("{wf_name}_{file_prefix}_{}.png", utils::timestamp());
                let data = conn_webdriver.screenshot().await?;
                utils::write_file(&config.temp_dir, &file_name, &data)?;
                Ok(())
            }
            WorkflowStep::PageWait(duration_ms) => {
                // reduce the wait time if running in remote mode
                let duration_ms = if config.headless_browser {
                    config::REMOTE_WAIT_FACTOR * (*duration_ms as f64)
                } else {
                    *duration_ms as f64
                }
                .round() as u64;
                println!(
                    "{:>width$}Effective wait time: {}ms",
                    "",
                    duration_ms,
                    width = depth * config::TAB_SIZE
                );
                tokio::time::sleep(std::time::Duration::from_millis(duration_ms)).await;
                Ok(())
            }
            WorkflowStep::ElementClick(check_enabled, check_url) => {
                let current_elements = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!");
                let elem = current_elements
                    .get(current_elements.len() - 1)
                    .expect("Workflow Definition Error: `current_element` is empty!");
                // Make sure the element is enabled if `check_enabled` is `true`
                if *check_enabled {
                    let status = elem.is_enabled().await?;
                    if !status {
                        return Err(fantoccini::error::CmdError::NotJson(
                            "Element is Disabled".to_string(),
                        ));
                    }
                }
                // Get URL before the click
                let b_url = conn_webdriver.current_url().await?;
                // Click on the element
                elem.click().await?;
                // Get URL after the click
                let a_url = conn_webdriver.current_url().await?;
                println!(
                    "{:>width$}Before URL: {}, After URL: {}!",
                    "",
                    b_url,
                    a_url,
                    width = depth * config::TAB_SIZE
                );
                // Make sure current_url is changed if `check_url` is `true`
                if *check_url {
                    if b_url == a_url {
                        return Err(fantoccini::error::CmdError::NotJson(
                            "No change in URL".to_string(),
                        ));
                    }
                }
                Ok(())
            }
            WorkflowStep::ElementClickNewWindow => {
                let current_elements = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!");
                let elem = current_elements
                    .get(current_elements.len() - 1)
                    .expect("Workflow Definition Error: `current_element` is empty!");
                let href = elem.attr("href").await?.expect(
                    "Workflow Definition Error: Selected elements doesn't have 'href' attribute!",
                );
                println!(
                    "{:>width$}Current Element HREF: {}",
                    "",
                    href,
                    width = depth * config::TAB_SIZE
                );
                let new_window = conn_webdriver.new_window(true).await?.handle;
                conn_webdriver.switch_to_window(new_window).await?;
                conn_webdriver
                    .set_window_size(config::WINDOW_WIDTH_DEFAULT, config::WINDOW_HEIGHT_DEFAULT)
                    .await?;
                conn_webdriver.goto(&href).await?;
                let size = conn_webdriver.get_window_size().await?;
                println!(
                    "{:>width$}Window Size: {:?}",
                    "",
                    size,
                    width = depth * config::TAB_SIZE
                );
                Ok(())
            }
            WorkflowStep::ElementSendKeys(keys) => {
                let current_elements = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!");
                let elem = current_elements
                    .get(current_elements.len() - 1)
                    .expect("Workflow Definition Error: `current_element` is empty!");
                elem.send_keys(keys).await?;
                Ok(())
            }
            WorkflowStep::ElementTakeScreenshot(file_prefix) => {
                let current_elements = current_elements_stack
                    .get(current_elements_stack.len() - 1)
                    .expect("Workflow Definition Error: `current_elements_stack` is empty!");
                let elem = current_elements
                    .get(current_elements.len() - 1)
                    .expect("Workflow Definition Error: `current_element` is empty!");
                let len = current_elements.len();
                let file_name = format!("{file_prefix}_{}_{len}.png", utils::timestamp());
                match elem.screenshot().await {
                    // The error must be ignored to let the workflow continue, as
                    // some elements may be of 0 height
                    Err(error) => {
                        println!(
                            "{:>width$}Ignoring the error: {error}",
                            "",
                            width = depth * config::TAB_SIZE
                        );
                    }
                    Ok(value) => {
                        utils::write_file(&config.temp_dir, &file_name, &value)?;
                    }
                };
                Ok(())
            }
            WorkflowStep::PrintCurrentValues => {
                for (i, value) in current_values.iter().enumerate() {
                    let tup = (i, value);
                    println!(
                        "{:>width$}Value: {:?}",
                        "",
                        tup,
                        width = depth * config::TAB_SIZE
                    );
                }
                Ok(())
            }
        }
    }

    /// Provide string representation of a WorkflowStep
    /// Overriding what's derived from strum_macros::Display
    pub fn to_string(&self) -> String {
        match self {
            WorkflowStep::PageLocateElements(css, mode, index) => {
                format!("{self} by css={css} in mode={mode} at index={index}")
            }
            WorkflowStep::ElementsLoopThrough(sub_steps) => {
                format!("{self} with sub_steps: {:?}", sub_steps)
            }
            WorkflowStep::PageLoop(sub_steps) => {
                format!("{self} with sub_steps: {:?}", sub_steps)
            }
            WorkflowStep::ElementSaveHtmlValue(name, is_inner) => {
                format!("{self} with name={name}, inner={is_inner}")
            }
            WorkflowStep::PageOpen(url) => format!("{self} {url}"),

            WorkflowStep::PageScroll(mode, page_size) => {
                format!("{self} in {mode} mode by {page_size} pages")
            }
            WorkflowStep::PageTakeScreenshot(file_prefix) => {
                format!("{self} with file_prefix={file_prefix}")
            }
            WorkflowStep::PageWait(duration_ms) => format!("{self} for {duration_ms}ms"),

            WorkflowStep::ElementClick(check_enabled, check_url) => {
                format!("{self} with check_enabled={check_enabled}, check_url={check_url}")
            }
            WorkflowStep::ElementSendKeys(keys) => format!("{self} with keys={keys}"),

            WorkflowStep::ElementTakeScreenshot(file_prefix) => {
                format!("{self} with file_prefix={file_prefix}")
            }
            // default representation for simple and/or uncovered cases
            _ => format!("{self}"),
        }
    }
}

