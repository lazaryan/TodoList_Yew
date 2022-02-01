use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::state::{State, Filter};

pub enum Msg {
    AddOne,
}

pub struct App {
    state: State,
}

const STORE_KEY: &str = "yew.todomvc.self";

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let entries = LocalStorage::get(STORE_KEY).unwrap_or_else(|_| Vec::new());
        let state = State {
            entries,
            filter: Filter::All,
            edit_value: "".into(),
        };

        Self {
            state,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                    </header>
                </section>
            </div>
        }
    }
}
