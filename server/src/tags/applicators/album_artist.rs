use crate::tags::requested_tags::ClearableValue;
use crate::tags::{requested_tags::RequestedTags, tag_applicator::TagApplicator};

pub struct AlbumArtistApplicator {}

impl TagApplicator for AlbumArtistApplicator {
    fn should_apply(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.album_artist.is_some()
    }

    fn apply(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        match &values.album_artist {
            Some(ClearableValue::Value(album_artist)) => {
                tag.insert_text(lofty::ItemKey::AlbumArtist, album_artist.clone());
                Ok(())
            }
            Some(ClearableValue::Clear { clear: _ }) => {
                tag.insert_text(lofty::ItemKey::AlbumArtist, String::new());
                Ok(())
            }
            None => Err("Invalid album artist value provided".to_string()),
        }
    }
}
