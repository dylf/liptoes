use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Liptoes - Home"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Todos/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn Todos(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<String>>(
        cx,
        vec![
            String::from("Do a barrel roll!"),
            String::from("Play a game"),
        ],
    );

    view! { cx,
        <h1>"Welcome to Liptoes!"</h1>
        <ul>
          <For
            each=todos
            key=|todo| todo.clone()
            view=move |cx, todo: String| {
                    view! {cx,
                        <li>{todo}</li>
                    }
                }
            />

        </ul>
    }
}
