use crate::game::GameState;
use yew::prelude::*;

pub fn mutate_game<E: 'static>(
    game: UseStateHandle<GameState>,
    action: impl Fn(&mut GameState) + 'static,
) -> Callback<E> {
    Callback::from(move |_| {
        game.set({
            let mut g = (*game).clone();
            action(&mut g);
            g.save();
            g
        });
    })
}

pub fn mutate_game_index(
    game: UseStateHandle<GameState>,
    action: impl Fn(&mut GameState, usize) + 'static,
) -> Callback<usize> {
    Callback::from(move |idx| {
        game.set({
            let mut g = (*game).clone();
            action(&mut g, idx);
            g.save();
            g
        });
    })
}
