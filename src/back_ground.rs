use crate::{GameState, back_ground, physics::Velocity, trash::Trash};

use bevy::{prelude::*, transform};

use rand::{thread_rng, Rng};

pub struct Screen;


pub struct BackgroundTimer(pub Timer);
pub struct  BackGroundPlugin;
impl  Plugin for BackGroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
          .add_system_set(SystemSet::on_enter(GameState::Over).with_system(setTitle.system()))
          .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setTitle.system()))

        .add_startup_system(setup.system()).add_system(gen_cloud.system()).insert_resource(BackgroundTimer(Timer::from_seconds(2.0, true)));
    }
}

fn setTitle(
    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let start_texture_handle = asset_server.load("SpaceToStart.png");
    commands
        // Start Screen
        .spawn_bundle(SpriteBundle {
            material: materials.add(start_texture_handle.into()),
            transform:Transform{
                translation:Vec3::new(0.0, 100.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Screen);
}
fn setup(
    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,

) {
    let texture_handle1 = {
       asset_server.load("cloud_1.png")
       
    };

    let texture_handle2 = {
        asset_server.load("cloud_2.png")
     };
 
     let mut rng = thread_rng();

     let  rand_scale1 = rng.gen_range(1.0..6.0);

     let  rand_scale2 = rng.gen_range(1.0..6.0);
     
    commands.spawn_bundle(SpriteBundle {
        material:materials.add(texture_handle1.into()) ,
        
        transform:Transform{
            translation:Vec3::new( rng.gen_range(-300.0..300.0),  rng.gen_range(-300.0..300.0),1.0),
            scale:Vec3::new(rand_scale1,rand_scale1,0.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Velocity(Vec2::new(
        -10.0,
        0.0,
    )));

    commands.spawn_bundle(SpriteBundle {
        material:materials.add(texture_handle2.into()) ,
        transform:Transform{
            translation:Vec3::new( rng.gen_range(-300.0..300.0),  rng.gen_range(-300.0..300.0),1.0),
            scale:Vec3::new(rand_scale2,rand_scale2,0.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Velocity(Vec2::new(
        -10.0,
        0.0,
    )));

}

fn gen_cloud(
    time: Res<Time>,

    mut commands: Commands,
    windows : Res<Windows<>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut ground_timer: ResMut<BackgroundTimer>,

) {
    let mut rng = thread_rng();
    let texture_handle = match rng.gen_bool(0.5) {
        true => asset_server.load("cloud_1.png"),
        false => asset_server.load("cloud_2.png"),
    };

    // background+
    ground_timer.0.tick(time.delta());

    if ground_timer.0.finished() {
           let  rand_scale = rng.gen_range(1.0..10.0);
           let  rand_speed = rng.gen_range(-10.00..-1.0);

          let window = windows.get_primary().unwrap();        
            commands.spawn_bundle(SpriteBundle {
                material:materials.add(texture_handle.into()) ,
                transform:Transform{
                    translation:Vec3::new(window.width()/2.0+300.0,rng.gen_range( -window.height()/2.0+100.0..window.height()/2.0),1.0),
                    scale:Vec3::new(rand_scale,rand_scale,rand_scale),
                    ..Default::default()
                },
                ..Default::default()
            }
            ).insert(Velocity(Vec2::new(
                rand_speed,
                0.0,
            ))).insert(Trash);
    }
}