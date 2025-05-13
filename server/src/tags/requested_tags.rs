use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    String(String),

    // This is used to clear a tag,
    // the clear value is not used but is required for deserialization
    #[allow(dead_code)]
    Clear {
        clear: bool,
    },
}

#[derive(Debug, Deserialize)]
pub struct RequestedTags {
    pub title: Option<TagValue>,
    pub artist: Option<TagValue>,
    pub album: Option<TagValue>,
    pub album_artist: Option<TagValue>,
}
