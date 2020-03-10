extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Position, Map, NPC};
extern crate rltk;
use rltk::{field_of_view, Point, console};

pub struct HostileAI {}

impl<'a> System<'a> for HostileAI {
    type SystemData = ( ReadStorage<'a, Viewshed>, 
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, NPC>);

    fn run(&mut self, data : Self::SystemData) {
        let (viewshed, pos, npc) = data;

        for (viewshed,pos,_npc) in (&viewshed, &pos, &npc).join() {
            console::log("NPC considers their own existence");
        }
    }
}