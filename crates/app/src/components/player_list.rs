use data::Test;
use leptos::*;

use crate::sse::Sse;

#[component]
pub fn PlayerList() -> impl IntoView {
    let sse = use_context::<Sse>().expect("there to be a sse");

    let test = sse.subscribe::<Test>("test");

    view! {
        <table class="table w-full shadow bg-base-100">
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Score"</th>
                    <th>"Time"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>"Player 1"</td>
                    <td>{ move || test.get().number }</td>
                    <td>"1:00:00"</td>
                </tr>
                <tr>
                    <td>"Player 2"</td>
                    <td>"200"</td>
                    <td>"2:00:00"</td>
                </tr>
                <tr>
                    <td>"Player 3"</td>
                    <td>"300"</td>
                    <td>"3:00:00"</td>
                </tr>
            </tbody>
        </table>
    }
}
