use chrono::prelude::*;
use yew::prelude::*;

enum Msg {
    UpdateName,
    UpdateBirthday,
}

#[derive(Default)]
struct YourAge {
    name: String,
    birthday: Option<Date<Local>>,
}

impl YourAge {
    fn output(&self) -> Html {
        if self.name.len() == 0 && self.birthday.is_none() {
            return html! {};
        }

        let Some(birthday) = self.birthday;
        let duration = Local::now().signed_duration_since(birthday);
        let years = duration.num_days() / 365;
        let months = duration.num_days() / 30;
        let days = duration.num_days();
        let hours = duration.num_hours();
        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds();

        html! {
            <>
                <p>{ "Hello" } {self.name} { "!" }</p>

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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ "Type your name and birthday" }</p>

                <label for="name">{ "Name" }</label>
                <input name="name" />

                <label for="birthday">{ "Birthday" }</label>
                <input type="date" name="birthday" />

                {self.output()}
            </>
        }
    }
}

fn main() {
    yew::start_app::<YourAge>();
}
