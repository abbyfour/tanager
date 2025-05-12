use super::requested_tags::RequestedTags;

pub trait TagTransformer {
    fn should_transform(&self, tag: &lofty::Tag, value: &RequestedTags) -> bool;
    fn transform(&self, tag: &mut lofty::Tag, value: &RequestedTags) -> Result<(), String>;
}
