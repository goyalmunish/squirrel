use crate::{config, utils};

/// `WorkflowStep` enum defines individual step of `Workflow`.
/// Check its variants for currently supported actions.
#[derive(serde::Serialize, serde::Deserialize, Debug, strum_macros::Display)]
pub enum WorkflowStep {
    /// Open given url
    ///
    /// Arguments: `url` (`String`)
    PageOpen(String),
    /// Go back
    PageBack,
    /// Find elements on current page.
    ///
    /// Use given css as selector.
    /// If running in "all" mode, search all elements, ignoring the index.
    /// If running in "index" mode, then only select the element at given index
    /// among all found entries.
    /// If no matching element is found, return an error.
    ///
    /// Arguments: `css` (`String`), `mode` ("all"/"index") (`String`), `index` (`usize`)
    PageLocateElements(String, String, usize),
    /// Scroll current page.
    ///
    /// If running in "full" mode, provide page_size as 1.0 for scroll to
    /// the bottom, or page_size as -1.0 to scroll to the top of the page.
    /// If running in "page" mode, provide page_size.
    ///
    /// Arguments: `mode` (`String`), `page_size` (`f64`)
    PageScroll(String, f64),
    /// Take screenshot of current page.
    ///
    /// Arguments: `file_prefix` (`String`)
    PageTakeScreenshot(String),
    /// Wait for given milliseconds.
    ///
    /// Arguments: `duration_ms` (milliseconds) (`String`)
    PageWait(u64),
    /// Run an infinite loop, with given sub-steps.
    /// If any of its sub-steps returns error, break the loop.
    ///
    /// For example, this is helpful in the conditions where you want to
    /// keep clicking on "Next" button in pagination, until all pages are
    /// exhausted and so the "Next" button couldn't be found.
    ///
    /// Arguments: `sub_steps` (`Vec<WorkflowStep>`)
    PageLoop(Vec<WorkflowStep>),
    /// Loop through all current-page-elements, with given sub-steps.
    ///
    /// It is associated with `PageLocateElements`, which populates the
    /// current-page-elements Vector based on its outcome.
    ///
    /// Arguments: `sub_steps` (`Vec<WorkflowStep>`)
    ElementsLoopThrough(Vec<WorkflowStep>),
    /// Click the currently selected page element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ElementClick,
    /// Send keys to currently selected page element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Arguments: `keys` (`String`)
    ElementSendKeys(String),
    /// Save HTML value of the currently selected element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Arguments: `is_inner` (`bool`)
    ElementSaveHtmlValue(bool),
    /// Take screenshot of the current element.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Arguments: `file_prefix` (`String`)
    ElementTakeScreenshot(String),
    /// Remove current element from the currently selected page elements.
    ///
    /// It is associated with `ElementsLoopThrough` for the currently
    /// selected page element (the top element of current-page-elements Vector).
    ///
    /// Note: It is mandatory last sub-step for sub steps under `ElementsLoopThrough`.
    /// Note: As it's errors are not show-stopper, they are ignored to let
    /// the loop continue.
    ElementPop,

    PrintCurrentValues,
    // ElemOpenInCurrentTab,
    // ElemOpenInNewTab,
    // ElemExtractValueAs(String),
    // HelperSaveOutputAs(String),
    // HelperSanitizeText,
    // HelperParseNumbers,
}

