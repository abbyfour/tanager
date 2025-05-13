use crate::tags::{
    requested_tags::{RequestedTags, TagValue},
    tag_applicator::TagApplicator,
};
use lofty::Accessor;

pub struct TitleApplicator {}

impl TagApplicator for TitleApplicator {
    fn should_apply(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.title.is_some()
    }

    fn apply(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        match &values.title {
            Some(TagValue::String(title)) => {
                tag.set_title(title.clone());
                Ok(())
            }
            Some(TagValue::Clear { clear: _ }) => {
                tag.insert_text(lofty::ItemKey::TrackTitle, String::new());
                Ok(())
            }
            None => Err("Invalid title value provided".to_string()),
        }
    }
}

pub struct ArtistApplicator {}

impl TagApplicator for ArtistApplicator {
    fn should_apply(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.artist.is_some()
    }

    fn apply(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        match &values.artist {
            Some(TagValue::String(artist)) => {
                tag.set_artist(artist.clone());
                Ok(())
            }
            Some(TagValue::Clear { clear: _ }) => {
                tag.insert_text(lofty::ItemKey::TrackArtist, String::new());
                Ok(())
            }
            None => Err("Invalid artist value provided".to_string()),
        }
    }
}

pub struct AlbumApplicator {}

impl TagApplicator for AlbumApplicator {
    fn should_apply(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.album.is_some()
    }

    fn apply(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        match &values.album {
            Some(TagValue::String(album)) => {
                tag.set_album(album.clone());
                Ok(())
            }
            Some(TagValue::Clear { clear: _ }) => {
                tag.insert_text(lofty::ItemKey::AlbumTitle, String::new());
                Ok(())
            }
            None => Err("Invalid album value provided".to_string()),
        }
    }
}
