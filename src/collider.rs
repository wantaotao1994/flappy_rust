

use std::os::windows;

use bevy::{prelude::*, transform, window};
use bevy::sprite::collide_aabb::collide;

use crate::bird::Player;

use crate::{GameState, pipe::*};

pub enum Collider {
    Solid,
    ScoreGiver,
}


pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(setup.system());
    }
}

fn  setup(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut state: ResMut<State<GameState>>,
    windows:Res<Windows>,
    mut  player_query: Query<(&Player, &Collider, &Handle<TextureAtlas> ,&Transform)>,
    mut  pipe_query: Query<(&Pipe, &Collider, &Handle<TextureAtlas> ,&Transform)>,

){
    let window = windows.get_primary().unwrap();

    let half_windows_height = window.height()/2.0;



    if let Ok(( _,_,textrue,player_transForm)) = player_query.single_mut(){
       let  textrue = texture_atlases.get(textrue).unwrap();

       if  player_transForm.translation.y  > half_windows_height +30.0 || player_transForm.translation.y  < -half_windows_height -30.0{
                state.set(GameState::Over);

                return;
        }
       
       for (_,_, pipe_textrue_handle,pipe_transForm) in pipe_query.iter() {
        let  pipe_textrue = texture_atlases.get(pipe_textrue_handle).unwrap();

        let collision  =collide(
                player_transForm.translation,
              textrue.size,
              pipe_transForm.translation,
              pipe_textrue.size

            );



        if collision.is_some() {
                state.set(GameState::Over);
        }
       }
     
    }
}

