use bevy::{prelude::*, tasks::ComputeTaskPool};

use crate::GameState;

pub struct Velocity(pub Vec2);

pub struct Gravity(pub f32);


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
          app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(gravity_system.system())).add_system(velocity_system.system());
        
    }
}

fn gravity_system(mut sprites: Query<(&Gravity,&mut Velocity)>,time: Res<Time>) {
    for (mut gravity,mut velocity) in sprites.iter_mut() {
        velocity.0.y -= gravity.0 * time.delta_seconds();
    }
}
fn velocity_system(mut sprites: Query<(&mut Transform, &Velocity)>) {
    for (mut transform,velocity) in sprites.iter_mut() {
        transform.translation +=  velocity.0.extend(0.0)
    }
}
