mod input;
mod output;
mod separators;
mod share;
mod your_age;

use your_age::YourAge;

fn main() {
    yew::start_app::<YourAge>();
}
