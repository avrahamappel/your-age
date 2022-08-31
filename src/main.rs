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

struct State {
    name: String,
    birthday: Option<NaiveDate>,
    current_time: NaiveDateTime,
    _interval: Interval,
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

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        use Msg::*;

        match action {
            Tick => self.current_time = current_time(),
            UpdateName(name) => self.name = name,
            UpdateBirthday(birthday) => {
                self.birthday = NaiveDate::parse_from_str(&birthday, "%F").ok()
            }
        }

        self
    }
}

impl State {
    fn new() -> Self {
        let _interval = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::Tick))
        };

        Self {
            name: String::new(),
            birthday: None,
            current_time: current_time(),
            _interval,
        }
    }
}

#[function_component(YourAge)]
fn your_age() -> Html {
    let state = use_reducer(State::new);

    let name_callback = Callback::from(|evt: Event| Msg::UpdateName(input_event_value(evt)));

    let birthday_callback =
        Callback::from(|evt: Event| Msg::UpdateBirthday(input_event_value(evt)));

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
