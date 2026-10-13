mod declarations;
mod expressions;
mod types;
mod views;
use std::ops::Deref;

use common::VisibilityModifier;
use module_loader::FileId;
pub use views::*;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Visible<T> {
    pub visibility: VisibilityModifier,
    pub data: T,
    pub definer: FileId,
}

impl<T> Visible<T> {
    pub fn new(visibility: VisibilityModifier, definer: FileId, data: T) -> Self {
        Self {
            visibility,
            data,
            definer,
        }
    }
    pub fn is_visible(&self, requester: FileId) -> bool {
        self.visibility == VisibilityModifier::Public || requester == self.definer
    }
}

impl<T> Deref for Visible<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
