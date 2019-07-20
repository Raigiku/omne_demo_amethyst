use derive_new::new;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Gameplay,
}

#[derive(new)]
pub struct Game {
    #[new(value = "GameState::MainMenu")]
    pub current_state: GameState,
}
