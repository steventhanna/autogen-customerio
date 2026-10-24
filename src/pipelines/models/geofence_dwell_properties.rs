/*
 * Pipelines API
 *
 * # Overview In general, you'll consume this API through one of our source libraries—our JavaScript client library or any of our server packages. But you can also integrate directly with our REST API if you don't want to install one of our libraries or you want to support a source that we don't have a native integration with.  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://cdp.customer.io | | EU | https://cdp-eu.customer.io |  If you're in our EU region, you'll need to specify the EU URL when you initialize our server-side libraries. If you use our JavaScript client library, we'll set your region and route data/calls automatically.  # Authentication & rate limits  Our API uses basic authorization with the API key from your **Customer.io API** integration. If you use Postman or another platform that helps you send API calls, this API key is the *Username*, and the *Password* is blank.  # Get an API key  You can get a Pipelines API key by [setting up the **Customer.io API** integration](/integrations/data-in/connections/http-api/) in your workspace.  Now you can use the key as a username with a blank password to authenticate and send requests to our API.  ## Rate and payload limits  A request is limited to 32KB. A batch request is limited to 500KB total and 32KB per call in the request. If a request exceeds these limits, you will receive a 200 response, but the request will not go through.  The Data Pipelines API has no fixed rate limit. Customer.io accepts your requests as fast as you send them and doesn't reject or drop them because of volume. Our guideline for both active integrations and historical backfills is about 3000 requests per 3 seconds, an average of 1000 requests per second. The 3-second window leaves room for short bursts.  Traffic above this level usually comes from high-volume integrations, which our Premium and Enterprise plans support. If your traffic regularly goes above this, we may contact you to learn more about your integration and talk about optimizing it, or about a plan that fits your volume.  <div class=\"fly-panel fly-light regionUS\"> <div class=\"fly-panel-body us-server\"> <p class=\"text--bold\">Warning</p> <p>Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.</p> </div></div>  # Try out our postman collection  We've generated a Postman collection with all of the endpoints organized as you'll find them on this page, with a starter environment (mainly to contain your API key). For our API endpoints, **your API key is your username and your password is blank**.  You'll notice that payloads on this page can contain significantly more information than the payloads that appear in our collection. We've limited our collections to the fields that you'll _typically_ use when you send calls to our APIs and libraries, so it's easier to get started. But you can add additional fields to payloads—like `context`, `integrations`, and so on—if you want.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61%26entityType%3Dcollection%26workspaceId%3D35e4a70d-66bd-4b3e-8a0c-57f9e32080dc#?env%5BCustomer.io%20Data%20Pipelines%20API%20Environment%5D=W3sia2V5IjoiY2RwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiY2RwLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJ3cml0ZV9rZXkiLCJ0eXBlIjoic2VjcmV0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfV0=)  # Deletions, suppressions, and other semantic events  You'll notice that this API only contains `POST` calls; we don't have `DELETE` operations. For delete operations, and other operations that don't have bespoke endpoints, we use *semantic events*.  When you need to do things like deleting people, removing relationships, and other sorts of things, you'll send a request to the `/track` endpoint with a specific event `name` parameter. The `name` tells us what to do with the request.  For example, you can send a `track` event with the name `Delete Person` to remove a person from your workspace.  See [Semantic Events](/integrations/data-in/semantic-events/cio-journeys/) to see Customer.io-specific events.  We also support semantic events for other kinds of destinations. See [Semantic Events for other destinations](#semantic-events-for-other-destinations) below for more information.  | Event Name | Description | |------|--------| | `Device Created or Updated` | Adds or updates a mobile device (by token), and associates it with a person | | `Device Deleted` | Deletes a device | | `User Deleted` | Removes a person from your workspace | | `User Suppressed` | Removes a person from your workspace and suppresses their identifiers so you can't add them back to Customer.io. | | `User Unsuppressed` | Unsuppresses an identifier so you can add someone back to Customer.io and message them again. | | `Relationship Deleted` | Remove a relationship between a person and an object| | `Object Deleted` | Remove an object (like an account or company) from your Customer.io workspace| | `Report Delivery Event` | Report delivery events for messages|  ## Semantic events for other destinations  In addition to events that have special meanings [in Customer.io](#deletions-suppressions-and-other-semantic-events), we have a number of other events that we support across different kinds of destinations. These uniform events ensure that we'll map event data to destinations consistently—so you can move from one provider to another without having to change your event names or payload structures.  For example, our ecommerce events work across any ecommerce platform we integrate with—even if those ecommerce platforms have different APIs or payload structures.  See [Semantic Events](/integrations/data-in/semantic-events/getting-started/) to learn more.  * [A/B Test](/integrations/api/cdp/ab-test/) * [Ecommerce](/integrations/api/cdp/ecommerce/) * [Email](/integrations/api/cdp/email/) * [Live Chat](/integrations/api/cdp/live-chat/) * [Mobile App](/integrations/api/cdp/mobile-app/) * [Video](/integrations/api/cdp/video/)  ## Backfilling data  By default, Customer.io records a `timestamp` when we receive requests. If you're sending data to Customer.io in real time, you don't need to worry about the timestamp.  If you want to backfill requests, you can send a `timestamp`—an ISO 8601 date-time string—telling us when the request occurred. This provides a way to log `track` and `page` calls when the activities _actually_ took place. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::pipelines::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeofenceDwellProperties {
    /// How the SDK confirmed that the person is still inside the geofence. Use this property for troubleshooting; values vary by platform.
    #[serde(rename = "detectionSource")]
    pub detection_source: String,
    /// The geofence's dwell time, in seconds, when the SDK confirmed the dwell.
    #[serde(rename = "dwellThresholdSeconds")]
    pub dwell_threshold_seconds: i64,
    /// The person stayed inside the geofence for at least the geofence's dwell time. The SDK only sends one dwell event per visit.
    #[serde(rename = "transition")]
    pub transition: Transition,
    /// An ID for the visit, from the time the person entered the geofence until they left. If the SDK sends an exit event for this visit, it carries the same `visitId`.
    #[serde(rename = "visitId")]
    pub visit_id: String,
    /// Observed seconds inside the geofence when the SDK confirmed the dwell. This can exceed the dwell time if confirmation was delayed. Rounded up to whole seconds. Included only when the SDK observed the entry and can time the visit reliably.
    #[serde(rename = "dwellDurationSeconds", skip_serializing_if = "Option::is_none")]
    pub dwell_duration_seconds: Option<i64>,
    /// When the person entered the geofence, as a Unix timestamp in seconds. Included only when the SDK knows when the person entered.
    #[serde(rename = "enteredAt", skip_serializing_if = "Option::is_none")]
    pub entered_at: Option<i64>,
    /// Customer.io-generated ID of the geofence. This is different from the `external_id` used for geofence imports.
    #[serde(rename = "geofenceId", skip_serializing_if = "Option::is_none")]
    pub geofence_id: Option<String>,
    /// The geofence's name. Included only when the geofence has one.
    #[serde(rename = "geofenceName", skip_serializing_if = "Option::is_none")]
    pub geofence_name: Option<String>,
    /// Customer.io-generated ID of the geofence set. The SDK sends one event for each geofence set the geofence belongs to. A geofence that isn't in any set produces a single event without a `geosetId`.
    #[serde(rename = "geosetId", skip_serializing_if = "Option::is_none")]
    pub geoset_id: Option<String>,
    /// The metadata you define on the geofence in Customer.io. Values keep their types—a numeric value arrives as a number, not a string. Metadata values must be JSON scalars (`string`, `number`, or `boolean`); the SDKs drop array and object values. If the geofence has no metadata, the property is an empty object.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, models::GeofenceTransitionBasePropertiesMetadataValue>>,
    /// A unique ID for this transition. Events sent for different geofence sets share the same ID, and it stays stable if the SDK retries delivery. To de-duplicate transition events, use `transitionId` and `geosetId` together.
    #[serde(rename = "transitionId", skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<String>,
}

impl GeofenceDwellProperties {
    pub fn new(detection_source: String, dwell_threshold_seconds: i64, transition: Transition, visit_id: String) -> GeofenceDwellProperties {
        GeofenceDwellProperties {
            detection_source,
            dwell_threshold_seconds,
            transition,
            visit_id,
            dwell_duration_seconds: None,
            entered_at: None,
            geofence_id: None,
            geofence_name: None,
            geoset_id: None,
            metadata: None,
            transition_id: None,
        }
    }
}
/// The person stayed inside the geofence for at least the geofence's dwell time. The SDK only sends one dwell event per visit.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Transition {
    #[serde(rename = "dwell")]
    Dwell,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for Transition {
    fn default() -> Transition {
        Self::Dwell
    }
}

