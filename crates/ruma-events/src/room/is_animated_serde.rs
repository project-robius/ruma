//! De-/serialization functions for `Option<bool>` objects representing the `is_animated` flag.

use serde::{
    Deserialize, Deserializer,
    ser::{SerializeStruct, Serializer},
};
use serde_json::Value as JsonValue;

/// Serializes an `is_animated` flag to its stable field.
pub(super) fn serialize<S>(is_animated: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(is_animated) = is_animated {
        let mut st = serializer.serialize_struct("IsAnimated", 1)?;
        st.serialize_field("is_animated", is_animated)?;
        st.end()
    } else {
        serializer.serialize_none()
    }
}

/// Deserializes an `is_animated` flag from its stable field, or else its unstable field.
///
/// An unstable field that isn't a boolean is ignored, so it can't break the whole event.
pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct IsAnimatedJsonRepr {
        is_animated: Option<bool>,
        #[serde(rename = "org.matrix.msc4230.is_animated")]
        is_animated_unstable: Option<JsonValue>,
    }

    let IsAnimatedJsonRepr { is_animated, is_animated_unstable } =
        IsAnimatedJsonRepr::deserialize(deserializer)?;

    Ok(is_animated.or(is_animated_unstable.and_then(|value| value.as_bool())))
}
