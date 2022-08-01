#[cfg(feature = "p47")]
use case::CaseExt;
#[cfg(feature = "p47")]
use reqwest::blocking as reqwest;
#[cfg(feature = "p47")]
use std::env;
#[cfg(feature = "p47")]
use std::fs::File;
#[cfg(feature = "p47")]
use std::io::Write;
#[cfg(feature = "p47")]
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "p47")]
    generate_inv("1.8")?;
    Ok(())
}

#[cfg(feature = "p47")]
fn generate_inv(version: &str) -> anyhow::Result<()> {
    let data_path: serde_json::Value = reqwest::get(
        "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/dataPaths.json",
    )?
    .json()?;
    let out_dir = env::var("OUT_DIR")?;

    let mut enchant = Vec::<u8>::new();
    let mut item = Vec::<u8>::new();

    {
        enchant.write_all(b"// This file was generated and is not intended for manual editing\nuse crate::inv::enchant::EnchantCost;\n\n")?;

        let enchant_path = data_path["pc"][version]["enchantments"].as_str().unwrap();

        let enchants: serde_json::Value = reqwest::get(
        format!("https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/{enchant_path}/enchantments.json")
        )?.json()?;

        let enchants = enchants.as_array().unwrap();

        let mut bufs = Vec::<Vec<u8>>::new();
        let buf_amount = 18;

        for _ in 0..=buf_amount {
            bufs.push(Vec::<u8>::new());
        }

        bufs[0].write_all(b"#[derive(Debug, Copy, Clone)]\npub enum EnchantCategory {\n")?;
        bufs[1].write_all(b"impl crate::inv::enchant::EnchantCategory for EnchantCategory {\n    fn name(&self) -> &'static str {\n        match self {\n")?;
        bufs[2].write_all(
            b"\n    fn from_name(name: &str) -> anyhow::Result<Self> {\n        match name {\n",
        )?;
        bufs[3].write_all(b"#[derive(Debug, Copy, Clone)]\npub enum Enchant {\n")?;
        bufs[4].write_all(b"impl crate::inv::enchant::Enchant for Enchant {\n    type EnchantCategory = EnchantCategory;\n\n    fn id(&self) -> u16 {\n        match self {\n")?;
        bufs[5].write_all(
            b"\n    fn from_id(id: u16) -> anyhow::Result<Self> {\n        match id {\n",
        )?;
        bufs[6].write_all(b"\n    fn name(&self) -> &'static str {\n        match self {\n")?;
        bufs[7].write_all(
            b"\n    fn from_name(name: &str) -> anyhow::Result<Self> {\n        match name {\n",
        )?;
        bufs[8]
            .write_all(b"\n    fn display_name(&self) -> &'static str {\n        match self {\n")?;
        bufs[9].write_all(b"\n    fn max_lvl(&self) -> u16 {\n        match self {\n")?;
        bufs[10].write_all(b"\n    fn min_cost(&self) -> EnchantCost {\n        match self {\n")?;
        bufs[11].write_all(b"\n    fn max_cost(&self) -> EnchantCost {\n        match self {\n")?;
        bufs[12].write_all(b"\n    fn exclude(&self) -> &[Self] {\n        match self {\n")?;
        bufs[13].write_all(
            b"\n    fn category(&self) -> Self::EnchantCategory {\n        match self {\n",
        )?;
        bufs[14].write_all(b"\n    fn weight(&self) -> u16 {\n        match self {\n")?;
        bufs[15].write_all(b"\n    fn treasure_only(&self) -> bool {\n        match self {\n")?;
        bufs[16].write_all(b"\n    fn curse(&self) -> bool {\n        match self {\n")?;
        bufs[17].write_all(b"\n    fn tradeable(&self) -> bool {\n        match self {\n")?;
        bufs[18].write_all(b"\n    fn discoverable(&self) -> bool {\n        match self {\n")?;

        let mut enchant_categories = Vec::<&str>::new();
        let mut names = Vec::<&str>::new();

        for i in enchants {
            let name = i["name"].as_str().unwrap();
            names.push(name);
        }

        for i in enchants {
            let category = i["category"].as_str().unwrap();
            if !enchant_categories.contains(&category) {
                enchant_categories.push(category);
            };

            let name = i["name"].as_str().unwrap();
            let camel = name.to_camel();
            let id = i["id"].as_u64().unwrap();
            let display_name = i["displayName"].as_str().unwrap();
            let max_lvl = i["maxLevel"].as_u64().unwrap();
            let min_cost = &i["minCost"];
            let max_cost = &i["maxCost"];
            let weight = i["weight"].as_u64().unwrap();
            let treasure_only = i["treasureOnly"].as_bool().unwrap();
            let curse = i["curse"].as_bool().unwrap();
            let raw_exclude = i["exclude"].as_array().unwrap();
            let tradeable = i["tradeable"].as_bool().unwrap();
            let discoverable = i["discoverable"].as_bool().unwrap();
            let mut exclude = String::new();
            for i in raw_exclude {
                let name = i.as_str().unwrap();
                let camel = name.to_camel();

                if names.contains(&name) {
                    exclude.push_str(&format!("Self::{camel},"));
                };
            }

            bufs[3].write_all(format!("    {},\n", camel).as_bytes())?;
            bufs[4].write_all(format!("            Self::{camel} => {id},\n").as_bytes())?;
            bufs[5].write_all(format!("            {id} => Ok(Self::{camel}),\n").as_bytes())?;
            bufs[6].write_all(format!("            Self::{camel} => \"{name}\",\n").as_bytes())?;
            bufs[7]
                .write_all(format!("            \"{name}\" => Ok(Self::{camel}),\n").as_bytes())?;
            bufs[8].write_all(
                format!("            Self::{camel} => \"{display_name}\",\n").as_bytes(),
            )?;
            bufs[9].write_all(format!("            Self::{camel} => {max_lvl},\n").as_bytes())?;
            bufs[10].write_all(
                format!(
                    "            Self::{camel} => EnchantCost {{ a: {}, b: {} }},\n",
                    min_cost["a"].as_i64().unwrap(),
                    min_cost["b"].as_i64().unwrap()
                )
                .as_bytes(),
            )?;
            bufs[11].write_all(
                format!(
                    "            Self::{camel} => EnchantCost {{ a: {}, b: {} }},\n",
                    max_cost["a"].as_i64().unwrap(),
                    max_cost["b"].as_i64().unwrap()
                )
                .as_bytes(),
            )?;
            bufs[12]
                .write_all(format!("            Self::{camel} => &[{}],\n", exclude).as_bytes())?;
            bufs[13].write_all(
                format!(
                    "            Self::{camel} => EnchantCategory::{},\n",
                    category.to_camel()
                )
                .as_bytes(),
            )?;
            bufs[14].write_all(format!("            Self::{camel} => {weight},\n").as_bytes())?;
            bufs[15]
                .write_all(format!("            Self::{camel} => {treasure_only},\n").as_bytes())?;
            bufs[16].write_all(format!("            Self::{camel} => {curse},\n").as_bytes())?;
            bufs[17]
                .write_all(format!("            Self::{camel} => {tradeable},\n").as_bytes())?;
            bufs[18]
                .write_all(format!("            Self::{camel} => {discoverable},\n").as_bytes())?;
        }

        bufs[3].write_all(b"}\n\n")?;

        for i in enchant_categories {
            let camel = i.to_camel();
            bufs[0].write_all(format!("    {camel},\n").as_bytes())?;
            bufs[1].write_all(format!("            Self::{camel} => \"{i}\",\n").as_bytes())?;
            bufs[2].write_all(format!("            \"{i}\" => Ok(Self::{camel}),\n").as_bytes())?;
        }
        bufs[0].write_all("}\n\n".as_bytes())?;

        bufs[2].write_all(
            b"            _ => Err(anyhow::anyhow!(\"invalid enchant category name\"))\n",
        )?;
        bufs[5].write_all(b"            _ => Err(anyhow::anyhow!(\"invalid enchant id\"))\n")?;
        bufs[7].write_all(b"            _ => Err(anyhow::anyhow!(\"invalid enchant name\"))\n")?;

        for i in vec![1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18] {
            bufs[i].write_all(b"        }\n    }\n")?;
        }

        for i in 0..3 {
            enchant.write_all(&bufs[i])?;
        }
        enchant.write_all(b"}\n\n")?;

        for i in 3..=buf_amount {
            enchant.write_all(&bufs[i])?;
        }
        enchant.write_all(b"}\n\n")?;
        enchant.flush()?;
    }

    {
        item.write_all(b"// This file was generated and is not intended for manual editing\nuse anyhow::{anyhow, Result};\n\n#[derive(Debug, Copy, Clone)]\npub enum Item {\n")?;

        let item_path = data_path["pc"][version]["items"].as_str().unwrap();
        let resp = reqwest::get(format!("https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/{item_path}/items.json"))?;
        let json: serde_json::Value = serde_json::from_reader(resp)?;

        let mut buf = Vec::<u8>::new();
        let mut buf2 = Vec::<u8>::new();
        let mut buf3 = Vec::<u8>::new();
        let mut buf4 = Vec::<u8>::new();
        let mut buf5 = Vec::<u8>::new();
        let mut buf6 = Vec::<u8>::new();
        let mut buf7 = Vec::<u8>::new();

        buf.write_all(b"impl crate::inv::item::Item for Item {\n    fn id(&self) -> u16 {\n        match self {\n")?;
        buf2.write_all(b"    fn from_id(id: u16) -> Result<Self> {\n        match id {\n")?;
        buf3.write_all(b"    fn name(&self) -> &'static str {\n        match self {\n")?;
        buf4.write_all(b"    fn from_name(name: &str) -> Result<Self> {\n        match name {\n")?;
        buf5.write_all(b"    fn display_name(&self) -> &'static str {\n        match self {\n")?;
        buf6.write_all(b"    fn stack_size(&self) -> u32 {\n        match self {\n")?;
        buf7.write_all(b"    fn durability(&self) -> Option<u16> {\n        match self {\n")?;

        for i in json.as_array().unwrap() {
            let name = i["name"].as_str().unwrap();
            let name_camel = name.to_camel();
            let id = &i["id"];
            let display_name = &i["displayName"];
            let stack_size = &i["stackSize"];
            let max_durability = &i["maxDurability"];
            item.write_all(format!("    {},\n", name_camel).as_bytes())?;
            buf.write_all(format!("            Self::{} => {},\n", name_camel, id).as_bytes())?;
            buf2.write_all(
                format!("            {} => Ok(Self::{}),\n", id, name_camel).as_bytes(),
            )?;
            buf3.write_all(
                format!("            Self::{} => \"{}\",\n", name_camel, name).as_bytes(),
            )?;
            buf4.write_all(
                format!("            \"{}\" => Ok(Self::{}),\n", name, name_camel).as_bytes(),
            )?;
            buf5.write_all(
                format!("            Self::{} => {},\n", name_camel, display_name).as_bytes(),
            )?;
            buf6.write_all(
                format!("            Self::{} => {},\n", name_camel, stack_size).as_bytes(),
            )?;
            if max_durability != &serde_json::Value::from_str("null")? {
                buf7.write_all(
                    format!(
                        "            Self::{} => Some({}),\n",
                        name_camel, max_durability
                    )
                    .as_bytes(),
                )?;
            } else {
                buf7.write_all(format!("            Self::{} => None,\n", name_camel).as_bytes())?;
            }
        }

        item.write_all(b"}\n\n")?;
        buf.write_all(b"        }\n    }\n\n")?;
        buf2.write_all(
            b"            _ => Err(anyhow!(\"invalid item id\")),\n        }\n    }\n\n",
        )?;
        buf3.write_all(b"        }\n    }\n\n")?;
        buf4.write_all(
            b"            _ => Err(anyhow!(\"invalid item name\")),\n        }\n    }\n\n",
        )?;
        buf5.write_all(b"        }\n    }\n\n")?;
        buf6.write_all(b"        }\n    }\n\n")?;
        buf7.write_all(b"        }\n    }\n}\n")?;
        item.write_all(&buf)?;
        item.write_all(&buf2)?;
        item.write_all(&buf3)?;
        item.write_all(&buf4)?;
        item.write_all(&buf5)?;
        item.write_all(&buf6)?;
        item.write_all(&buf7)?;
        item.flush()?;
    }
    let enchant = String::from_utf8(enchant)?;
    let item = String::from_utf8(item)?;

    let mut f = File::create(format!("{out_dir}/inv.rs"))?;
    f.write(format!("pub mod enchant {{\n{enchant}}}\npub mod item{{\n{item}}}\npub type Slot = crate::inv::Slot<item::Item, enchant::Enchant>;\n").as_bytes())?;

    Ok(())
}
