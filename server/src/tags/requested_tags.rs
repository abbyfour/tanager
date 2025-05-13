use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ClearableValue<T = String> {
    Value(T),

    // This is used to clear a tag,
    // the clear value is not used but is required for deserialization
    #[allow(dead_code)]
    Clear {
        clear: bool,
    },
}

#[derive(Debug, Deserialize)]
pub struct RequestedTags {
    pub title: Option<ClearableValue>,
    pub artist: Option<ClearableValue>,
    pub album: Option<ClearableValue>,
    pub album_artist: Option<ClearableValue>,
}