impl WorkflowStep {
    /// Execute a WorkflowStep
    #[async_recursion::async_recursion]
    pub async fn execute(
        &self,
        config: &config::Config,
        conn_webdriver: &fantoccini::Client,
        current_elements: &mut Vec<fantoccini::elements::Element>,
        current_values: &mut Vec<String>,
        depth: usize,
    ) -> Result<(), fantoccini::error::CmdError> {
        let depth = depth + 1;
        match self {
            WorkflowStep::PageOpen(url) => {
                conn_webdriver.goto(url).await?;
                Ok(())
            }
            WorkflowStep::PageBack => {
                conn_webdriver.back().await?;
                Ok(())
            }
            WorkflowStep::PageLocateElements(css, mode, index) => {
                let mut elements = conn_webdriver
                    .find_all(fantoccini::Locator::Css(css))
                    .await?;
                if mode == "all" {
                    *current_elements = elements;
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
                    current_elements.push(elements.remove(*index));
                } else {
                    println!(
                        "{:>width$}Incorrect arguments!",
                        "",
                        width = depth * config::TAB_SIZE
                    )
                }
                println!(
                    "{:>width$}Current Element Size: {}",
                    "",
                    current_elements.len(),
                    width = depth * config::TAB_SIZE
                );
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
                let file_name = format!("{file_prefix}_{}.png", utils::timestamp());
                let data = conn_webdriver.screenshot().await?;
                utils::write_file(&config.temp_dir, &file_name, &data)?;
                Ok(())
            }
            WorkflowStep::PageWait(duration_ms) => {
                tokio::time::sleep(std::time::Duration::from_millis(*duration_ms)).await;
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
                                current_elements,
                                current_values,
                                depth + 1,
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
            WorkflowStep::ElementsLoopThrough(sub_steps) => {
                while current_elements.len() > 0 {
                    let index_elem = current_elements.len() - 1;
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
                                current_elements,
                                current_values,
                                depth + 1,
                            )
                            .await?;
                    }
                }
                Ok(())
            }
            WorkflowStep::ElementClick => {
                let elem = current_elements.pop();
                match elem {
                    None => {
                        println!(
                            "{:>width$}End of loop!",
                            "",
                            width = depth * config::TAB_SIZE
                        )
                    }
                    Some(elem) => {
                        elem.click().await?;
                    }
                }
                Ok(())
            }
            WorkflowStep::ElementSendKeys(keys) => {
                let elem = current_elements.pop();
                match elem {
                    None => {
                        println!(
                            "{:>width$}End of loop!",
                            "",
                            width = depth * config::TAB_SIZE
                        )
                    }
                    Some(elem) => {
                        elem.send_keys(keys).await?;
                    }
                }
                Ok(())
            }
            WorkflowStep::ElementSaveHtmlValue(is_inner) => {
                let elem = current_elements.pop();
                match elem {
                    None => {
                        println!(
                            "{:>width$}End of loop!",
                            "",
                            width = depth * config::TAB_SIZE
                        );
                    }
                    Some(elem) => {
                        let elem_html = elem.html(*is_inner).await?;
                        current_values.push(elem_html);
                        current_elements.push(elem);
                    }
                }
                Ok(())
            }
            WorkflowStep::ElementTakeScreenshot(file_prefix) => {
                let elem_option = current_elements.pop();
                let len = current_elements.len();
                match elem_option {
                    None => {
                        println!(
                            "{:>width$}Element not found!",
                            "",
                            width = depth * config::TAB_SIZE
                        )
                    }
                    Some(elem) => {
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
                                current_elements.push(elem);
                            }
                        };
                    }
                }
                Ok(())
            }
            WorkflowStep::ElementPop => {
                current_elements.pop();
                Ok(())
            }
            WorkflowStep::PrintCurrentValues => {
                for (i, value) in current_values.iter().enumerate() {
                    println!(
                        "{:>width$}HTML at index {i}: {value}",
                        "",
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
            WorkflowStep::PageOpen(url) => format!("{self} {url}"),
            WorkflowStep::PageLocateElements(css, mode, index) => {
                format!("{self} by {css} css in {mode} mode at {index} index")
            }
            WorkflowStep::PageScroll(mode, page_size) => {
                format!("{self} in {mode} mode by {page_size} pages")
            }
            WorkflowStep::PageTakeScreenshot(file_prefix) => {
                format!("{self} with file_prefix={file_prefix}")
            }
            WorkflowStep::PageWait(duration_ms) => format!("{self} for {duration_ms}ms"),
            WorkflowStep::PageLoop(sub_steps) => {
                format!("{self} with sub_steps: {:?}", sub_steps)
            }
            WorkflowStep::ElementsLoopThrough(sub_steps) => {
                format!("{self} with sub_steps: {:?}", sub_steps)
            }
            WorkflowStep::ElementSendKeys(keys) => format!("{self} with keys={keys}"),
            WorkflowStep::ElementSaveHtmlValue(is_inner) => format!("{self} with inner={is_inner}"),
            WorkflowStep::ElementTakeScreenshot(file_prefix) => {
                format!("{self} with file_prefix={file_prefix}")
            }
            // default representation for simple and/or uncovered cases
            _ => format!("{self}"),
        }
    }
}
