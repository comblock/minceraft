pub mod enchant;
pub mod item;

#[derive(Debug, Clone, Copy)]
pub enum Slot<T: item::Item, U: enchant::Enchant> {
    Empty,
    Filled(item::Itemstack<T, U>),
}
