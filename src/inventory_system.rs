use specs::prelude::*;
use super::{IntentToPickUpItem, IntentToUseItem, IntentToDropItem, Consumable, AddHealth, CombatStats, Name, InInventory, Position, gamelog::GameLog};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, IntentToPickUpItem>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, InInventory>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, mut intent_pickup, mut positions, names, mut backpack) = data;

        for pickup in intent_pickup.join() {
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InInventory{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                gamelog.entries.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }

        intent_pickup.clear();
    }
}

// todo: make this generic later on for any healing item.
pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, IntentToUseItem>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Consumable>,
                        ReadStorage<'a, AddHealth>,
                        WriteStorage<'a, CombatStats>
                      );

        fn run(&mut self, data : Self::SystemData) {
            let (player_entity, mut gamelog, entities, mut intent_to_use, names, consumables, healing, mut combat_stats) = data;
    
            for (entity, useitem, stats) in (&entities, &intent_to_use, &mut combat_stats).join() {
                let consumable = consumables.get(useitem.item);
                
                let item_heals = healing.get(useitem.item);
                match item_heals {
                    None => {}
                    Some(healer) => {
                        stats.hp = i32::min(stats.max_hp, stats.hp + healer.heal_amount);
                        if entity == *player_entity {
                            gamelog.entries.push(format!("You drink the {}, healing {} hp.", names.get(useitem.item).unwrap().name, healer.heal_amount));
                        }
                    }
                }

                match consumable {
                    None => {}
                    Some(_) => {
                        entities.delete(useitem.item).expect("Delete failed");
                }       
            }
        }

        intent_to_use.clear();
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, IntentToDropItem>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, InInventory>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_drop, names, mut positions, mut backpack) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos : Position = Position{x:0, y:0};
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions.insert(to_drop.item, Position{ x : dropper_pos.x, y : dropper_pos.y }).expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!("You drop the {}.", names.get(to_drop.item).unwrap().name));
            }
        }

        wants_drop.clear();
    }
}