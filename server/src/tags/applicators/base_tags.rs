use crate::tags::{
    requested_tags::{ClearableValue, RequestedTags},
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
            Some(ClearableValue::Value(title)) => {
                tag.set_title(title.clone());
                Ok(())
            }
            Some(ClearableValue::Clear { clear: _ }) => {
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
            Some(ClearableValue::Value(artist)) => {
                tag.set_artist(artist.clone());
                Ok(())
            }
            Some(ClearableValue::Clear { clear: _ }) => {
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
            Some(ClearableValue::Value(album)) => {
                tag.set_album(album.clone());
                Ok(())
            }
            Some(ClearableValue::Clear { clear: _ }) => {
                tag.insert_text(lofty::ItemKey::AlbumTitle, String::new());
                Ok(())
            }
            None => Err("Invalid album value provided".to_string()),
        }
    }
}
