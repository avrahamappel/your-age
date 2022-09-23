use chrono::Duration;
use yew::prelude::*;

use crate::separators::WithSeparators;
use crate::share::ShareButton;

#[derive(Eq, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub duration: Option<Duration>,
}

macro_rules! age_html {
    ($age:ident) => {{
        let label = stringify!($age).to_string();

        // Depluralize if necessary
        let label = if $age == "1" {
            &label[..label.len() - 1]
        } else {
            &label
        };

        html! {
            <>
                <b>{ $age }</b>
                { format!(" {} old", label) }
            </>
        }
    }};
}

/// Format the output of the age as Html
#[function_component(Output)]
pub fn output(Props { duration, name }: &Props) -> Html {
    if name.is_empty() {
        return html! {};
    }

    if let Some(duration) = duration {
        let days = duration.num_days();
        let years = (days / 365).to_string().with_separators();
        let months = (days / 30).to_string().with_separators();
        let hours = duration.num_hours().to_string().with_separators();
        let minutes = duration.num_minutes().to_string().with_separators();
        let seconds = duration.num_seconds().to_string().with_separators();
        let days = days.to_string().with_separators();

        html! {
            <>
                <h2>{ "Hello " } {name} { "!" }</h2>

                <p>{ "You are:" }</p>

                <p>{ age_html!(years) }</p>
                <p>{ age_html!(months) }</p>
                <p>{ age_html!(days) }</p>
                <p>{ age_html!(hours) }</p>
                <p>{ age_html!(minutes) }</p>
                <p>{ age_html!(seconds) }</p>

                <br />

                <p>{ "Share this page" }</p>
                <ShareButton></ShareButton>
            </>
        }
    } else {
        html! { <p><i>{ "Enter a valid birthday" }</i></p> }
    }
}
