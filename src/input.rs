use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub name_callback: Callback<Event>,
    pub birthday_callback: Callback<Event>,
}

/// Input form for entering data
#[function_component(Input)]
pub fn input(
    Props {
        name_callback,
        birthday_callback,
    }: &Props,
) -> Html {
    html! {
        <>
            <h2>{ "Type your name and birthday" }</h2>

            <label for="name">{ "Name" }</label>
            <input name="name" onchange={name_callback} />

            <br />

            <label for="birthday">{ "Birthday" }</label>
            <input type="date" name="birthday" onchange={birthday_callback} />
        </>
    }
}
