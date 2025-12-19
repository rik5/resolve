#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Outcome {
    Match(String),
    NoMatch,
    Error(String),
    Unsupported(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DecisionStep {
    pub source: String,
    pub outcome: Outcome,
    pub reason: String,
}

#[allow(dead_code)]
pub type DecisionTree = Vec<DecisionStep>;

#[allow(dead_code)]
pub fn build_decision_tree(/* params */) -> DecisionTree {
    // TODO: Implement decision tree building
    vec![]
}