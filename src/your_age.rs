use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use chrono::{Local, NaiveDate, NaiveDateTime};
use gloo_timers::callback::Interval;
use url::form_urlencoded::{self, Serializer};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::input::Input;
use crate::output::Output;

const DATE_FORMAT: &str = "%F";

enum QueryParamsAction {
    UpdateName(String),
    UpdateBirthday(String),
}

#[derive(Clone)]
struct QueryParams {
    name: String,
    birthday: Option<NaiveDate>,
}

impl Reducible for QueryParams {
    type Action = QueryParamsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let params = match Rc::try_unwrap(self) {
            Ok(params) => params,
            Err(rc) => (*rc).clone(),
        };

        match action {
            QueryParamsAction::UpdateName(name) => params.update_name(name),
            QueryParamsAction::UpdateBirthday(birthday) => params.update_birthday(birthday),
        }
        .into()
    }
}

impl Display for QueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serializer = Serializer::new(String::new());

        if !self.name.is_empty() {
            serializer.append_pair("name", &self.name);
        }

        if let Some(birthday) = self.birthday {
            serializer.append_pair("birthday", &birthday.format(DATE_FORMAT).to_string());
        }

        write!(f, "{}", serializer.finish())
    }
}

impl QueryParams {
    fn update_name(self, name: String) -> Self {
        QueryParams { name, ..self }.set_location_search()
    }

    fn update_birthday(self, birthday: String) -> Self {
        let birthday = NaiveDate::parse_from_str(&birthday, DATE_FORMAT).ok();
        QueryParams { birthday, ..self }.set_location_search()
    }

    fn set_location_search(self) -> Self {
        gloo_utils::window()
            .location()
            .set_search(self.to_string().as_str())
            .expect_throw("Couldn't update URL query");
        self
    }
}

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

fn state_from_query() -> QueryParams {
    let query = gloo_utils::window()
        .location()
        .search()
        .expect_throw("Couldn't get query string");
    let query_map = form_urlencoded::parse(query.as_bytes()).collect::<HashMap<_, _>>();

    QueryParams {
        name: query_map
            .get("name")
            .map(|name| name.to_string())
            .unwrap_or_default(),
        birthday: query_map
            .get("birthday")
            .and_then(|val| NaiveDate::parse_from_str(val, DATE_FORMAT).ok()),
    }
}

#[function_component(YourAge)]
pub fn your_age() -> Html {
    let state = use_reducer(state_from_query);
    let current_time = use_state(get_current_time);

    // TODO Add a hook here that will push the birthday and name to the URL query
    // Later we can use that to implement a share button

    let _interval = use_state({
        let current_time = current_time.clone();
        move || Interval::new(1000, move || current_time.set(get_current_time()))
    });

    let name_callback = {
        let state = state.clone();
        Callback::from(move |evt: Event| {
            state.dispatch(QueryParamsAction::UpdateName(input_event_value(evt)))
        })
    };

    let birthday_callback = {
        let state = state.clone();
        Callback::from(move |evt: Event| {
            state.dispatch(QueryParamsAction::UpdateBirthday(input_event_value(evt)))
        })
    };

    let duration = state
        .birthday
        .map(|birthday| current_time.signed_duration_since(birthday.and_hms(0, 0, 0)));

    let name = state.name.clone();

    html! {
        <>
            <Input {name_callback} {birthday_callback} />
            <Output {name} {duration} />
        </>
    }
}
