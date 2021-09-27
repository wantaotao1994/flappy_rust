#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    Over,
}




pub struct  GameScrore(pub u32);