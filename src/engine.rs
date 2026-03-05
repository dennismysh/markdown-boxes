use std::collections::HashMap;

/// Evaluate MDAL expression blocks (conditionals and loops) in template body.
/// Processes {{#if}}/{{else}}/{{/if}} and {{#each}} blocks.
/// Returns the body with blocks expanded/collapsed based on values.
pub fn evaluate_blocks(body: &str, values: &HashMap<String, String>) -> String {
    let after_ifs = evaluate_ifs(body, values);
    evaluate_eachs(&after_ifs, values)
}

fn evaluate_ifs(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut remaining = body;

    while let Some(start) = remaining.find("{{#if ") {
        result.push_str(&remaining[..start]);

        let after_tag = &remaining[start + 6..];
        let cond_end = match after_tag.find("}}") {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };
        let condition = after_tag[..cond_end].trim();

        let block_start = start + 6 + cond_end + 2;
        let rest = &remaining[block_start..];

        let endif_tag = "{{/if}}";
        let else_tag = "{{else}}";

        let endif_pos = match rest.find(endif_tag) {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };

        let (if_block, else_block) = if let Some(else_pos) = rest[..endif_pos].find(else_tag) {
            (
                rest[..else_pos].trim(),
                rest[else_pos + else_tag.len()..endif_pos].trim(),
            )
        } else {
            (rest[..endif_pos].trim(), "")
        };

        let truthy = evaluate_condition(condition, values);

        if truthy {
            result.push_str(if_block);
        } else {
            result.push_str(else_block);
        }

        remaining = &rest[endif_pos + endif_tag.len()..];
    }

    result.push_str(remaining);
    result
}

fn evaluate_eachs(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut remaining = body;

    while let Some(start) = remaining.find("{{#each ") {
        result.push_str(&remaining[..start]);

        let after_tag = &remaining[start + 8..];
        let tag_end = match after_tag.find("}}") {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };

        let expr = &after_tag[..tag_end];
        let parts: Vec<&str> = expr.split(" as ").collect();
        let collection_name = parts[0].trim();
        let item_name = if parts.len() > 1 {
            parts[1].trim()
        } else {
            "item"
        };

        let rest = &after_tag[tag_end + 2..];

        let end_tag = "{{/each}}";
        let end_pos = match rest.find(end_tag) {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };

        let loop_body = &rest[..end_pos];

        let items_str = values
            .get(collection_name)
            .map(|s| s.as_str())
            .unwrap_or("");
        if !items_str.is_empty() {
            for item in items_str.split(',') {
                let item = item.trim();
                let placeholder = format!("{{{{{item_name}}}}}");
                let expanded = loop_body.replace(&placeholder, item);
                result.push_str(&expanded);
            }
        }

        remaining = &rest[end_pos + end_tag.len()..];
    }

    result.push_str(remaining);
    result
}

fn evaluate_condition(condition: &str, values: &HashMap<String, String>) -> bool {
    let condition = condition.trim();

    // Check for == comparison
    if let Some((left, right)) = condition.split_once("==") {
        let left = left.trim();
        let right = right.trim().trim_matches('"');
        let val = values.get(left).map(|s| s.as_str()).unwrap_or("");
        return val == right;
    }

    // Check for > comparison
    if let Some((left, right)) = condition.split_once('>') {
        let left = left.trim();
        let right = right.trim();

        if left.ends_with(".length") {
            let var_name = left.trim_end_matches(".length");
            let val = values.get(var_name).map(|s| s.as_str()).unwrap_or("");
            let count = if val.is_empty() {
                0
            } else {
                val.split(',').count()
            };
            if let Ok(n) = right.parse::<usize>() {
                return count > n;
            }
        }
    }

    // Bare variable — truthy if non-empty
    let val = values.get(condition).map(|s| s.as_str()).unwrap_or("");
    !val.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_truthy_variable() {
        let body = "before {{#if name}}Hello{{/if}} after";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Alice".to_string());
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "before Hello after");
    }

    #[test]
    fn if_falsy_variable() {
        let body = "before {{#if name}}Hello{{/if}} after";
        let values = HashMap::new();
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "before  after");
    }

    #[test]
    fn if_else_truthy() {
        let body = "{{#if name}}Hello{{else}}No name{{/if}}";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Bob".to_string());
        assert_eq!(evaluate_blocks(body, &values), "Hello");
    }

    #[test]
    fn if_else_falsy() {
        let body = "{{#if name}}Hello{{else}}No name{{/if}}";
        let values = HashMap::new();
        assert_eq!(evaluate_blocks(body, &values), "No name");
    }

    #[test]
    fn if_equality_check() {
        let body = "{{#if role == \"admin\"}}Admin panel{{else}}User view{{/if}}";
        let mut values = HashMap::new();
        values.insert("role".to_string(), "admin".to_string());
        assert_eq!(evaluate_blocks(body, &values), "Admin panel");
    }

    #[test]
    fn if_equality_check_fails() {
        let body = "{{#if role == \"admin\"}}Admin panel{{else}}User view{{/if}}";
        let mut values = HashMap::new();
        values.insert("role".to_string(), "user".to_string());
        assert_eq!(evaluate_blocks(body, &values), "User view");
    }

    #[test]
    fn no_conditionals_passthrough() {
        let body = "Just regular text with {{placeholder}}";
        let values = HashMap::new();
        assert_eq!(evaluate_blocks(body, &values), body);
    }

    #[test]
    fn malformed_if_left_as_literal() {
        let body = "before {{#if name after";
        let values = HashMap::new();
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "before {{#if name after");
    }

    #[test]
    fn each_loop_basic() {
        let body = "{{#each items as item}}- {{item}}\n{{/each}}";
        let mut values = HashMap::new();
        values.insert("items".to_string(), "Apple,Banana,Cherry".to_string());
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "- Apple\n- Banana\n- Cherry\n");
    }

    #[test]
    fn each_loop_empty() {
        let body = "{{#each items as item}}- {{item}}\n{{/each}}";
        let values = HashMap::new();
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "");
    }
}
