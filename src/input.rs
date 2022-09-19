use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub name_callback: Callback<Event>,
    pub birthday: AttrValue,
    pub birthday_callback: Callback<Event>,
}

/// Input form for entering data
#[function_component(Input)]
pub fn input(
    Props {
        name,
        name_callback,
        birthday,
        birthday_callback,
    }: &Props,
) -> Html {
    html! {
        <>
            <h2>{ "Type your name and birthday" }</h2>

            <label for="name">{ "Name" }</label>
            <input name="name" value={name.clone()} onchange={name_callback} />

            <br />

            <label for="birthday">{ "Birthday" }</label>
            <input type="date" name="birthday" value={birthday.clone()} onchange={birthday_callback} />
        </>
    }
}
