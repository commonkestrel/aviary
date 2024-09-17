mod window;
mod terminal;

use terminal::Terminal;
use window::Window;
use leptos::*;

fn main() {
    mount_to_body(|| view!{
        <p>"Hello world"</p>
        <Window>
            <Terminal></Terminal>
        </Window>
        <div class="taskbar">

        </div>
    })
}
