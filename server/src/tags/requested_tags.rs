use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestedTags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
}
