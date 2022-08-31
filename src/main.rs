use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod separators;
mod state;

use separators::WithSeparators;
use state::{Action, State};

fn input_event_value(evt: Event) -> String {
    evt.target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}

macro_rules! age_html {
    ($age:ident) => {{
        let label = stringify!($age).to_string();

        // Depluralize if necessary
        let label = if $age == "1" {
            &label[..label.len() - 1]
        } else {
            &label
        };

        html! {
            <>
                <b>{ $age }</b>
                { format!(" {} old", label) }
            </>
        }
    }};
}

/// Input form for entering data
fn input(state: &UseReducerHandle<State>) -> Html {
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

    html! {
        <>
            <h2>{ "Type your name and birthday" }</h2>

            <label for="name">{ "Name" }</label>
            <input name="name" onchange={name_callback} />

            <br />

            <label for="birthday">{ "Birthday" }</label>
            <input type="date" name="birthday" onchange={birthday_callback} />
        </>
    }
}

/// Format the output of the age as Html
fn output(state: &State) -> Html {
    if state.name.is_empty() {
        return html! {};
    }

    if let Some(birthday) = state.birthday {
        let duration = state
            .current_time
            .signed_duration_since(birthday.and_hms(0, 0, 0));
        let days = duration.num_days();
        let years = (days / 365).to_string().with_separators();
        let months = (days / 30).to_string().with_separators();
        let hours = duration.num_hours().to_string().with_separators();
        let minutes = duration.num_minutes().to_string().with_separators();
        let seconds = duration.num_seconds().to_string().with_separators();
        let days = days.to_string().with_separators();

        html! {
            <>
                <h2>{ "Hello " } {&state.name} { "!" }</h2>

                <p>{ "You are:" }</p>

                <p>{ age_html!(years) }</p>
                <p>{ age_html!(months) }</p>
                <p>{ age_html!(days) }</p>
                <p>{ age_html!(hours) }</p>
                <p>{ age_html!(minutes) }</p>
                <p>{ age_html!(seconds) }</p>
            </>
        }
    } else {
        html! { <p><i>{ "Enter a valid birthday" }</i></p> }
    }
}

#[function_component(YourAge)]
fn your_age() -> Html {
    let state = use_reducer(State::new);

    let _interval = use_state({
        let state = state.clone();
        move || Interval::new(1000, move || state.dispatch(Action::Tick))
    });

    let input = input(&state);
    let output = output(&state);

    html! {
        <>
            {input}
            {output}
        </>
    }
}

fn main() {
    yew::start_app::<YourAge>();
}
