use chrono::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Msg {
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

/// Insert separators in a number string.
/// https://stackoverflow.com/a/58437629
fn insert_separators(num_str: String) -> String {
    let mut s = Vec::with_capacity(num_str.len() + (num_str.len() / 3));
    let it = num_str.chars().rev().enumerate();

    for (i, ch) in it {
        if i != 0 && i % 3 == 0 {
            s.push(',');
        }
        s.push(ch);
    }

    s.iter().rev().collect()
}

trait WithSeparators {
    fn with_separators(self) -> String
    where
        Self: Sized;
}

impl WithSeparators for String {
    fn with_separators(self) -> Self {
        insert_separators(self)
    }
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

#[derive(Default)]
struct YourAge {
    name: String,
    birthday: Option<NaiveDate>,
}

impl YourAge {
    /// Format the output of the age as Html
    fn output(&self) -> Html {
        if self.name.is_empty() {
            return html! {};
        }

        if let Some(birthday) = self.birthday {
            let duration = Local::today().naive_local().signed_duration_since(birthday);
            let days = duration.num_days();
            let years = (days / 365).to_string().with_separators();
            let months = (days / 30).to_string().with_separators();
            let hours = duration.num_hours().to_string().with_separators();
            let minutes = duration.num_minutes().to_string().with_separators();
            let seconds = duration.num_seconds().to_string().with_separators();
            let days = days.to_string().with_separators();

            html! {
                <>
                    <h2>{ "Hello " } {&self.name} { "!" }</h2>

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
}

impl Component for YourAge {
    type Properties = ();
    type Message = Msg;

    fn create(_: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _: &Context<Self>, msg: Msg) -> bool {
        use Msg::*;

        match msg {
            UpdateName(name) => self.name = name,
            UpdateBirthday(birthday) => {
                self.birthday = NaiveDate::parse_from_str(&birthday, "%F").ok()
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name_callback = ctx
            .link()
            .callback(|evt: Event| Msg::UpdateName(input_event_value(evt)));

        let birthday_callback = ctx
            .link()
            .callback(|evt: Event| Msg::UpdateBirthday(input_event_value(evt)));

        html! {
            <>
                <h2>{ "Type your name and birthday" }</h2>

                <label for="name">{ "Name" }</label>
                <input name="name" onchange={name_callback} />

                <br />

                <label for="birthday">{ "Birthday" }</label>
                <input type="date" name="birthday" onchange={birthday_callback} />

                {self.output()}
            </>
        }
    }
}

fn main() {
    yew::start_app::<YourAge>();
}
