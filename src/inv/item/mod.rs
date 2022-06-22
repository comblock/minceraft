use std::fmt::Debug;

use super::enchant::Enchant;

///  The trait `Item` represents an item type.
/// You would implement this by generating one big enum (by minecraft-data from prismarineJS) with all of the items in the version you're targetting.
pub trait Item: Sized + Debug {
    fn id(&self) -> u16;
    fn from_id(id: u16) -> anyhow::Result<Self>;
    fn name(&self) -> &'static str;
    fn from_name(name: &str) -> anyhow::Result<Self>;
    fn display_name(&self) -> &'static str;
    fn stack_size(&self) -> u32;
    fn durability(&self) -> Option<u16>;
}

#[derive(Debug, Clone, Copy)]
pub struct Itemstack<T: Item, U: Enchant> {
    pub item: T,
    pub count: i8,
    pub meta: Option<ItemStackMetaData<U>>,
}

impl<T: Item, U: Enchant> Itemstack<T, U> {
    /// Returns a new `ItemStack`
    pub fn new(item: T, count: i8) -> anyhow::Result<Self> {
        Ok(Self {
            item,
            count,
            meta: None,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemStackMetaData<T> {
    pub a: T, // hack to get the compiler to compile this
}
