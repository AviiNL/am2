use leptos::*;

#[component]
pub fn Dropdown(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="dropdown">
            <label tabindex="0" class="btn btn-ghost btn-square">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /></svg>
            </label>
            <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow shadow-lg bg-base-200 rounded-box w-52">
                { children }
            </ul>
        </div>
    }
}
