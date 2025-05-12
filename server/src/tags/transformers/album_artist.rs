use crate::tags::{requested_tags::RequestedTags, tag_transformer::TagTransformer};

pub struct AlbumArtistTransformer {}

impl TagTransformer for AlbumArtistTransformer {
    fn should_transform(&self, _tag: &lofty::Tag, values: &RequestedTags) -> bool {
        values.album_artist.is_some()
    }

    fn transform(&self, tag: &mut lofty::Tag, values: &RequestedTags) -> Result<(), String> {
        if let Some(album_artist) = &values.album_artist {
            tag.insert_text(lofty::ItemKey::AlbumArtist, album_artist.clone());
        }

        Ok(())
    }
}
