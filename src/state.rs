use std::rc::Rc;

use chrono::{Local, NaiveDate, NaiveDateTime};
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::input::Input;
use crate::output::Output;

fn input_event_value(evt: Event) -> String {
    evt.target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}

pub enum Action {
    Tick,
    UpdateName(String),
    UpdateBirthday(String),
}

#[derive(Clone)]
pub struct State {
    pub name: String,
    pub birthday: Option<NaiveDate>,
    pub current_time: NaiveDateTime,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use Action::*;

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

fn current_time() -> NaiveDateTime {
    Local::now().naive_local()
}

impl State {
    pub fn new() -> Self {
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
pub fn your_age() -> Html {
    let state = use_reducer(State::new);

    // TODO Add a hook here or in state (I was going to combine state and this component anyway)
    // that will push the birthday and name to the URL query
    // Later we can use that to implement a share button

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

    let duration = state.birthday.map(|birthday| {
        state
            .current_time
            .signed_duration_since(birthday.and_hms(0, 0, 0))
    });

    let name = state.name.clone();

    html! {
        <>
            <Input {name_callback} {birthday_callback} />
            <Output {name} {duration} />
        </>
    }
}
