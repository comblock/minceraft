pub mod enchant;
pub mod item;

#[derive(Debug, Clone, Copy)]
pub enum Slot<I: item::Item, E: enchant::Enchant> {
    Empty,
    Filled(item::Itemstack<I, E>),
}
