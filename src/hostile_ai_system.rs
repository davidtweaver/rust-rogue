use specs::prelude::*;
use super::{Viewshed, Position, Map, NPC, Name, IntentToMelee, RunState, Confusion};
use rltk::{field_of_view, Point, console};

pub struct HostileAI {}

impl<'a> System<'a> for HostileAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, NPC>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, IntentToMelee>,
                        WriteStorage<'a, Confusion>);


    // this can probably be abstracted to a general ai system
    fn run(&mut self, data : Self::SystemData) {
       
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, npc, mut position, mut intent_to_melee, mut confused) = data;

        if *runstate != RunState::AITurn { return; }
        
        for (entity, mut viewshed,_npc,mut pos) in (&entities, &mut viewshed, &npc, &mut position).join() {
            
            let mut can_act = true;
            let is_confused = confused.get_mut(entity);

            // confusion currently makes an npc skip x turns
            if let Some(npc_confused) = is_confused {
                npc_confused.turns -= 1;
                if npc_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;
            }

            if can_act {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    // Attack goes here
                    intent_to_melee.insert(entity, IntentToMelee{ target: *player_entity }).expect("Unable to insert attack");
                    return;
                }
                
                if viewshed.visible_tiles.contains(&*player_pos) {
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &mut *map
                    );
                    if path.success && path.steps.len()>1 {
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}