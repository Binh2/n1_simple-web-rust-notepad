// use crate::util::{inner_u64, inner_u32};
use leptos::prelude::*;
use leptos::leptos_dom::logging::*;
use leptos::control_flow::{For, };
use leptos::web_sys::HtmlInputElement;
use leptos::web_sys;
use wasm_bindgen::{JsCast};
use leptos::ev::{self, MouseEvent};
use leptos::html;
use rand::seq::IndexedRandom; 

fn main() {
    leptos::mount::mount_to_body(|| view! { <App></App> })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TextEditorStruct {
    id: usize,
    title: RwSignal<String>,
    text: RwSignal<String>
}

const TITLES: &[&str] = &[ "Quick notes", "To-do", "Scratchpad", "Brainstorms", "Meeting notes", "Daily journal", "Secret", "Life admin", "Swipe file", "Vault", "Archive" ];
fn create_text_editor(id: usize) -> TextEditorStruct {
    let mut rng = rand::rng();
    let title = TITLES.choose(&mut rng).unwrap_or(&"Error :)");
    console_log(title);

    TextEditorStruct {
        id: id,
        title: RwSignal::new(title.to_string()),
        text: RwSignal::new(String::from("")),
    }
}

#[component]
fn App() -> impl IntoView {
    let (active_id, set_active_id) = signal(0 as usize);
    let initial_text_editors = vec![
        create_text_editor(0),        
        create_text_editor(1),  
        create_text_editor(2),    
    ];
    let (id_counter, set_id_counter) = signal(3);
    let (text_editors, set_text_editors) = signal::<Vec<TextEditorStruct>>(initial_text_editors);

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

    let remove_tab = move |id: usize| {
        set_text_editors.update(move |vec| {
            vec.retain(|el| el.id != id);
        });
    };

    let doc = document();
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
            let Ok(mouse_event) = MouseEvent::new("click") else { return; };
            add_tab(mouse_event);
        }
        if evt.key() == "F2" {
            evt.prevent_default();
            if let Ok(Some(el)) = doc.query_selector(".text-editor__tab-title--active") {
                if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                    let _ = input.focus();
                    let length = input.value().len() as u32;
                    let _ = input.set_selection_range(length, length);
                }
            }
        }
        if evt.ctrl_key() && evt.key() == ";" {
            evt.prevent_default();

            if let Some(textarea) = active_textarea.get() {
                let _ = textarea.focus();
            }
        }
        if evt.ctrl_key() && evt.key() == "Escape" {
            evt.prevent_default();
            
            let Ok(mouse_event) = MouseEvent::new("click") else {return;};
            remove_tab(active_id.get());
        }
    });
    window_event_listener(ev::focusin, move |evt: ev::FocusEvent| {
        let Some(target) = evt.target() else {return;};
        let Ok(active_element) = target.dyn_into::<web_sys::HtmlElement>() else {return;}; 
        if active_element.matches("input.text-editor__tab-title").unwrap_or(false) {
            let Some(id) = active_element.dataset().get("tabId") else { return; };
            let Ok(usize_id) = id.parse::<usize>() else { return; };
            set_active_id.set(usize_id);
            console_log(format!("Active id: {}", id).as_str());
        }
    });





    view! {
        <div class="text-editor__tab-titles">
            <For
                each=move || text_editors.get()
                key=|text_editor| text_editor.id
                children=move |text_editor: TextEditorStruct| {
                    view! {
                        <div class="text-editor__tab-title-container">
                            <input
                                data-tab-id=move || text_editor.id
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
                                    let Some(id) = el.dataset().get("tabId") else { return; };
                                    let Ok(usize_id) = id.parse::<usize>() else { return; };
                                    set_active_id.set(usize_id);
                                }
                            />
                            <button on:click=move |evt| {
                                let Some(el) = evt.target() else { return; };
                                let Ok(el) = el.dyn_into::<web_sys::HtmlElement>() else { return; };
                                let Some(targeted_id) = el.dataset().get("tabId") else { return; };
                                let Ok(usize_id) = targeted_id.parse::<usize>() else { return; };
                                remove_tab(usize_id);
                            } data-tab-id=move || text_editor.id class="text_editor__tab-title-delete-button">"x"</button>
                        </div>
                    }
                }
            />
            <button
                on:click=add_tab
                class="text-editor__tab-title text-editor__tab-title--add-button"
            >
                "+"
            </button>
        </div>

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
        <Shortcuts></Shortcuts>
    }
}

#[component]
fn Shortcuts() -> impl IntoView {
    let (is_hidden, set_is_hidden) = signal(false);
    let display_none_style = move || {
        if is_hidden.get() { "none" } else { "block" }
    };
    view! {
        <div class="shortcuts">
            <p
                on:click=move |_| {
                    set_is_hidden
                        .update(|is_hidden| {
                            *is_hidden = !*is_hidden;
                        })
                }
                class="shortcuts__title"
            >
                <strong>"Shortcuts"</strong>
                " v"
            </p>
            <p style:display=display_none_style class="shortcuts__shortcut">
                "Ctrl + ,: Previous tab"
            </p>
            <p style:display=display_none_style class="shortcuts__shortcut">
                "Ctrl + .: Next tab"
            </p>
            <p style:display=display_none_style class="shortcuts__shortcut">
                "Ctrl + /: New tab"
            </p>
            <p style:display=display_none_style class="shortcuts__shortcut">
                "Ctrl + Esc: Remove current tab"
            </p>
            <p style:display=display_none_style class="shortcuts__shortcut">
                "F2: Rename tab"
            </p>
            <p style:display=display_none_style class="shortcuts__shortcut">
                "Ctrl + ;: Focus on note"
            </p>
        </div>
    }
}
