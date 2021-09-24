enum  GameState {
    menu,
    playing,
    over
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(setup.system());
            
    }
}


