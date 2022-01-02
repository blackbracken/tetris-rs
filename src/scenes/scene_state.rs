pub trait SceneState<G> {
    fn game_state(&self) -> &G;
    fn is_paused(&self) -> bool;
}
