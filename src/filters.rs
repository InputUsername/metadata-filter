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
///
/// # Example
///
/// ```
/// use metadata_filter::rules::{remastered_filter_rules, trim_whitespace_filter_rules};
/// use metadata_filter::filters::apply_rules;
///
/// let mut rules = remastered_filter_rules();
/// rules.extend(trim_whitespace_filter_rules());
/// let filtered = apply_rules("Here Comes The Sun (Remastered)", &rules);
///
/// assert_eq!(filtered, "Here Comes The Sun");
/// ```
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