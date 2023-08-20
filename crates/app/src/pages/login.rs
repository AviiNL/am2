use leptos::*;

use crate::app_state::AppState;

#[component]
pub fn Login() -> impl IntoView {
    let state = use_context::<AppState>().expect("there to be a state");

    view! {
        <section>
            <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
                <div class="w-full bg-base-100 rounded-lg shadow md:mt-0 sm:max-w-md xl:p-0">
                    <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
                        <h1 class="text-xl font-bold leading-tight tracking-tight md:text-2xl">
                            { move || state.theme.get() } "Sign in to your account"
                        </h1>
                        <form class="space-y-4 md:space-y-6" action="#">
                            <div>
                                <label for="email" class="block mb-2 text-sm font-medium">"Your email"</label>
                                <input type="email" name="email" id="email" class="input input-bordered w-full" placeholder="name@company.com" required=""/>
                            </div>
                            <div>
                                <label for="password" class="block mb-2 text-sm font-medium">"Password"</label>
                                <input type="password" name="password" id="password" placeholder="••••••••" class="input input-bordered w-full" required=""/>
                            </div>
                            <div class="flex items-center justify-between">
                                <div class="flex items-start">
                                    <div class="flex items-center h-5">
                                        <input id="remember" aria-describedby="remember" type="checkbox" class="checkbox" />
                                    </div>
                                    <div class="ml-3 text-sm">
                                        <label for="remember" class="block mb-2 text-sm font-medium">"Remember me"</label>
                                    </div>
                                </div>
                                <a href="#" class="text-sm font-medium hover:underline">"Forgot password?"</a>
                            </div>
                            <button type="submit" class="btn btn-primary w-full">"Sign in"</button>
                            <p class="text-sm font-light">
                                "Don't have an account yet? "<a href="#" class="font-medium hover:underline">"Sign up"</a>
                            </p>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}
