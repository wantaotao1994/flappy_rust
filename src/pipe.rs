use std::time::Duration;

use bevy::{asset::LoadState, ecs::entity, prelude::*};
use rand::{Rng, thread_rng};

use crate::{GameState, collider::Collider, physics::Velocity, trash::Trash};


pub struct  Pipe;
struct PipeTimer(Timer);

pub struct GameSetting{
    pub min_pipe_distance: f32,
    pub max_pipe_distance: f32,

    pub max_center_delta: f32,
    pub min_center_delta: f32,

    pub move_speed :f32,

    pub half_pipe_length :f32,
    pub pipe_bottom_padding :f32,
    pub min_time_gen_pipe :f32,
    pub max_time_gen_pipe :f32

}


pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut AppBuilder) {
            app
            .add_startup_system(setup.system())

            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup.system()))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(gen_pipe.system())).insert_resource(PipeTimer(Timer::from_seconds(2.0, true))).insert_resource(GameSetting{
                min_pipe_distance:150.0,
                max_pipe_distance: 300.0,
                max_center_delta: 500.0,
                min_center_delta: 300.0,
                move_speed: 5.0,
                half_pipe_length:820.0 *0.5,
                pipe_bottom_padding:50.0,
                min_time_gen_pipe : 0.9,
                max_time_gen_pipe:3.0,
            });
    }
}

fn  setup(  time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut ground_timer: ResMut<PipeTimer>,
    setting: Res<GameSetting>,) {
    
        let texture_handle = asset_server.load("pipe.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(148.0, 820.0), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.into(),
            sprite: TextureAtlasSprite::new(1),
        
            transform:Transform{
                translation:Vec3::new(0.0,0.0,-10.0),
            
                ..Default::default()
            },
            ..Default::default()
        }
        ).insert(Pipe);

}
fn cleanup(
    mut commands: Commands,
    mut pipe_query: Query<(Entity,&mut Transform, &Pipe,&Collider)>,
) {
    for (entity,_,_,_) in pipe_query.iter_mut() {
        commands.entity(entity).despawn();

    }
}
fn gen_pipe(
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    
    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut ground_timer: ResMut<PipeTimer>,
    setting: Res<GameSetting>,

) {
    let mut rng = thread_rng();

    let texture_handle = asset_server.load("pipe.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(148.0, 820.0), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);



    
    // background+
    ground_timer.0.tick(time.delta());

    if ground_timer.0.finished()  {
            let window = windows.get_primary().unwrap();        
            
            let nex_time = rng.gen_range(setting.min_time_gen_pipe..setting.max_time_gen_pipe);
            
            ground_timer.0.set_duration(Duration::from_secs_f32(nex_time));

            let  half_window_height = window.height()/2.0;
            let half_distance = rng.gen_range(setting.min_pipe_distance..setting.max_pipe_distance)/2.0;

            let center = rng.gen_range(-half_window_height+100.0..half_window_height-100.0);


            
            let distance_pipe_window =  setting.half_pipe_length*2.0-half_window_height;


            
            let top_offset_y =  half_window_height-setting.half_pipe_length  + distance_pipe_window +center+half_distance;
            let bottom_offset_y =  -half_window_height+setting.half_pipe_length- distance_pipe_window +center-half_distance;
            

            
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone().into(),
                sprite: TextureAtlasSprite::new(0),

                transform:Transform{
                    translation:Vec3::new(window.width()/2.0+300.0,bottom_offset_y,10.0),
                    ..Default::default()
                },
                ..Default::default()
            }
            ).insert(Velocity(Vec2::new(
                -setting.move_speed,
                0.0,
            ))).insert(Trash).insert(Collider::Solid).insert(Pipe);
    

            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.into(),
                sprite: TextureAtlasSprite::new(1),
            
                transform:Transform{
                    translation:Vec3::new(window.width()/2.0+300.0,top_offset_y,10.0),
                
                    ..Default::default()
                },
                ..Default::default()
            }
            ).insert(Velocity(Vec2::new(
                -setting.move_speed,
                0.0,
            ))).insert(Trash).insert(Collider::Solid).insert(Pipe);
            
    }
}