use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StyleSpec {
    pub name: String,
    pub approach: Option<String>,
    pub colors: HashMap<String, String>,
    pub typography: HashMap<String, String>,
    pub effects: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_style_spec() {
        let json = "{\"name\":\"DesignSystem\",\"approach\":\"brutalist\",\"colors\":{\"primary\":\"#000\",\"background\":\"#fff\"},\"typography\":{\"headings\":\"Mono\"},\"effects\":{\"borders\":\"2px\"}}";

        let s: StyleSpec = serde_json::from_str(json).unwrap();
        assert_eq!(s.approach.unwrap(), "brutalist");
        assert_eq!(s.colors.get("primary").unwrap(), "#000");
    }
}
