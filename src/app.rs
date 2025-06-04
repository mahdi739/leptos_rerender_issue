use leptos::{leptos_dom::logging::console_log, prelude::*};
use reactive_stores::{Field, Store};

#[derive(Default, Store, Debug, Clone)]
pub struct State {
  #[store(key: usize = |session| session.id)]
  pub sessions: Vec<Session>,
  pub selected_session: Option<Field<Session>>,
}
#[derive(Default, Store, Debug, Clone)]
pub struct Session {
  pub id: usize,
  pub title: String,
}

#[component]
pub fn App() -> impl IntoView {
  let state = Store::new(State {
    sessions: vec![
      Session { id: 0, title: "Title".to_string() },
      Session { id: 1, title: "Title".to_string() },
      Session { id: 2, title: "Title".to_string() },
    ],
    selected_session: None,
  });

  view! {
    <ul>
      <For each=move || state.sessions() key=|item| item.id().get() let(session)>
        <li on:click=move |_| {
          state.selected_session().set(Some(session.into()));
        }>{move || session.get().title}</li>
      </For>
    </ul>
    {move || {
      state
        .selected_session()
        .get()
        .map(|selected_session| {
          Effect::new(move |prev| {
            console_log(&format!("Previous value: {prev:#?}"));
            selected_session.title().get()
          });
          view! { "Hey" }
        })
    }}
  }
}
