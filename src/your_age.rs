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

fn get_current_time() -> NaiveDateTime {
    Local::now().naive_local()
}

#[function_component(YourAge)]
pub fn your_age() -> Html {
    let name = use_state(String::new);
    let birthday = use_state(|| None);
    let current_time = use_state(get_current_time);

    // TODO Add a hook here that will push the birthday and name to the URL query
    // Later we can use that to implement a share button

    let _interval = use_state({
        let current_time = current_time.clone();
        move || Interval::new(1000, move || current_time.set(get_current_time()))
    });

    let name_callback = {
        let name = name.clone();
        Callback::from(move |evt: Event| name.set(input_event_value(evt)))
    };

    let birthday_callback = {
        let birthday = birthday.clone();
        Callback::from(move |evt: Event| {
            birthday.set(NaiveDate::parse_from_str(input_event_value(evt).as_str(), "%F").ok())
        })
    };

    let duration =
        birthday.map(|birthday| current_time.signed_duration_since(birthday.and_hms(0, 0, 0)));

    html! {
        <>
            <Input {name_callback} {birthday_callback} />
            <Output name={(*name).clone()} {duration} />
        </>
    }
}
