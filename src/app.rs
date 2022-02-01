use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement as InputElement;
use yew::{
    classes,
    events::{FocusEvent, KeyboardEvent},
    html,
    html::Scope,
    Classes, Component, Context, Html, NodeRef, TargetCast,
};

use crate::state::{State, Filter, Entry};

pub enum Msg {
    Add(String),
    SetFilter(Filter),
}

pub struct App {
    state: State,
}

const STORAGE_KEY: &str = "yew.todomvc.self";

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let entries = LocalStorage::get(STORAGE_KEY).unwrap_or_else(|_| Vec::new());
        let state = State {
            entries,
            filter: Filter::All,
            edit_value: "".into(),
        };

        Self {
            state,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(description) => {
                if !description.is_empty() {
                    let entry = Entry {
                        description: description.trim().to_string(),
                        completed: false,
                        editing: false,
                    };
                    self.state.entries.push(entry);
                }
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
        }

        LocalStorage::set(STORAGE_KEY, &self.state.entries).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };

        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        { self.view_input(ctx.link()) }
                    </header>
                    <section class={classes!("main", hidden_class)}>
                        <ul class="todo-list">
                            {
                                for self.state.entries
                                    .iter()
                                    .filter(|e| self.state.filter.fits(e))
                                    .enumerate()
                                    .map(|e| self.view_entry(e, ctx.link()))
                            }
                        </ul>
                    </section>
                    <footer class={classes!("footer", hidden_class)}>
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/lazaryan" target="_blank">{ "Sergey Lazaryan" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });

        html! {
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo");
        if entry.editing {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }
        html! {
            <li {class}>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked={entry.completed}
                    />
                    <label>{ &entry.description }</label>
                    <button class="destroy" />
                </div>
            </li>
        }
    }
}