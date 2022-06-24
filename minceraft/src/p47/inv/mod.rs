mod enchant;
mod item;
pub type Slot = crate::inv::Slot<item::Item, enchant::Enchant>;
