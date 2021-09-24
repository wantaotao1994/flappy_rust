use std::os::windows;

use crate::{physics::Velocity};


use bevy::{prelude::*, transform};
use rand::{thread_rng, Rng};
use crate:: trash::*;

pub struct GroundTimer(pub Timer);
pub struct  GroundPlugin;
impl  Plugin for GroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_system(gen_mountain.system())
        
        
        .insert_resource(GroundTimer(Timer::from_seconds(3.0, true)));
    }
}




fn setup(
    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,

) {
    let window = windows.get_primary().unwrap();        

    let texture_handle = {
       asset_server.load("mountain.png")
       
    };
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(ColorMaterial::modulated_texture(
            texture_handle.clone().into(),
            Color::rgb(0.36, 0.36, 0.36),
        )),

        
        transform:Transform{
            translation:Vec3::new(window.width()/2.0+100.0, -window.height()/2.0-100.0,1.0),
            scale:Vec3::new(1.0,1.0,1.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Velocity(Vec2::new(
        -10.0,
        0.0,
    )));

}

fn gen_mountain(
    time: Res<Time>,

    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut ground_timer: ResMut<GroundTimer>,

) {
    let mut rng = thread_rng();
    let texture_handle = match rng.gen_bool(0.5) {
        true => asset_server.load("mountain.png"),
        false => asset_server.load("mountain.png"),
    };

    // background+
    ground_timer.0.tick(time.delta());

    if ground_timer.0.finished() {
          let window = windows.get_primary().unwrap();        
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(ColorMaterial::modulated_texture(
                    texture_handle.clone().into(),
                    Color::rgb(0.36, 0.36, 0.36),
                )),
    
                transform:Transform{
                    translation:Vec3::new(window.width()/2.0+300.0,rng.gen_range( -window.height()/2.0-100.0..-window.height()/2.0),1.0),
                    scale:Vec3::new(1.0,1.0,1.0),
                    ..Default::default()
                },
                ..Default::default()
            }
            ).insert(Velocity(Vec2::new(
                -5.0,
                0.0,
            ))).insert(Trash);
        
    
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(ColorMaterial::modulated_texture(
                texture_handle.clone().into(),
                Color::rgb(0.26, 0.26, 0.26),
            )),           
            transform:Transform{
                translation:Vec3::new(window.width()/2.0+300.0,rng.gen_range( -window.height()/2.0-100.0..-window.height()/2.0),2.0),
                scale:Vec3::new(1.0,1.0,1.0),
                ..Default::default()
            },
            ..Default::default()
        }
        ).insert(Velocity(Vec2::new(
            -8.0,
            0.0,
        ))).insert(Trash);
    }
}