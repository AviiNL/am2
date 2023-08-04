use crate::{app_state::AppState, components::*};
use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let state = use_context::<AppState>().expect("there to be a state");

    view! {
        <div class="navbar bg-base-100 shadow-xl rounded-box gap-4">
            <div class="flex-none">
                <Dropdown>
                    <li><a>"Dashboard"</a></li>
                    <li><a>"Logs"</a></li>
                    <li><a>"Mods"</a></li>
                    <li><a>"Missions"</a></li>
                    <Divider />
                    <li><a>"Profile"</a></li>
                    <li><a>"Logout"</a></li>
                </Dropdown>
            </div>
            <div class="flex-1">
                <div class="text-sm breadcrumbs">
                    <ul>
                        <li>"Dashboard"</li>
                        // <li><a>"Documents"</a></li>
                        // <li>"Add Document"</li>
                    </ul>
                </div>
            </div>
            <div class="flex-none">
                { move || state.theme.get() }
            </div>
        </div>
    }
}
