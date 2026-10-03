// Guards the open-enum rewrite in scripts/fix-open-enums.py: the API can return enum values
// the spec does not list yet, and without a fallback variant one such string fails
// deserialization of the entire response. If a regeneration ever drops the
// `#[serde(untagged)] UnknownValue(String)` variant, these tests fail.
#![cfg(feature = "track")]

use autogen_customerio::track::models::identify_person::Type;

#[test]
fn spec_values_still_parse_to_named_variants() {
    let parsed: Type = serde_json::from_str("\"person\"").unwrap();
    assert_eq!(parsed, Type::Person);
}

#[test]
fn unknown_value_falls_back_and_round_trips() {
    let parsed: Type = serde_json::from_str("\"robot\"").unwrap();
    assert_eq!(parsed, Type::UnknownValue("robot".to_string()));
    assert_eq!(serde_json::to_string(&parsed).unwrap(), "\"robot\"");
}
