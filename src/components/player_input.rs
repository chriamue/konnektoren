use crate::model::{
    history::HISTORY_KEY, player::PLAYER_KEY, History, Player, TestResult, TestType,
};
use crate::route::Route;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Properties)]
pub struct PlayerInputProps {
    pub test_type: TestType,
    pub total_questions: usize,
    pub correct_answers: usize,
    pub incorrect_answers: usize,
}

#[function_component(PlayerInput)]
pub fn player_input(props: &PlayerInputProps) -> Html {
    let default_player_name = LocalStorage::get::<Player>(PLAYER_KEY)
        .map(|player| player.name)
        .unwrap_or_default();
    let player_name = use_state(|| default_player_name);
    let navigator = use_navigator().expect("No navigator");

    let on_generate_click = {
        let player_name = player_name.clone();
        let test_type = props.test_type.clone();
        let total_questions = props.total_questions;
        let correct_answers = props.correct_answers;
        let incorrect_answers = props.incorrect_answers;
        let date = chrono::Utc::now();

        Callback::from(move |_| {
            let mut test_result = TestResult::new(
                test_type.clone(),
                total_questions,
                correct_answers,
                incorrect_answers,
                (*player_name).clone(),
                date,
            );
            test_result.create_signature();

            let mut history: History =
                LocalStorage::get(HISTORY_KEY).unwrap_or_else(|_| History::new());
            history.add_test_result(test_result.clone());

            LocalStorage::set("history", &history).expect("Failed to save history");

            let encoded = test_result.to_base64();
            navigator.push(&Route::Results { code: encoded });
        })
    };

    html! {
        <div class="player-input-container">
            <input type="text" placeholder="Enter your name" value={(*player_name).clone()} oninput={Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                player_name.set(input.value());
            })} />
            <button onclick={on_generate_click}>{ "Generate Results Page" }</button>
        </div>
    }
}
