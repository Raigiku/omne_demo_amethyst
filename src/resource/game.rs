#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Gameplay
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}

pub struct GameResource {
    pub current_state: GameState
}

impl Default for GameResource {
    fn default() -> Self {
        GameResource {
            current_state: GameState::default()
        }
    }
}
