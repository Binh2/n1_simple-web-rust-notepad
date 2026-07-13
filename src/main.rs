use leptos::prelude::*;
use leptos::leptos_dom::logging::*;
use leptos::html::{button, p, textarea};
use leptos::ev;
use leptos::control_flow::{For, ForProps};
use leptos::svg::text;

fn main() {
    leptos::mount::mount_to_body(|| view! { <App></App> })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TextEditorStruct {
    id: usize,
    title: RwSignal<String>,
    text: RwSignal<String>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
  id: usize,
  count: RwSignal<i32>
}

#[component]
fn Counters() -> impl IntoView {
  let (counters, set_counters) = create_signal::<Vec<Counter>>(vec![]);

  view! {
      <div>
          <For
              // a function that returns the items we're iterating over; a signal is fine
              each=move || counters.get()
              // a unique key for each item
              key=|counter| counter.id
              // renders each item to a view
              children=move |counter: Counter| {
                  view! { <button>"Value: " {move || counter.count.get()}</button> }
              }
          />
      </div>
  }
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

    let add_text_editor = move |_| {
        set_text_editors.update(|vec| {
            vec.push(create_text_editor(id_counter.get()));
            set_id_counter.update(|counter| *counter += 1);
        })
    };

    let active_text_editor = Memo::new(move |_| text_editors.get().into_iter().find(|text_editor| text_editor.id == active_id.get()));

    view! {
        <For
            each=move || text_editors.get()
            key=|text_editor| text_editor.id
            children=move |text_editor: TextEditorStruct| {
                view! {
                    <input
                        class=""
                        value=move || text_editor.title.get()
                        on:click=move |e| {
                            set_active_id.set(1);
                            console_log(&*format!("Active id: {}", 1));
                        }
                    />
                }
            }
        />
        <button on:click=add_text_editor>"+"</button>
        <br />
        <For
            each=move || text_editors.get()
            key=|text_editor| text_editor.id
            children=move |text_editor: TextEditorStruct| {
                view! { <textarea>{move || text_editor.text.get()}</textarea> }
            }
        />
        <br />
        <Show when=move || active_text_editor.get().is_some() fallback=|| view! { <></> }>
            <textarea on:input=move |e| {
                active_text_editor.get().unwrap().text.set(event_target_value(&e))
            }>{active_text_editor.get().unwrap().text.get()}</textarea>
        </Show>
    }
}

fn Tab() -> impl IntoView {
    let (title, setTitle) = signal(String::from("New document"));
    let (text, setText) = signal(String::from(""));
    let first_word = move || {
        text.with(|t| {
            let first_word = t.split(' ').next().unwrap_or("");
            format!("{:.6}", first_word)
        })
    };
    
    (
        p()
            .class("tab__title")
            .child(first_word()),
        TextEditor(text, setText)
    )
}

fn TextEditor(text: ReadSignal<String>, setText: WriteSignal<String>) -> impl IntoView {
    // let print_screen = move |_| console_log(text.get().as_str());

    (
        textarea()
            .attr("placeholder", "Give me some text...")
            .attr("class", "text-editor__textarea")
            .attr("autofocus", true)
            .prop("value", move || text.get())
            .on(ev::input, move |e| { setText.set(event_target_value(&e)); } )
            .on(ev::input, move |e| { console_log(text.get().as_str())} )
,
        p()
            .child(text.get())
    )

}