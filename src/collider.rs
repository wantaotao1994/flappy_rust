use bevy::{prelude::*};


use bevy::sprite::collide_aabb::collide;

use crate::bird::Player;

use crate::game_data::GameScrore;
use crate::physics::Velocity;
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
   mut  game_score:ResMut<GameScrore>,
    mut score_query: Query<(&mut ScoreSprite, &Collider,&Transform,&Sprite)>,
    player_query: Query<(&Player, &Collider, &Handle<TextureAtlas> ,&Transform)>,
   mut pipe_query: Query<(&Pipe,&Collider, &Handle<TextureAtlas> ,&Transform, &mut Velocity)>,

){
    let window = windows.get_primary().unwrap();

    let half_windows_height = window.height()/2.0;



    if let Ok(( _,_,textrue,player_trans_form)) = player_query.single(){
       let  textrue = texture_atlases.get(textrue).unwrap();

       for (mut score_sprite,_,trans_from,sprite) in score_query.iter_mut() {
           if score_sprite.0 {
               continue;
           }

           if  trans_from.translation.x+sprite.size.x/2.0 < player_trans_form.translation.x - textrue.size.x/2.0  {
                game_score.0+= 1;
                println!("Got 1 Score , Now you have {:?}",game_score.0);
                score_sprite.0= true;
           }

       }


       if  player_trans_form.translation.y  > half_windows_height +30.0 || player_trans_form.translation.y  < -half_windows_height -30.0{
               state.set(GameState::Over).unwrap();
                return;
        }
        let colision_size_player = textrue.size*0.8;   //scal player's size

       for (_,_, pipe_textrue_handle,pipe_trans_form, _) in pipe_query.iter_mut() {
    
        let  pipe_textrue = texture_atlases.get(pipe_textrue_handle).unwrap();
        let mut colision_size_pipe = pipe_textrue.size;

        colision_size_pipe.x *=0.5;
        let collision  =collide(
            pipe_trans_form.translation,
            colision_size_pipe,
            player_trans_form.translation,
            colision_size_player, 
            );
        if collision.is_some() {
              state.set(GameState::Over).unwrap();
        }

       }
     
    }
}

