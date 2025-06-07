use leptos::{either::Either, leptos_dom::logging::console_log, prelude::*};
use reactive_stores::{Field, OptionStoreExt, Store, StoreFieldIterator};

#[derive(Default, Store, Debug, Clone)]
pub struct State {
  #[store(key: usize = |session| session.id)]
  pub sessions: Vec<Session>,
  pub selected_session: Option<Field<Session>>,
}

#[derive(Default, Store, Debug, Clone, PartialEq, Eq)]
pub struct Session {
  pub id: usize,
  pub title: String,
}

#[component]
pub fn App() -> impl IntoView {
  let state = Store::new(State {
    sessions: vec![
      Session { id: 0, title: "Title1".to_string() },
      Session { id: 1, title: "Title2".to_string() },
      Session { id: 2, title: "Title30".to_string() },
    ],
    selected_session: None,
  });

  view! {
    <ul>
      <For each=move || state.sessions() key=|item| item.id().get() let(session)>
        <li on:click=move |_| {
          state.selected_session().set(Some(session.into()));
        }>{session.title()}</li>
      </For>
    </ul>
    <button on:click=move |_| {
      state.selected_session().get().map(|ss| ss.title().update(|title| title.push_str("0")));
    }>"Modify Selected Title"</button>
    <Show when=move || state.selected_session().get().is_some()>
      <Content session={state.selected_session().unwrap()} />
    </Show>
  }
}

#[component]
pub fn Content(#[prop(into)] session: Field<Field<Session>>) -> impl IntoView {
  console_log("Hey!");
  view! {
    <input type="text" bind:value=session.get().title() />
    {move || {
      if session.get().title().with(String::len) == 7 {
        Either::Right(view! { "Title has seven letters" })
      } else {
        Either::Left(view! { "Title doesn't have seven letters" })
      }
    }}
  }
}
