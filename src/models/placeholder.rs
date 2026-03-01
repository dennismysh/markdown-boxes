use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Placeholder {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub kind: PlaceholderType,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlaceholderType {
    Text,
    Select,
    Multiline,
    Boolean,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_text_placeholder() {
        let json = r#"{"key":"name","label":"Name","type":"text","options":null}"#;
        let p: Placeholder = serde_json::from_str(json).unwrap();
        assert_eq!(p.kind, PlaceholderType::Text);
        assert!(p.options.is_none());
    }

    #[test]
    fn deserialize_select_placeholder() {
        let json = r#"{"key":"framework","label":"Framework","type":"select","options":["React","Vue"]}"#;
        let p: Placeholder = serde_json::from_str(json).unwrap();
        assert_eq!(p.kind, PlaceholderType::Select);
        assert_eq!(p.options.as_ref().unwrap().len(), 2);
    }
}
