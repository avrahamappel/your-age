use std::rc::Rc;

use chrono::{Local, NaiveDate, NaiveDateTime};
use yew::Reducible;

pub enum Msg {
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
