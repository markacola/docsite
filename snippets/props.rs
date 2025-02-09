/// # Custom Props: props.rs
///
/// Dioxus components use hooks to store state between renders. The `use_state` hooks make it easy to update state from
/// event listeners attached to elements in the component. Whenever the state is modified, the component will be re-rendered.
///
/// Thanks to Rust's ownership rules, it's impossible to misuse the `use_state` hook.
#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}

fn Stateful(cx: Scope<PropBased>) -> Element {
    cx.render(rsx!("Hello {props.name}, you are {props.age} years old!"))
}
