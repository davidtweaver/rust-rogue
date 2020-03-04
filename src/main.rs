use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

#[macro_use]
extern crate specs_derive;

mod components;
pub use components::{Position, Renderable, Player};
mod player;
use player::*;
mod map;
use map::*;

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        //let mut lw = LeftWalker{};
        //lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();


        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}


fn main() {

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    let mut gs = State {
        ecs: World::new()
    };

    // add map
    gs.ecs.insert(new_map());
    
    // register our components with the ECS
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // create an entity which uses these components
    gs.ecs
    .create_entity()
    .with(Position { x: 40, y: 25 })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(Player{})
    .build();

    rltk::main_loop(context, gs);
}
