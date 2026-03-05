use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub name: String,
    pub section_type: SectionType,
    pub properties: HashMap<String, String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SectionType {
    Component,
    Layout,
    Logic,
    Data,
    Style,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_section() {
        let json = "{\"name\": \"Hero\", \"section_type\": \"component\", \"properties\": {\"layout\": \"centered\", \"required\": \"true\"}, \"content\": \"Headline here\"}";
        let s: Section = serde_json::from_str(json).unwrap();
        assert_eq!(s.name, "Hero");
        assert_eq!(s.section_type, SectionType::Component);
        assert_eq!(s.properties.get("layout").unwrap(), "centered");
    }
}
