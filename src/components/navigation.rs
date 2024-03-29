use crate::components::Logo;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <>
        <nav>
        <Link<Route> to={Route::Konnektoren}>{ "Konnektoren" }</Link<Route>>
        <Link<Route> to={Route::Adjectives}>{ "Adjektive" }</Link<Route>>
        <Link<Route> to={Route::Home}><Logo img_src={"/favicon.png".to_string()} /></Link<Route>>
        <Link<Route> to={Route::Verbs}>{ "Verben" }</Link<Route>>
        <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
        <Link<Route> to={Route::Config}>{ "Config" }</Link<Route>>
        </nav>
        </>
    }
}
