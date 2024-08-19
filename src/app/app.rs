use gloo::storage::{LocalStorage, Storage};
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement as InputElement;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Classes, Component, Context, Html, NodeRef, TargetCast};

use super::state::{Entry, Filter, State};

const KEY: &str = "yew.todomvc.self";

pub enum Msg {
    Add(String),
    Edit((usize, String)),
    Remove(usize),
    Toggle(usize),
    ToggleAll,
    ToggleEdit(usize),
    SetFilter(Filter),
    ClearCompleted,
    Focus,
}

pub struct App {
    state: State,
    focus_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let entries = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        let state = State {
            entries,
            filter: Filter::All,
            edit_value: "".into(),
        };

        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(description) => {
                let description = description.trim();
                if !description.is_empty() {
                    let entry = Entry {
                        description: description.to_string(),
                        completed: false,
                        editing: false,
                    };

                    self.state.entries.push(entry);
                }
            }

            Msg::Edit((idx, description)) => {
                self.state
                    .complete_edit(idx, description.trim().to_string());
                self.state.edit_value = "".into();
            }

            Msg::Remove(idx) => {
                self.state.remove(idx);
            }

            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }

            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }

            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }

            Msg::ToggleEdit(idx) => {
                let entry = self
                    .state
                    .entries
                    .iter()
                    .filter(|e| self.state.filter.fits(e))
                    .nth(idx)
                    .unwrap();
                self.state.edit_value.clone_from(&entry.description);
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }

            Msg::ClearCompleted => {
                self.state.clear_completed();
            }

            Msg::Focus => {
                if let Some(element) = self.focus_ref.cast::<InputElement>() {
                    element.focus().unwrap();
                }
            }
        }
        LocalStorage::set(KEY, &self.state.entries).expect("Failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };

        html! {
            <div class="todomvc-wrapper h-100 w-full flex flex-col items-center justify-center bg-teal-lightest font-sans">
                <img class="block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0" src="/static/images/rust-logo.png" alt="Rust Logo" />

                <section class="todo-app bg-white rounded shadow p-6 m-4 w-full lg:w-3/4 lg:max-w-lg">
                    <header class="header mb-4">
                        <h1 class="text-grey-darkest">{"ToDos"}</h1>
                        { self.view_input(ctx.link())}
                    </header>
                    <section class={classes!("main", hidden_class)}>
                        <ul class="menu menu-vertical lg:menu-horizontal bg-base-200 rounded-box">
                            { for Filter::iter().map(|f| self.view_filter(f, ctx.link())) }
                        </ul>
                        <label class="label cursor-pointer">
                            <input
                                type="checkbox"
                                class="checkbox checkbox-primary mr-2"
                                id="toggle-all"
                                checked={self.state.is_all_completed()}
                                onclick={ctx.link().callback(|_| Msg::ToggleAll)}
                            />
                            <span for="label-text">{"Mark all as complete"}</span>
                        </label>
                        <ul class="todo-list flex flex-col justify-start mb-4 items-center">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fits(e)).enumerate().map(|e| self.view_entry(e, ctx.link())) }
                        </ul>
                    </section>
                    <footer class={classes!("footer", hidden_class, ["flex", "flex-col"])}>
                        <div class="todo-count w-full">
                            <span class="font-bold">{ self.state.total() }</span><span>{ " item(s) left" }</span>
                        </div>
                        <div class="clear-completed" onclick={ctx.link().callback(|_| Msg::ClearCompleted)}>
                            { format!("Clear completed ({})", self.state.total_completed())}
                        </div>
                    </footer>
                </section>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter, link: &Scope<Self>) -> Html {
        let mut cls = Classes::from("filter");
        if self.state.filter == filter {
            cls.push("selected");
        } else {
            cls.push(classes!("unselected"))
        };

        html! {
            <li>
                <a class={cls}
                    href={filter.as_href()}
                    onclick={link.callback(move |_| Msg::SetFilter(filter))}>
                    { filter }
                </a>
            </li>
        }
    }

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
            <div class="flex mt-4">
                <input
                    type="text"
                    class="shadow appearance-none border rounded w-full py-2 px-3 mr-4 text-grey-darker"
                    placeholder="Add todos"
                    onkeypress={onkeypress}
                />
            </div>
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo flex w-full justify-between items-center mb-2");
        if entry.editing {
            class.push(" editing");
        }
        if entry.completed {
            class.push(" completed");
        }

        html! {
            <li {class}>
                <div class="todo-item flex items-center gap-2 flex-1">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked={entry.completed}
                        onclick={link.callback(move |_| Msg::Toggle(idx))}
                    />
                    {
                        if entry.editing {
                            html! {
                                self.view_entry_edit_input((idx, entry), link)
                            }
                        } else {
                            html! {
                                <label class="w-full text-grey-darkest" ondblclick={link.callback(move |_| Msg::ToggleEdit(idx))}>{ &entry.description }</label>
                            }
                        }
                    }
                    <button class="btn btn-error" onclick={link.callback(move |_| Msg::Remove(idx))}>
                        { "Remove" }
                    </button>
                </div>
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let edit = move |input: InputElement| {
            let value = input.value();
            input.set_value("");
            Msg::Edit((idx, value))
        };

        let onblur = link.callback(move |e: FocusEvent| edit(e.target_unchecked_into()));

        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
        });

        if entry.editing {
            html! {
                <input
                    type="text"
                    class="edit flex-1"
                    value={self.state.edit_value.clone()}
                    ref={self.focus_ref.clone()}
                    onmouseover={link.callback(|_| Msg::Focus)}
                    {onblur}
                    {onkeypress}
                />
            }
        } else {
            html! {
                <input type="hidden"/>
            }
        }
    }
}
