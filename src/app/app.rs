use gloo::storage::{LocalStorage, Storage};
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement as InputElement;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{html, Classes, Component, Context, Html, NodeRef, TargetCast};

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
        let logo_url = "./static/images/rust-logo.png";

        html! {
            <div class="h-100 w-full flex flex-col items-center justify-center bg-teal-lightest font-sans">
                <img class="block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0" src={logo_url} alt="Rust Logo" />

                <section class="card bg-base-100 lg:card-side shadow-xl flex flex-col">

                    <div class="card-body">
                        <div class="card-title flex flex-col">
                            <h1 class="text-grey-darkest">{"ToDos"}</h1>
                            { self.view_input(ctx.link())}
                        </div>
                        <ul class="menu menu-horizontal menu-sm bg-base-200 rounded-box gap-2">
                            { for Filter::iter().map(|f| self.view_filter(f, ctx.link())) }
                        </ul>
                        <ul class="flex flex-col justify-start mb-4 items-center">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fits(e)).enumerate().map(|e| self.view_entry(e, ctx.link())) }
                        </ul>
                    </div>
                </section>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter, link: &Scope<Self>) -> Html {
        let mut cls = Classes::from("");
        if self.state.filter == filter {
            cls.push("btn-active");
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
            <div class="join">
                <input
                    type="text"
                    class="input input-bordered join-item"
                    placeholder="Add todos"
                    onkeypress={onkeypress}
                />
                <button class="btn join-item">{"Add"}</button>
            </div>
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry), link: &Scope<Self>) -> Html {
        let cls = Classes::from("flex w-full justify-between items-center mb-2");

        html! {
            <li class={cls}>
                <div class="flex items-center gap-2 flex-1">
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
                    <kbd class="kbd cursor-pointer opacity-100 hover:opacity-80" onclick={link.callback(move |_| Msg::Remove(idx))}>{"x"}</kbd>
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
                    class="input input-ghost w-full max-w-xs flex-1"
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
