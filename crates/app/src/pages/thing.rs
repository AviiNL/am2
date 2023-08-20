use leptos::*;

#[server(DoThing, "/api")]
pub async fn do_thing() -> Result<String, ServerFnError> {
    use data::repository::UserRepository;
    let user_repository = use_context::<UserRepository>().expect("UserRepository not found");

    let result = match user_repository.do_something().await {
        Ok(result) => result,
        Err(e) => return Err(ServerFnError::ServerError(format!("Failed to do something: {}", e))),
    };

    Ok(result)
}

#[component]
pub fn Thing() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                let result = do_thing().await;
                leptos::log!("Result: {:?}", result);
            });
        }>
            "Do Thing!"
        </button>
    }
}
