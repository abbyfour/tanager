use super::requested_tags::RequestedTags;

pub trait TagApplicator {
    fn should_apply(&self, tag: &lofty::Tag, value: &RequestedTags) -> bool;
    fn apply(&self, tag: &mut lofty::Tag, value: &RequestedTags) -> Result<(), String>;
}
