
use bevy::prelude::*;
pub struct  Trash;

pub struct TrashSetting{
    pub out_to_trash: f32,
}

pub struct TrashPlugin;

impl Plugin for TrashPlugin {
    fn build(&self, app: &mut AppBuilder) {
            app.add_system(setup.system()).insert_resource(TrashSetting{
                out_to_trash:300.0,
                
            });
    }
}



fn setup(mut sprites: Query<(Entity,&mut Transform, &Trash)>,
        setting :Res<TrashSetting>,
        mut commands: Commands,
        windows : Res<Windows>
    ) {
    let window = windows.get_primary().unwrap();
    let half_window_width = window.width()/2.0;
    for (entity, transform,_) in sprites.iter_mut() {
            if transform.translation.x < -half_window_width - setting.out_to_trash {
                commands.entity(entity).despawn_recursive();
            }
    }
}
