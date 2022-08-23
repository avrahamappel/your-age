use yew::prelude::*;

struct YourBirthday {
    name: String,
    birthday: Option<Time>,
}

impl YourAge {
    fn output() {
        //
    }
}

impl Component for YourAge {
    fn create() -> Self {
        Self {
            name: "".into(),
            birthday: None,
        }
    }

    fn view() -> Html {
        html! {
            <>
                <p>Type your name and birthday</p>

                <label for="name">Name</label>
                <input name="name">

                <label for="birthday">Birthday</label>
                <input type="date" name="birthday">

                {if !name.empty() && birthday.is_some()}
                    <p>Hello {name}!</p>

                    <p>You are:</p>

                    <p>{years} years old</p>

                    <p>{months} months old</p>

                    <p>{days} days old</p>

                    <p>{hours} hours old</p>

                    <p>{minutes} minutes old</p>

                    <p>{seconds} seconds old</p>
                {endif}
        }
    }
}
