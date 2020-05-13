use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;

#[derive(Component, ConvertSaveload)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order :i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct NPC {}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct Name {
    pub name : String
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct CombatStats {
    pub max_hp : i32,
    pub hp : i32,
    pub defense : i32,
    pub power : i32
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct IntentToMelee {
    pub target : Entity
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct SufferDamage {
    pub amount : Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount : vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item { }

#[derive(Component, Debug, Clone, Serialize, Deserialize,)]
pub struct Consumable { }

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct AddHealth {
    pub heal_amount : i32
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InInventory {
    pub owner : Entity
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct IntentToPickUpItem {
    pub collected_by : Entity,
    pub item : Entity
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct IntentToUseItem {
    pub item : Entity,
    pub target : Option<rltk::Point>
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct IntentToDropItem {
    pub item : Entity
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct Ranged {
    pub range : i32
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InflictDamage {
    pub damage : i32
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct AreaOfEffect {
    pub radius : i32
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct Confusion {
    pub turns : i32
}

// Special component for marking which entities to serialize
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SerializeMe;

// Special component that exists to help serialize the game data
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map : super::map::Map
}