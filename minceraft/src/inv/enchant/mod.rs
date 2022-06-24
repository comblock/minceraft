use std::fmt::Debug;

pub struct EnchantCost {
    pub a: i8,
    pub b: i8,
}

pub trait EnchantCategory: Sized + Debug {
    fn name(&self) -> &'static str;
    fn from_name(name: &str) -> anyhow::Result<Self>;
}

pub trait Enchant: Sized + Debug {
    type EnchantCategory: EnchantCategory;

    fn id(&self) -> u16;
    fn from_id(id: u16) -> anyhow::Result<Self>;
    fn name(&self) -> &'static str;
    fn from_name(name: &str) -> anyhow::Result<Self>;
    fn display_name(&self) -> &'static str;
    fn max_lvl(&self) -> u16;
    fn min_cost(&self) -> EnchantCost;
    fn max_cost(&self) -> EnchantCost;
    fn exclude(&self) -> &[Self];
    fn category(&self) -> Self::EnchantCategory;
    fn weight(&self) -> u16;
    fn treasure_only(&self) -> bool;
    fn curse(&self) -> bool;
    fn tradeable(&self) -> bool;
    fn discoverable(&self) -> bool;
}
