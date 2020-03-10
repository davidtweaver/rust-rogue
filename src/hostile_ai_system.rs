extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Position, Map, NPC};
extern crate rltk;
use rltk::{field_of_view, Point, console};

pub struct HostileAI {}

impl<'a> System<'a> for HostileAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>, 
                        ReadStorage<'a, NPC>);

    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, viewshed, npc) = data;

        for (viewshed, _npc) in (&viewshed, &npc).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("NPC muses aloud"));
            }
        }
    }
}