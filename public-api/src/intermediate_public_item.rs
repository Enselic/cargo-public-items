use rustdoc_types::{Id, Item};

use crate::nameable_item::NameableItem;
use crate::path_component::PathComponent;
use crate::public_item::PublicItemPath;
use crate::render::RenderingContext;
use crate::tokens::Token;

/// This struct represents one public item of a crate, but in intermediate form.
/// Conceptually it wraps a single [`Item`] even though the path to the item
/// consists of many [`Item`]s. Later, one [`Self`] will be converted to exactly
/// one [`crate::PublicItem`].
#[derive(Clone, Debug)]
pub struct IntermediatePublicItem<'c> {
    path: Vec<PathComponent<'c>>,
    parent_id: Option<Id>,
    id: Id,
}

impl<'c> IntermediatePublicItem<'c> {
    pub fn new(path: Vec<PathComponent<'c>>, parent_id: Option<Id>, id: Id) -> Self {
        Self {
            path,
            parent_id,
            id,
        }
    }

    #[must_use]
    pub fn item(&self) -> &'c Item {
        self.path()
            .last()
            .expect("path must not be empty")
            .item
            .item
    }

    #[must_use]
    pub fn path(&self) -> &[PathComponent<'c>] {
        &self.path
    }

    #[must_use]
    pub fn parent_id(&self) -> Option<Id> {
        self.parent_id
    }

    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }

    /// See [`crate::item_processor::sorting_prefix()`] docs for an explanation why we have this.
    #[must_use]
    pub fn sortable_path(&self, context: &RenderingContext) -> PublicItemPath {
        self.path()
            .iter()
            .map(|p| NameableItem::sortable_name(&p.item, context))
            .collect()
    }

    #[must_use]
    pub fn path_contains_renamed_item(&self) -> bool {
        self.path().iter().any(|m| m.item.overridden_name.is_some())
    }

    pub fn render_token_stream(&self, context: &RenderingContext) -> Vec<Token> {
        context.token_stream(self)
    }
}
