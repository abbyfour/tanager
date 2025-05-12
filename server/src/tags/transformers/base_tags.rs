use crate::tags::{requested_tags::RequestedTags, tag_transformer::TagTransformer};
use lofty::Accessor;

pub struct TitleTransformer {}

impl TagTransformer for TitleTransformer {
    fn should_transform(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.title.is_some()
    }

    fn transform(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        if let Some(title) = &values.title {
            tag.set_title(title.clone());
        }

        Ok(())
    }
}

pub struct ArtistTransformer {}

impl TagTransformer for ArtistTransformer {
    fn should_transform(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.artist.is_some()
    }

    fn transform(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        if let Some(artist) = &values.artist {
            tag.set_artist(artist.clone());
        }

        Ok(())
    }
}

pub struct AlbumTransformer {}

impl TagTransformer for AlbumTransformer {
    fn should_transform(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.album.is_some()
    }

    fn transform(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        if let Some(album) = &values.album {
            tag.set_album(album.clone());
        }

        Ok(())
    }
}
