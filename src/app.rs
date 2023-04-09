use leptos::html::Div;
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
        <Stylesheet id="leptos" href="/pkg/tatlow-easter.css"/>

        // sets the document title
        <Title text="Tatlow Easter Egg Hunt!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path=":id" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let data = create_resource(
        cx,
        move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
        move |id| get_value(id),
    );

    // let loading = data.loading();
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
    };

    view! { cx,
        <div class="container" on:click=on_click>
            <div class:pyro={move || count() >= 5}>
                <div class="before"></div>
                <div class="after"></div>
            </div>
            <div
                class="easter-animation"
                class:step-one={move || count() == 1}
                class:step-two={move || count() == 2}
                class:step-three={move || count() == 3}
                class:step-four={move || count() == 4}
                class:step-five={move || count() >= 5}
            >
                <div class="money">
                    <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
                    {move || {
                        data.read(cx).map(|value| view! { cx,  "$"{value} })
                      }
                    }
                    </Suspense>

                </div>
                <div class="chick">
                    <div class="beak"></div>
                </div>
                <div class="back-flower">
                    <div class="daisy">
                        <div class="flower"></div>
                    </div>
                </div>
                <div class="egg">
                    <div class="shell"></div>
                </div>
                <div class="egg-top">
                    <div class="shell-top"></div>
                </div>
                <div class="daisy">
                    <div class="flower"></div>
                </div>
                <div class="grass"></div>
                <p class="text">"- tap on the egg -"</p>
            </div>
        </div>
    }
}

async fn get_value(id: String) -> u64 {
    match id.as_str() {
        "egg1" => 5,
        "egg2" => 20,
        "egg3" => 5,
        "egg4" => 10,
        "egg5" => 5,
        "egg6" => 20,
        "egg7" => 5,
        "egg8" => 50,
        "egg9" => 10,
        _ => 0,
    }
}
