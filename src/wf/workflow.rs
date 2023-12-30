use crate::wf::workflow_step;

/// `Workflow` struct defines workflow name and sequence of steps to be executed.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Workflow {
    pub name: String,
    pub steps: Vec<workflow_step::WorkflowStep>,
}
