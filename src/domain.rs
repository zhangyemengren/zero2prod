//! src/domain.rs
use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(String);

impl SubscriberName{
    pub fn parse(s: String) -> Self{
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contain_forbidden_characters = s.chars().any(|g| {
            forbidden_characters.contains(&g)
        });
         if is_empty_or_whitespace || is_too_long || contain_forbidden_characters {
             panic!("Invalid subscriber name.")
         } else {
             Self(s)
         }
    }
    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}