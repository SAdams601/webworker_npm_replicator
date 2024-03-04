use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_agent::oneshot::{use_oneshot_runner, OneshotProvider};

use crate::agent::{FibonacciTask, Postcard};
use crate::is_even::is_even;

pub mod agent;
pub mod is_even;


#[function_component]
fn Main() -> Html {
    let input_value = use_state_eq(|| 44);
    let output = use_state(|| "Try out some fibonacci calculations!".to_string());
    let fib_task = use_oneshot_runner::<FibonacciTask>();    

    let calculate = {
        let input_value = *input_value;
        let output = output.clone();
        move |_e: MouseEvent| {
            let fib_agent = fib_task.clone();
            let output = output.clone();

            spawn_local(async move {
                // start the worker
                let output_value = fib_agent.run(input_value).await;

                output.set(format!("Fibonacci value: {}", output_value));
            });
        }
    };

    let on_input_change = {
        let input_value = input_value.clone();
        move |e: InputEvent| {
            input_value.set(
                e.target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .parse()
                    .expect("failed to parse"),
            );
        }
    };

    html! {
        <>
            <h1>{ "Web worker demo" }</h1>
            <p>{ "Submit a value to calculate, then increase the counter on the main thread!"} </p>
            <p>{ "Large numbers will take some time!" }</p>
            <h3>{ "Output: " } { &*output }</h3>
            <br />
            <input type="number" value={input_value.to_string()} max="50" oninput={on_input_change} />
            <button onclick={calculate}>{ "submit" }</button>
            <br /> <br />
            <h3>{ "Is Even: " } { is_even((*input_value).into()) }</h3>
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <OneshotProvider<FibonacciTask, Postcard> path="/worker.js">
            <Main />
        </OneshotProvider<FibonacciTask, Postcard>>
    }
}