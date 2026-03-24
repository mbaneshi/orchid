use uuid::Uuid;

use crate::{InputMapping, Step, Trigger, Workflow};

pub fn builtin_workflows() -> Vec<Workflow> {
    vec![dev_to_content(), repo_health()]
}

fn dev_to_content() -> Workflow {
    let summarizer_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let drafter_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();

    Workflow {
        id: Uuid::parse_str("10000000-0000-0000-0000-000000000001").unwrap(),
        name: "dev-to-content".to_string(),
        description: "Summarize git history, then draft social media content".to_string(),
        trigger: Trigger::Manual,
        steps: vec![
            Step {
                id: summarizer_id,
                name: "summarize".to_string(),
                agent_name: "git-summarizer".to_string(),
                depends_on: vec![],
                input_mapping: InputMapping::FromCli,
            },
            Step {
                id: drafter_id,
                name: "draft".to_string(),
                agent_name: "content-drafter".to_string(),
                depends_on: vec![summarizer_id],
                input_mapping: InputMapping::FromStep(summarizer_id),
            },
        ],
    }
}

fn repo_health() -> Workflow {
    let summarizer_id = Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();

    Workflow {
        id: Uuid::parse_str("10000000-0000-0000-0000-000000000002").unwrap(),
        name: "repo-health".to_string(),
        description: "Summarize git history and check repository health".to_string(),
        trigger: Trigger::Manual,
        steps: vec![Step {
            id: summarizer_id,
            name: "summarize".to_string(),
            agent_name: "git-summarizer".to_string(),
            depends_on: vec![],
            input_mapping: InputMapping::FromCli,
        }],
    }
}
