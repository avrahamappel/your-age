use chrono::prelude::*;
use gloo_console::log;
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

#[derive(Default)]
struct YourAge {
    name: String,
    birthday: Option<DateTime<Local>>,
}

impl YourAge {
    fn output(&self) -> Html {
        if self.name.is_empty() {
            return html! {};
        }

        if let Some(birthday) = self.birthday {
            let duration = Local::now().signed_duration_since(birthday);
            let days = duration.num_days();
            let years = days / 365;
            let months = years * 12;
            let hours = duration.num_hours();
            let minutes = duration.num_minutes();
            let seconds = duration.num_seconds();

            html! {
                <>
                    <h2>{ "Hello" } {&self.name} { "!" }</h2>

                    <p>{ "You are:" }</p>

                    <p>{years} { "years old" }</p>

                    <p>{months} { "months old" }</p>

                    <p>{days} { "days old" }</p>

                    <p>{hours} { "hours old" }</p>

                    <p>{minutes} { "minutes old" }</p>

                    <p>{seconds} { "seconds old" }</p>
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
                // debug
                log!(&birthday);

                self.birthday = Local.datetime_from_str(&birthday, "").ok()
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
