

mod back_ground;
mod bird;
mod ground;
mod physics;
mod pipe;
mod trash;
mod collider;

use std::time::Duration;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
};
use bevy::sprite::collide_aabb::{collide, Collision};
use bird::*;
use back_ground::*;
use collider::ColliderPlugin;
use ground::*;
use physics::*;
use pipe::*;
use trash::*;
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

        //  .init_resource::<ButtonMaterials>()

        //  .add_state(GameState::Menu)
        //  .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
        //  .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu.system()))
        //  .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu.system()))

        // .add_system_set(SystemSet::on_enter(GameState::InGame)
        //         )
           

        // .add_system_set(SystemSet::on_update(GameState::InGame)
        //         .with_system(input_system.system())
        //         .with_system(ball_movement_system.system())

        //     )

        .run();
}



#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    InGame,
    Over,
}

struct MenuData {
    button_entity: Entity,
}


fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),

                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}



fn menu(
    mut state: ResMut<State<GameState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(GameState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}

// fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&mut Bird, &mut Transform)>) {
//     if let Ok((mut ball, mut transform)) = ball_query.single_mut() {
//         let delta_seconds = time.time_since_startup()- ball.lastInputTime;
//         let delta_seconds = delta_seconds.as_secs_f32();

//         if ball.isFlying {
//             let speed =  ball.jumSpeed- ball.gravity * delta_seconds;      
//             if speed <=0.0 {   //
//                 ball.lastInputTime = time.time_since_startup();
//                 ball.lastInputY   = transform.translation.y;
//                 ball.isFlying = false;
//             }else {
//                 transform.translation.y = ball.lastInputY + ball.jumSpeed*delta_seconds - ball.gravity * delta_seconds*delta_seconds *0.5;
//             }
//         }else {
//             transform.translation.y =  ball.lastInputY +-1.0 * ball.gravity * delta_seconds*delta_seconds *0.5;
//         }

//     }
    
// }

// fn input_system(time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,

//     mut ball_query: Query<(&mut Bird, &mut Transform)>){
//         if keyboard_input.pressed(KeyCode::Space) {
//             if let Ok((mut ball, mut transform)) = ball_query.single_mut() {
//                 ball.isFlying = true;

//                 ball.lastInputTime = time.time_since_startup();
//                 ball.lastInputY = transform.translation.y;
//             };
//         }
// }

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}
impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}
