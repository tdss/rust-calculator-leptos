use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rand::Rng;
use std::time::Instant;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/test-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/real-life-gamification" view=Book/>
                    <Route path="/calc-master" view=CalcMaster/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Book() -> impl IntoView {
    view! {
        <h1>"Real Life Gamification!"</h1>
    }

}
#[derive(Debug, Clone)]
struct LevelData {
    key: String,
    number1: RwSignal<i32>,
    number2: RwSignal<i32>,
    result: RwSignal<i32>,
    errors: RwSignal<i32>,
    time: RwSignal<i32>,
}

fn CheckResult() {

}

#[component]
fn CalcMaster() -> impl IntoView {

    let start_level = create_rw_signal(10);
    let start_level_max = create_rw_signal(40);
    let mut rng = rand::thread_rng();
    let (level, set_level) = create_signal(1);
    let (number1, set_number1) = create_signal(start_level.get());
    let (number2, set_number2) = create_signal(start_level_max.get()+level.get());
    let (result, set_result) = create_signal("".to_string());
    let (correct_result, set_correct_result) = create_signal(0);
    let (data, set_data) = create_signal(vec![
        LevelData {
            key: "1".to_string(),
            number1: create_rw_signal(10),
            number2: create_rw_signal(10),
            result: create_rw_signal(0),
            errors: create_rw_signal(0),
            time: create_rw_signal(0),

        },
         
    ]);
    fn CheckResult() {
        
    }
     // let on_click = move
    set_number1(data.get()[0].number1.get());
    set_number2(data.get()[0].number2.get());
    set_correct_result(data.get()[0].number1.get() * data.get()[0].number2.get());
    view! {
        <h1>"Calculate this!"</h1>
        <h2>Level <span>{level}</span> numbers between {start_level.get()}..{start_level_max.get()}</h2>
        <h2>{number1} x {number2} = ?</h2>
        <Form method="GET" action="">
            <input type="text" name="result" placeholder="Type your answer here" 
            on:input=move |ev| {
                set_result(event_target_value(&ev));
                let keyresult = event_target_value(&ev).parse::<i32>().unwrap();
                if (keyresult == correct_result()) {
                    
                    let mut n1: i32 = rng.gen_range(start_level.get()..=start_level.get()+level.get());
                    let mut n2: i32 = rng.gen_range(start_level.get()..=start_level.get()+level.get());
                    set_correct_result(n1 * n2);
                    set_level.update(|level| *level += 1);
                    set_data.update(|data: &mut Vec<LevelData> | data.push(
                    LevelData {
                        key: level.get().to_string(),
                        number1: create_rw_signal(n1),
                        number2: create_rw_signal(n2),
                        result: create_rw_signal(0),
                        errors: create_rw_signal(0),
                        time: create_rw_signal(0),
                    }
                    ));
                    set_result("".to_string());
                     logging::log!("{:?}", data.get());
                }
            }
            prop:value=result 
            />
        </Form>
        <p>You entered: {result}, should be: {correct_result}
        <br/>Is correct? {}</p>
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>x: {child.key}: {child.number1} X {child.number2}</p>
        </For>
        <button on:click=move |_| {
            data.with(|data| {
                 // for row in data {
                     // row.value.update(|value| *value *=2);
                 // };
             });
        }>ADD</button>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
