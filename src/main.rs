use chrono::prelude::*;
use yew::prelude::*;

enum Msg {
    UpdateName(String),
    UpdateBirthday(String),
}

#[derive(Default)]
struct YourAge {
    name: String,
    birthday: Option<DateTime<Local>>,
}

impl YourAge {
    fn output(&self) -> Html {
        if self.name.len() == 0 && self.birthday.is_none() {
            return html! {};
        }

        let Some(birthday) = self.birthday;
        let duration = Local::now().signed_duration_since(birthday);
        let days = duration.num_days();
        let years = days / 365;
        let months = years * 12;
        let hours = duration.num_hours();
        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds();

        html! {
            <>
                <h2>{ "Hello" } {self.name} { "!" }</h2>

                <p>{ "You are:" }</p>

                <p>{years} { "years old" }</p>

                <p>{months} { "months old" }</p>

                <p>{days} { "days old" }</p>

                <p>{hours} { "hours old" }</p>

                <p>{minutes} { "minutes old" }</p>

                <p>{seconds} { "seconds old" }</p>
            </>
        }
    }
}

impl Component for YourAge {
    type Properties = ();
    type Message = Msg;

    fn create(_: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(self, _: &Context<Self>, msg: Msg) -> Self {
        use Msg::*;

        match msg {
            UpdateName(name) => Self { name, ..self },
            UpdateBirthday(birthday) => Self {
                birthday: DateTime::parse(birthday),
                ..self
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Type your name and birthday" }</h2>

                <label for="name">{ "Name" }</label>
                <input name="name" onchange={ctx.link().callback(|evt| Msg::UpdateName(evt.target.value))} />

                <label for="birthday">{ "Birthday" }</label>
                <input type="date" name="birthday" onchange={ctx.link().callback(|evt| Msg::UpdateBirthday(evt.target.value))} />

                {self.output()}
            </>
        }
    }
}

fn main() {
    yew::start_app::<YourAge>();
}
