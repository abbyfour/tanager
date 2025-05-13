use lofty::{AudioFile, TaggedFile, TaggedFileExt};

use super::{requested_tags::RequestedTags, tag_applicator::TagApplicator};

use super::applicators::{album_artist, base_tags};

pub struct TagEditRequest<'a> {
    pub tagged_file: &'a mut TaggedFile,
    pub tags: &'a RequestedTags,
}

pub struct TagEditor {
    applicators: Vec<Box<dyn TagApplicator>>,
}

impl TagEditor {
    pub fn new() -> Self {
        Self {
            applicators: vec![
                Box::new(base_tags::TitleApplicator {}),
                Box::new(base_tags::ArtistApplicator {}),
                Box::new(base_tags::AlbumApplicator {}),
                Box::new(album_artist::AlbumArtistApplicator {}),
            ],
        }
    }

    pub fn apply_and_save(
        &self,
        request: &mut TagEditRequest,
        to_path: String,
    ) -> Result<(), String> {
        self.apply(request)?;
        self.save(request, to_path)
    }

    pub fn apply(&self, request: &mut TagEditRequest) -> Result<(), String> {
        let tag = request
            .tagged_file
            .primary_tag_mut()
            .ok_or_else(|| "No primary tag found in the file".to_string())?;

        for applicator in &self.applicators {
            if applicator.should_apply(tag, request.tags) {
                applicator.apply(tag, request.tags)?;
            }
        }

        Ok(())
    }

    pub fn save(&self, request: &TagEditRequest, to_path: String) -> Result<(), String> {
        request
            .tagged_file
            .save_to_path(to_path)
            .map_err(|e| format!("Failed to save file: {e}"))
    }
}
