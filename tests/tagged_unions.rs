// Guards scripts/fix-discriminator-mapping.py. The Track spec discriminates v2 entity/batch
// payloads on `action` without a mapping, and every member also declares `action` as a field.
// Unpatched, the generator emits model names as tags ("Person__Identify") and a duplicate
// `action` key. The wire format must be `{"action": "identify", ...}` with one `action`.
#![cfg(feature = "track")]

use autogen_customerio::track::models::{
    Id1, IdentifyPerson, PersonCommonIdentifiers, PersonOperations, identify_person,
};

fn identify() -> PersonOperations {
    PersonOperations::Identify(Box::new(IdentifyPerson::new(
        PersonCommonIdentifiers::Id1(Box::new(Id1::new("42".to_string()))),
        identify_person::Type::Person,
    )))
}

#[test]
fn serializes_the_spec_tag_value_once() {
    let json = serde_json::to_string(&identify()).unwrap();
    assert_eq!(json.matches("\"action\"").count(), 1, "{json}");
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["action"], "identify");
    assert_eq!(value["type"], "person");
    assert_eq!(value["identifiers"]["id"], "42");
}

#[test]
fn round_trips() {
    let json = serde_json::to_string(&identify()).unwrap();
    let parsed: PersonOperations = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, identify());
}
