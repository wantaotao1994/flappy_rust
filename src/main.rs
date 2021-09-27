

mod back_ground;
mod bird;
mod ground;
mod physics;
mod pipe;
mod trash;
mod collider;
mod game_data;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

use bird::*;
use back_ground::*;
use collider::ColliderPlugin;
use ground::*;
use physics::*;
use pipe::*;
use trash::*;
use game_data::*;
/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
         .add_state(GameState::Menu)
         .add_plugin(PipePlugin)
         .add_plugin(GroundPlugin)
         .add_plugin(BackGroundPlugin)
         .add_plugin(BirdPlugin)
         .add_plugin(PhysicsPlugin)
         .add_plugin(TrashPlugin)
         .add_plugin(ColliderPlugin)
         .add_plugins(DefaultPlugins)
         .insert_resource(ClearColor(Color::rgb(0.34, 0.75, 0.79)))
         .insert_resource(GameScrore(0))
        .run();
}


