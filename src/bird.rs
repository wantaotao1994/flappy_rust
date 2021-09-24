
use std::default;

use crate::{GameState, back_ground::Screen, collider::Collider, physics::{Gravity, Velocity}};

use bevy::{prelude::*};
use crate::bird;

pub struct  BirdPlugin;
impl  Plugin for BirdPlugin {
    fn build(&self, app: &mut AppBuilder) {
            app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup.system()))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup.system()))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(begin.system()))
            .add_system_set(SystemSet::on_enter(GameState::Over).with_system(begin.system()))

            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(handle_fly_system.system()))

            .add_system(animate_sprite_system.system()).add_system(handle_system.system())
            .insert_resource(BirdStaus{
                is_flying:true,
            });
    }
}

pub struct  JumpSpeed(f32);
pub struct  Player();

pub struct BirdStaus{
    is_flying:bool
}
pub struct AngleConstans {
    pub angle_up: f32,
    pub angle_down: f32,

    pub velocity_max: f32,
}


fn begin(
    mut birdStatus :ResMut<BirdStaus>,
    mut  query: Query<(&mut Player,  &mut Velocity,&mut Transform)>,
)  {
    birdStatus.is_flying = true;
    if let Ok((_,mut vel,mut tras)) =  query.single_mut(){

        vel.0 = Vec2::ZERO;
        tras.translation = Vec3::new(0.0, 0.0, 100.0);
        tras.rotation = Quat::from_rotation_z(0.0);

    }
}
fn cleanup()  {
    
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
   // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let texture_handle = asset_server.load("bird.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform:Transform{
                translation:Vec3::new(0.0,0.0,100.00),
                scale:Vec3::new(2.0, 2.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true)).insert(Gravity(7.0))
        
        .insert(Velocity(Vec2::new(0.0, 0.0))).insert(JumpSpeed(5.0)).insert(Player{
        }).insert(AngleConstans{
            angle_up: std::f32::consts::PI * 0.5 * 0.7,
            angle_down: -std::f32::consts::PI * 0.5 * 0.5,
            velocity_max: 400.0,
        }).insert(Collider::Solid);

        
}

fn handle_fly_system(
   mut birdStatus :ResMut<BirdStaus>,
    mut  query: Query<(&mut Player,  &Velocity,&AngleConstans,&mut Transform,&Gravity,&JumpSpeed)>,
) {
      if let Ok( (mut player, velocity,angle,mut transform,gravity,speed)) = query.single_mut(){
        if velocity.0.y >= 0.0 {  //up
           let angle = (velocity.0.y/speed.0 )* angle.angle_up;
           transform.rotation =  Quat::from_rotation_z( angle);
           birdStatus.is_flying = true;

        }else {
            birdStatus.is_flying = false;

            let mut  angle_num = (-velocity.0.y/gravity.0 )* angle.angle_down;
            if angle_num< angle.angle_down{
                angle_num = angle.angle_down;
            }
            transform.rotation =  Quat::from_rotation_z( angle_num);
        }
      }
}


fn handle_system(
    mut state: ResMut<State<GameState>>,
    mut  screen_query: Query<(&Screen,Entity)>,
    mut commands: Commands,

    keyboard_input: Res<Input<KeyCode>>,
    mut  query: Query<(&Player, &mut Velocity,&JumpSpeed,&mut Transform)>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
   
     match state.current() {
        GameState::InGame => {
            let   (_,mut velocity,speed,mut trans) = query.single_mut().unwrap();
            velocity.0.y= speed.0;
            trans.rotation = Quat::from_rotation_z(0.0);
        },
        _default => {
            
            let scrren = screen_query.single_mut().unwrap();
            commands.entity(scrren.1).despawn();
            state.set(GameState::InGame);
        },
    }
}
fn animate_sprite_system(
    time: Res<Time>,
    birdStatus : ResMut<BirdStaus>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                if !birdStatus.is_flying{
                    sprite.index = 1;  //hard code 
                    return;
                }
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}