extern crate specs;
use specs::prelude::*;
use super::{CombatStats, IntentToMelee, Name, SufferDamage};
use rltk::console;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, IntentToMelee>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut intent_to_melee, names, combat_stats, mut inflict_damage) = data;

        for (_entity, intent_to_melee, name, stats) in (&entities, &intent_to_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(intent_to_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(intent_to_melee.target).unwrap();

                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        console::log(&format!("{} is unable to hurt {}", &name.name, &target_name.name));
                    } else {
                        console::log(&format!("{} hits {}, for {} hp.", &name.name, &target_name.name, damage));
                        SufferDamage::new_damage(&mut inflict_damage, intent_to_melee.target, damage);
                    }
                }
            }
        }

        intent_to_melee.clear();
    }
}