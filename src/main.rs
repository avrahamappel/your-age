use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod input;
mod output;
mod separators;
mod state;

use input::Input;
use output::Output;
use state::{Action, State};

fn input_event_value(evt: Event) -> String {
    evt.target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}

#[function_component(YourAge)]
fn your_age() -> Html {
    let state = use_reducer(State::new);

    let _interval = use_state({
        let state = state.clone();
        move || Interval::new(1000, move || state.dispatch(Action::Tick))
    });

    let name_callback = {
        let state = state.clone();
        Callback::from(move |evt: Event| state.dispatch(Action::UpdateName(input_event_value(evt))))
    };

    let birthday_callback = {
        let state = state.clone();
        Callback::from(move |evt: Event| {
            state.dispatch(Action::UpdateBirthday(input_event_value(evt)))
        })
    };

    let duration = match state.birthday {
        Some(birthday) => Some(
            state
                .current_time
                .signed_duration_since(birthday.and_hms(0, 0, 0)),
        ),
        None => None,
    };

    let name = state.name.clone();

    html! {
        <>
            <Input {name_callback} {birthday_callback} />
            <Output {name} {duration} />
        </>
    }
}

fn main() {
    yew::start_app::<YourAge>();
}
