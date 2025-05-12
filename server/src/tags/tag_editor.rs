use lofty::{AudioFile, TaggedFile, TaggedFileExt};

use super::{requested_tags::RequestedTags, tag_transformer::TagTransformer};

use super::transformers::{album_artist, base_tags};

pub struct TagEditor {
    tagged_file: TaggedFile,
    request: RequestedTags,

    transformers: Vec<Box<dyn TagTransformer>>,
}

impl TagEditor {
    pub fn new(tagged_file: TaggedFile, request: RequestedTags) -> Self {
        Self {
            tagged_file,
            request,
            transformers: vec![
                Box::new(base_tags::TitleTransformer {}),
                Box::new(base_tags::ArtistTransformer {}),
                Box::new(base_tags::AlbumTransformer {}),
                Box::new(album_artist::AlbumArtistTransformer {}),
            ],
        }
    }

    pub fn apply_and_save(&mut self, to_path: String) -> Result<(), String> {
        self.apply()?;
        self.save(to_path)
    }

    pub fn apply(&mut self) -> Result<(), String> {
        let tag = self
            .tagged_file
            .primary_tag_mut()
            .ok_or_else(|| "No primary tag found in the file".to_string())?;

        for transformer in &mut self.transformers {
            if transformer.should_transform(tag, &self.request) {
                transformer.transform(tag, &self.request)?;
            }
        }

        Ok(())
    }

    pub fn save(&mut self, to_path: String) -> Result<(), String> {
        self.tagged_file
            .save_to_path(to_path)
            .map_err(|e| format!("Failed to save file: {e}"))
    }
}
