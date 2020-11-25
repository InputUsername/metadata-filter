//! Functionality to filter artist, album and song names.

use std::borrow::Cow;

use crate::rules::FilterRule;

fn apply_once(text: String, rules: &[FilterRule]) -> String {
    rules.iter().fold(text, |mut result, rule| {
        let filtered = rule.apply(&result);
        if let Cow::Owned(filtered) = filtered {
            result.clear();
            result.push_str(&filtered);
        }
        result
    })
}

/// Apply a list of filter rules until the result doesn't change anymore.
pub fn apply_rules(text: &str, rules: &[FilterRule]) -> String {
    let mut prev = text.to_string();
    let mut result = apply_once(text.to_string(), rules);
    while result != prev {
        prev.clear();
        prev.push_str(&result);
        result = apply_once(result, rules);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::*;

    #[test]
    fn test_multiple_rulesets() {
        let rules = [remastered_filter_rules(), trim_whitespace_filter_rules()].concat();
        let filtered = apply_rules("Here Comes The Sun (Remastered)", &rules);

        assert_eq!(filtered, "Here Comes The Sun");
    }
}
