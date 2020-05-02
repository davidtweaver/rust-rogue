use specs::prelude::*;
use super::{Map, IntentToPickUpItem, IntentToUseItem, IntentToDropItem, Consumable, AddHealth, InflictDamage, SufferDamage, CombatStats, Name, InInventory, Position, gamelog::GameLog};

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
                        ReadExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, IntentToUseItem>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Consumable>,
                        ReadStorage<'a, AddHealth>,
                        WriteStorage<'a, CombatStats>,
                        ReadStorage<'a, InflictDamage>,
                        WriteStorage<'a, SufferDamage>
                      );

        fn run(&mut self, data : Self::SystemData) {
            let (player_entity, mut gamelog, map, entities, mut intent_to_use, names, consumables, healing, mut combat_stats, inflict_damage, mut suffer_damage ) = data;
    
            for (entity, useitem, stats) in (&entities, &intent_to_use, &mut combat_stats).join() {
                let consumable = consumables.get(useitem.item);
                
                // healing logic
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

                // item damange logic
                let item_damages = inflict_damage.get(useitem.item);
                match item_damages {
                    None => {}
                    Some(damage) => {
                        let target_point = useitem.target.unwrap();
                        let idx = map.xy_idx(target_point.x, target_point.y);
                        let mut used_item = false;
                        for mob in map.tile_content[idx].iter() {
                            SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                            if entity == *player_entity {
                                let mob_name = names.get(*mob).unwrap();
                                let item_name = names.get(useitem.item).unwrap();
                                gamelog.entries.push(format!("You use {} on {}, inflicting {} hp.", item_name.name, mob_name.name, damage.damage));
                            }
                
                            used_item = true;
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