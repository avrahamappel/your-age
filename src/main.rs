use std::rc::Rc;

use chrono::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod separators;

use separators::WithSeparators;

enum Msg {
    Tick,
    UpdateName(String),
    UpdateBirthday(String),
}

fn input_event_value(evt: Event) -> String {
    evt.target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}

macro_rules! age_html {
    ($age:ident) => {{
        let mut label = stringify!($age).to_string();

        // Depluralize if necessary
        if $age == "1" {
            label.pop();
        }

        html! {
            <>
                <b>{ $age }</b>
                { format!(" {} old", label) }
            </>
        }
    }};
}

fn current_time() -> NaiveDateTime {
    Local::now().naive_local()
}

#[derive(Clone)]
struct State {
    name: String,
    birthday: Option<NaiveDate>,
    current_time: NaiveDateTime,
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

impl Reducible for State {
    type Action = Msg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use Msg::*;

        let state = match Rc::try_unwrap(self) {
            Ok(state) => state,
            Err(rc) => (*rc).clone(),
        };

        match action {
            Tick => state.update_time(),
            UpdateName(name) => state.update_name(name),
            UpdateBirthday(birthday) => state.update_birthday(birthday),
        }
        .into()
    }
}

impl State {
    fn new() -> Self {
        Self {
            name: String::new(),
            birthday: None,
            current_time: current_time(),
        }
    }

    fn update_time(self) -> Self {
        Self {
            current_time: current_time(),
            ..self
        }
    }

    fn update_name(self, name: String) -> Self {
        Self { name, ..self }
    }

    fn update_birthday(self, birthday: String) -> Self {
        Self {
            birthday: NaiveDate::parse_from_str(&birthday, "%F").ok(),
            ..self
        }
    }
}

#[function_component(YourAge)]
fn your_age() -> Html {
    let state = use_reducer(State::new);

    let _interval = use_state({
        let state = state.clone();
        move || Interval::new(1000, move || state.dispatch(Msg::Tick))
    });

    let name_callback = {
        let state = state.clone();
        Callback::from(move |evt: Event| state.dispatch(Msg::UpdateName(input_event_value(evt))))
    };

    let birthday_callback = {
        let state = state.clone();
        Callback::from(move |evt: Event| {
            state.dispatch(Msg::UpdateBirthday(input_event_value(evt)))
        })
    };

    let output = output(&*state);

    html! {
        <>
            <h2>{ "Type your name and birthday" }</h2>

            <label for="name">{ "Name" }</label>
            <input name="name" onchange={name_callback} />

            <br />

            <label for="birthday">{ "Birthday" }</label>
            <input type="date" name="birthday" onchange={birthday_callback} />

            {output}
        </>
    }
}

fn main() {
    yew::start_app::<YourAge>();
}
