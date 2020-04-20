use specs::prelude::*;
use super::{IntentToPickUpItem, IntentToUseHealingItem, AddHealth, CombatStats, Name, InInventory, Position, gamelog::GameLog};

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
pub struct PotionUseSystem {}

impl<'a> System<'a> for PotionUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, IntentToUseHealingItem>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, AddHealth>,
                        WriteStorage<'a, CombatStats>
                      );

        fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_drink, names, potions, mut combat_stats) = data;

        for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
            let potion = potions.get(drink.health_item);
            gamelog.entries.push(format!("Attempting to drink potion."));
            match potion {
                None => {}
                Some(potion) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                    if entity == *player_entity {
                        gamelog.entries.push(format!("You drink the {}, healing {} hp.", names.get(drink.health_item).unwrap().name, potion.heal_amount));
                    }
                    entities.delete(drink.health_item).expect("Delete failed");
                }
            }
        }

        wants_drink.clear();
    }
}