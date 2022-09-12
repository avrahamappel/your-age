mod input;
mod output;
mod separators;
mod state;

use state::YourAge;

fn main() {
    yew::start_app::<YourAge>();
}
