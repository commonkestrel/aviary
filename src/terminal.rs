use leptos::*;

#[component]
pub fn Terminal() -> impl IntoView {
    let (output, set_output) = create_signal("/ >".to_string());

    view! {
        <div class="terminal">
            <p class="content">{output}</p>
        </div>
    }
}