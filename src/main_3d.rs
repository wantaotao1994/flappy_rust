use std::vec;

use bevy::{ecs::query, prelude::*, render::{camera::Camera, render_graph::base::camera::CAMERA_3D}};

#[derive(Debug)]
struct Game {

    current_localtion:Vec3,
    looking_for:Vec3,
    cameraUp   :Vec3,
    lastCurPosition : Vec2,
     pitch : f32,
     yaw   :f32,
}
const CAMERA_SPEED: f32 = 0.05;
const sensitivity:f32 = 0.05;



fn main() {
    App::build()
        .insert_resource(Game{
            current_localtion: Vec3::new(0.0, 0.5, 5.0),
            looking_for: Vec3::new(0.0, 0.0, -1.0),
            cameraUp: Vec3::Y,
            lastCurPosition:Vec2::ZERO,
            pitch :  0.0,
            yaw   :-90.0,
        })

        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_camera_system.system())

        .run();
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut  game : ResMut<Game>,

    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let window = windows.get_primary_mut().unwrap();

    let  curX  =  window.width()/2.0;
    let curY = window.height()/2.0;
    window.set_cursor_position(Vec2::new(curX, curY));
    window.set_cursor_visibility(false);
    game.lastCurPosition =Vec2::new(curX, curY);
    
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

 
    

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.9, 0.5, 0.0),
        ..Default::default()
    });


    
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.9, 1.0, 1.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 4.0),
        ..Default::default()
    });


    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(game.current_localtion).looking_at( game.looking_for, game.cameraUp),
        ..Default::default()
    });

    commands.spawn_bundle(UiCameraBundle::default());

}


// System for rotating and translating the camera
fn move_camera_system(time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut  game : ResMut<Game>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut transforms: QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
) {

    let cross = Vec3::normalize(game.looking_for.cross(game.cameraUp));
    if keyboard_input.pressed(KeyCode::A) {

        game.current_localtion  -= cross * CAMERA_SPEED;

    }
    if keyboard_input.pressed(KeyCode::S) {
        game.current_localtion  =  game.current_localtion-  game.looking_for*CAMERA_SPEED;

    }
    if keyboard_input.pressed(KeyCode::D) {
        game.current_localtion  += cross * CAMERA_SPEED;

    }
    if keyboard_input.pressed(KeyCode::W) {
        game.current_localtion  = game.current_localtion+ game.looking_for*CAMERA_SPEED;

    }

    let mut  xOffset :f32 = 0.0;
    let mut  yOffset :f32 = 0.0;
    for event in cursor_moved_events.iter() {
        xOffset+= event.position.x- game.lastCurPosition.x;
        yOffset += event.position.y-game.lastCurPosition.y;

        xOffset*=sensitivity;
        yOffset*=sensitivity;
        
        game.lastCurPosition =event.position;
    }

    game.yaw   += xOffset;
    game.pitch += yOffset;

    if game.pitch>89.0 {
        game.pitch = 89.0;

    }else if game.pitch< -89.0 {
        game.pitch = -89.0;
    }

    game.looking_for.x=game.yaw.to_radians().cos()*game.pitch.to_radians().cos();
    
    game.looking_for.y=game.pitch.sin();
    game.looking_for.z=game.yaw.to_radians().sin()*game.pitch.to_radians().cos();

   game.looking_for.normalize();
    
    
    // look at that new camera's actual focus
    for (mut transform, camera) in transforms.q0_mut().iter_mut() {
        if camera.name == Some(CAMERA_3D.to_string()) {
            *transform = Transform {
                translation: game.current_localtion,
                ..Default::default()
            };
            *transform = transform.looking_at(game.current_localtion+game.looking_for, game.cameraUp);
        }
    }

   // *camera_trans =  Transform::from_xyz(1.0, 2.0, 2.0).looking_at( game.current_localtion, Vec3::Y)
}