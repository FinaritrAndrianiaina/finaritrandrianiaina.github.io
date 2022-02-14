use yew::prelude::*;


#[derive(Debug)]
struct Model {
    value: i8,
    step: u8
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value:0,
        step:1
    });
    let on_click_increment = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Model {
                value: state.value+ state.step as i8,
                step: state.step
            })
        })
    };
    let on_click_decrement = {
        let state = state.clone();
        Callback::from(move |_| {
            if ( state.value - state.step as i8 )  > 0  {
                state.set(Model {
                    value: state.value - state.step as i8,
                    step: state.step
                })
            } else {
                state.set(Model {
                    value: 0,
                    step: state.step
                })
            }
        })
    };
    let on_change_number = {
        let state = state.clone();

        Callback::from(move |e: InputEvent| {
            state.set(Model {
                value: state.value,
                step: e.data().unwrap_or("0".to_string()).parse::<u8>().unwrap_or(1)
            })
        })
    };
    html! {
        <>
            <button onclick={on_click_increment}>{format!("+ {} ",state.step)}</button>
            <button onclick={on_click_decrement}>{format!("- {}",state.step)}</button>
            <h1>{ "Hello World" }</h1>
            <input type={"number"} min={1} name="Step" oninput={on_change_number} />
            <p>{state.value}</p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}