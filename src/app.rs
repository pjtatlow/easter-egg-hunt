use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tatlow-easter1.css"/>

        // sets the document title
        <Title text="Tatlow Easter Egg Hunt!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path=":id" view=|| view! { <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let params = use_params_map();
    let egg = params.with(|p| p.get("id").cloned().unwrap_or_default());
    let value = match egg.as_str() {
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
    };

    // let loading = data.loading();
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
    };

    view! {
        <div class="container">
            <div class:pyro={move || count.get() >= 5}>
                <div class="before"></div>
                <div class="after"></div>
            </div>
            <div
                class="easter-animation"
                class:step-one={move || count.get() == 1}
                class:step-two={move || count.get() == 2}
                class:step-three={move || count.get() == 3}
                class:step-four={move || count.get() == 4}
                class:step-five={move || count.get() >= 5}
            >
                <div class="money">
                     "$"{value}

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
                <button
                    class="open-button"
                    class:hidden={move || count.get() >= 5}
                    on:click=on_click

                >"open the egg"</button>

            </div>
        </div>
    }
}
