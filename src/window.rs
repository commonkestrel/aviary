use leptos::*;

#[component]
pub fn Window(children: Children) -> impl IntoView {
    view! {
        <div class="window">
            <div class="titlebar">
                <h2>
                </h2>
                <div class="titlebar-buttons">
                    <button class="titlebar-button minimize"></button>
                    <button class="titlebar-button close"></button>
                </div>
            </div>
            <div class="content">
                {children()}
            </div>
        </div>
    }
}
