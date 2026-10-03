// Guards scripts/fix-id-types.py and src/ids.rs: allowlisted IDs must reach the generated
// signatures and models as newtypes, and must stay plain JSON numbers / strings on the wire.
// If a regeneration drops the rewrite, the function-pointer coercions below stop compiling.
#![cfg(all(feature = "app", feature = "pipelines"))]

use autogen_customerio::app::apis::{configuration::Configuration, messages_api};
use autogen_customerio::app::models::BroadcastObject;
use autogen_customerio::ids::{BroadcastId, DeliveryId, UserId};

#[test]
fn entity_model_ids_are_typed_and_transparent() {
    let parsed: BroadcastObject = serde_json::from_str(r#"{"id": 7}"#).unwrap();
    assert_eq!(parsed.id, Some(BroadcastId::new(7)));
    assert_eq!(serde_json::to_value(&parsed).unwrap()["id"], 7);
}

#[test]
fn string_ids_are_transparent() {
    let id = DeliveryId::new("dlv_1");
    assert_eq!(serde_json::to_string(&id).unwrap(), "\"dlv_1\"");
    assert_eq!(id.to_string(), "dlv_1");
}

#[test]
fn integer_ids_are_i64() {
    // Customer.io IDs and timestamps exceed i32 range eventually; the newtype holds i64.
    let id = BroadcastId::new(i64::from(i32::MAX) + 1);
    assert_eq!(id.get(), 2_147_483_648);
}

#[test]
fn generated_signatures_take_newtypes() {
    async fn uses_delivery_id(config: &Configuration, id: &DeliveryId) {
        let _ = messages_api::get_message(config, id, None).await;
    }
    let _ = uses_delivery_id;
    let _ = autogen_customerio::pipelines::models::KnownUser1::new(
        autogen_customerio::pipelines::models::known_user_1::Type::Identify,
        UserId::new("u1"),
    );
}
