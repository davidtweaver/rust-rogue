extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Position, Map, NPC, Name};
extern crate rltk;
use rltk::{field_of_view, Point, console};

pub struct HostileAI {}

impl<'a> System<'a> for HostileAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, NPC>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>);


    // this can probably be abstracted to a general ai system
    fn run(&mut self, data : Self::SystemData) {
        let (mut map, player_pos, mut viewshed, npc, name, mut position) = data;

        for (mut viewshed, _npc, name, mut pos) in (&mut viewshed, &npc, &name, &mut position).join() {
            
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // Attack goes here
                console::log(&format!("{} shouts insults", name.name));
                return;
            }
            
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} muses aloud", name.name));
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