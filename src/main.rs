use leptos::prelude::*;
use leptos::leptos_dom::logging::*;
use leptos::control_flow::{For, };
use leptos::web_sys::HtmlInputElement;
use leptos::web_sys;
use wasm_bindgen::{JsCast, JsValue};
use leptos::ev::{self, MouseEvent};
use leptos::html;

fn main() {
    leptos::mount::mount_to_body(|| view! { <App></App> })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TextEditorStruct {
    id: usize,
    title: RwSignal<String>,
    text: RwSignal<String>
}

fn create_text_editor(id: usize) -> TextEditorStruct {
    TextEditorStruct {
        id: id,
        title: RwSignal::new(String::from("hi")),
        text: RwSignal::new(String::from("")),
    }
}

#[component]
fn App() -> impl IntoView {
    let (active_id, set_active_id) = signal(0 as usize);
    let ids = [0,1,2];
    let initial_text_editors: Vec<TextEditorStruct> = ids.into_iter().map(move |id| TextEditorStruct {
        id: id,
        title: RwSignal::new(String::from("he")),
        text: RwSignal::new(String::from("fe")),
    }).collect();
    let (id_counter, set_id_counter) = signal(3);
    let (text_editors, set_text_editors) = create_signal::<Vec<TextEditorStruct>>(initial_text_editors);

    let active_text_editor = Memo::new(move |_| text_editors.get().into_iter().find(|text_editor| text_editor.id == active_id.get()));

    

    let active_textarea: NodeRef<html::Textarea> = NodeRef::new();
    let add_tab = move |_: MouseEvent| {
        set_text_editors
            .update(|vec| {
                vec.push(create_text_editor(id_counter.get()));
                set_active_id.update(move |id| *id = id_counter.get());
                set_id_counter.update(|counter| *counter += 1);
            });
        let Some(textarea) = active_textarea.get() else {
            return;
        };
        let _ = textarea.focus();
    };

    window_event_listener(ev::keydown, move |evt: ev::KeyboardEvent| {
        if evt.ctrl_key() && evt.key() == "." {
            evt.prevent_default();
            set_active_id.update(|id| *id =  (*id + 1) % id_counter.get());
        }
        if evt.ctrl_key() && evt.key() == "," {
            evt.prevent_default();
            set_active_id.update(|id| *id =  (*id + id_counter.get() - 1) % id_counter.get());
        }
        if evt.ctrl_key() && evt.key() == "/" {
            evt.prevent_default();
            add_tab(MouseEvent::new("click").unwrap());
        }
    });

    view! {
        <For
            each=move || text_editors.get()
            key=|text_editor| text_editor.id
            children=move |text_editor: TextEditorStruct| {
                view! {
                    <input
                        id=move || text_editor.id
                        class="text-editor__tab-title"
                        class:text-editor__tab-title--active=move || text_editor.id == active_id.get()
                        value=move || text_editor.title.get()
                        on:click=move |e| {
                            let Some(t) = e.target() else {
                                return;
                            };
                            let Ok(el) = t.clone().dyn_into::<HtmlInputElement>() else {
                                return;
                            };
                            let Ok(id) = el.id().parse::<usize>() else {
                                return;
                            };
                            set_active_id.set(id);
                        }
                    />
                }
            }
        />
        <button on:click=add_tab class="text-editor__tab-title text-editor__tab-title--add-button">
            "+"
        </button>

        <br />

        <Show when=move || active_text_editor.get().is_some() fallback=|| view! { <></> }>
            <textarea
                node_ref=active_textarea
                class="text-editor__textarea"
                on:input=move |e| {
                    active_text_editor.get().unwrap().text.set(event_target_value(&e))
                }
                prop:value=move || {
                    let Some(text_editor) = active_text_editor.get() else {
                        return String::new();
                    };
                    return text_editor.text.get();
                }
                autofocus
            ></textarea>
        </Show>
        <div>
            <p>
                <strong>"Shortcuts"</strong>
            </p>
            <p>"Ctrl + ,: Previous tab"</p>
            <p>"Ctrl + .: Next tab"</p>
            <p>"Ctrl + /: New tab"</p>
        </div>
    }
}
