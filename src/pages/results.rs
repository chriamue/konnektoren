use crate::route::Route;
use crate::{components::Certificate, model::TestResult};

#[cfg(feature = "verifiable-credentials")]
use crate::components::VerifiableCredential;
use gloo_timers::callback::Timeout;
use konnektoren_yew::components::MusicComponent;
use urlencoding::encode;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::{use_clipboard, UseClipboardHandle};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultsProps {
    pub code: Option<String>,
}

#[function_component(Results)]
pub fn results(props: &ResultsProps) -> Html {
    let clipboard_handle: UseClipboardHandle = use_clipboard();
    let show_copied_message = use_state(|| false);

    let code: String = props.code.as_ref().unwrap_or(&"".to_string()).clone();
    let test_result = TestResult::from_base64(&code.clone());

    let host = window().unwrap().location().host().unwrap_or_default();
    let protocol = window().unwrap().location().protocol().unwrap_or_default();

    match test_result {
        Ok(test_result) => {
            let encoded_code: String = encode(&test_result.to_base64()).into_owned();
            let share_url = format!("{}//{}/?page=results&code={}", protocol, host, encoded_code);

            let on_share_click = {
                let clipboard_handle = clipboard_handle.clone();
                let data = share_url.clone();
                let show_copied_message = show_copied_message.clone();
                Callback::from(move |_| {
                    clipboard_handle.write_text(data.to_string());
                    show_copied_message.set(true);
                    let show_copied_message = show_copied_message.clone();
                    Timeout::new(3000, move || {
                        show_copied_message.set(false);
                    })
                    .forget();
                })
            };

            let verification_message = if test_result.verify() {
                "This result has been verified. ✅"
            } else {
                "Verification pending. ⏳"
            };

            #[cfg(feature = "verifiable-credentials")]
            let verifiable_credential = html! {
                <VerifiableCredential test_result={test_result.clone()} />
            };
            #[cfg(not(feature = "verifiable-credentials"))]
            let verifiable_credential = html! {};

            html! {
                <div class="results-page">
                    <MusicComponent url="/assets/background_main.wav" />
                    <h1>{ "Test Results" }</h1>
                    <p>{ format!("{} did a fantastic job in the {} test, achieving a performance of {:.2}% by answering correctly {} out of {} questions.",
                        test_result.player_name, test_result.test_type, test_result.performance_percentage, test_result.correct_answers, test_result.total_questions) }</p>
                    <div class={"verified"}>
                        <span>{verification_message}</span>
                    </div>
                    <input type="text" class="share-url-input" readonly=true value={share_url.clone()} />
                    <button onclick={on_share_click}>{ "Share This Result" }</button>
                    if *show_copied_message {
                        <p class="copied-message">{"Link copied to clipboard!"}</p>
                    }

                    <Certificate test_result={test_result.clone()} />
                    {verifiable_credential}

                    <div class="try-again">
                    <Link<Route> to={Route::Home}>{ "Try the test yourself again!" }</Link<Route>>
                </div>
                </div>
            }
        }
        Err(error_message) => html! {
            <div class="results-page">
                <h1>{ "Oops!" }</h1>
                <p>{ "It seems like something went wrong with displaying your test results." }</p>
                <p>{ error_message.to_string() }</p>
                <p>{ "Maybe the code was incorrect, or perhaps our server just needs a coffee break. 🤷‍♂️ ☕" }</p>
            </div>
        },
    }
}
