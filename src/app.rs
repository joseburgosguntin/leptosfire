use firebase_rs::Firebase;
use std::{collections::HashMap, fmt::Debug};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="admin" view=|cx| view! { cx, <AdminPage/> }/>
                    <Route path="log" view=|cx| view! { cx, <LogPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    // let insert_user = create_server_action::<InsertUser>(cx);

    // let users = create_resource(
    //     cx,
    //     move || (insert_user.version().get()),
    //     move |_| async { get_users().await.unwrap() },
    // );

    // let users_view = move || {
    //     users.with(cx, |users| {
    //         log::debug!("users: {:?}", users);
    //         users
    //             .clone()
    //             .iter()
    //             .map(|(id, u)| {
    //                 view! { cx,
    //                     <h3>"Users" {id}" "{u.admin}</h3>
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //     })
    // };

    view! { cx,
        // <ActionForm action=insert_user>
        //     <label>
        //         "Add a User"
        //     </label>
        //     <input type="submit" value="Submit"/>
        // </ActionForm>
        <h1>"Welcome to Leptos!"</h1>
        <h2>"This is a simple counter example"</h2>
        // <Transition fallback=move || view! {cx, <p> "Loading users..."</p>} >
        //     {users_view}
        // </Transition>
    }
    // <button on:click=on_click>"Click Me: " {count}</button>
}

#[component]
fn AdminPage(cx: Scope) -> impl IntoView {
    // let insert_user = create_server_action::<InsertUser>(cx);

    // let users = create_resource(
    //     cx,
    //     move || (insert_user.version().get()),
    //     move |_| async { get_users().await.unwrap() },
    // );

    // let users_view = move || {
    //     users.with(cx, |users| {
    //         log::debug!("users: {:?}", users);
    //         users
    //             .clone()
    //             .iter()
    //             .map(|(id, u)| {
    //                 view! { cx,
    //                     <h3>"Users" {id}" "{u.admin}</h3>
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //     })
    // };

    // let user_form = move || view! { cx,
    //     <ActionForm action=insert_user>
    //         <input type="hidden" name="admin" value={"true"}/>
    //         <input type="submit" value="Submit"/>
    //     </ActionForm>
    //     // <label for="admin" >"Admin?"</label>
    //         // <input type="checkbox" name="admin" placeholder=true/>
    //     // <label for="true">"True"</label>
    //         // <input type="radio" id="false" name="admin" value="false"/>
    //         // <label for="false">"False"</label>
    // };

    view! { cx,
        <h1>"Admin Page"</h1>
        // { user_form }
        // <Transition fallback=move || view! { cx, <p>"loading..."</p> }>
        //     { users_view }
        // </Transition>
    }
}

#[component]
fn LogPage(cx: Scope) -> impl IntoView {
    let log_bool = create_server_action::<LogBool>(cx);

    // log::debug!("{:?}", leptos::leptos_server::server_fns_by_path);
    view! { cx,
        <h1>"Log Page"</h1>
        <ActionForm action=log_bool>
            <label for="admin">"Admin?"</label>
            <input type="checkbox" name="admin" placeholder=true/>
            <input type="submit" value="Submit"/>
        </ActionForm>
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    admin: bool,
}

pub struct Admin(bool);

#[server(GetUsers, "/api")]
async fn get_users() -> Result<HashMap<String, User>, ServerFnError> {
    match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
        Ok(fb) => match fb.at("users").get().await {
            Ok(users) => Ok(users),
            Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        },
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(LogBool, "/api")]
pub async fn log_bool(cx: Scope, admin: bool) -> Result<(), ServerFnError> {
    if admin {
        log::info!("Admin is true");
    } else {
        log::info!("Admin is false");
    }
    Ok(())
}

// #[server(InsertUser, "/api")]
// async fn insert_user(admin: String) -> Result<(), ServerFnError> {
//     log::debug!("admin: {:?}", admin);
//     match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
//         Ok(fb) => match fb.at("users").set(&User { admin: match admin {"true" => true, _ => false} }).await {
//             Ok(_) => {
//                 log::info!("User inserted");
//                 Ok(())
//             },
//             Err(e) => {
//                 log::error!("Error: {:?}", e);
//                 Err(ServerFnError::ServerError(e.to_string()))
//             },
//         },
//         Err(e) => {
//             log::error!("Error: {:?}", e);
//             Err(ServerFnError::ServerError(e.to_string()))
//         },
//     }
// }

// #[server(InsertUser, "/api")]
// async fn insert_user(admin: String) -> Result<(), ServerFnError> {
//     log::debug!("admin: {:?}", admin);
//     match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
//         Ok(fb) => match fb.at("users").set(&User { admin: match admin {"true" => true, _ => false} }).await {
//             Ok(_) => {
//                 log::info!("User inserted");
//                 Ok(())
//             },
//             Err(e) => {
//                 log::error!("Error: {:?}", e);
//                 Err(ServerFnError::ServerError(e.to_string()))
//             },
//         },
//         Err(e) => {
//             log::error!("Error: {:?}", e);
//             Err(ServerFnError::ServerError(e.to_string()))
//         },
//     }
// }

// #[server(InsertUser, "/api")]
// async fn insert_user(admin: bool) -> Result<(), ServerFnError> {
//     match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
//         Ok(fb) => match fb.at("users").set(&User { admin }).await {
//             Ok(_) => {
//                 log::info!("User inserted");
//                 Ok(())
//             },
//             Err(e) => {
//                 log::error!("Error: {:?}", e);
//                 Err(ServerFnError::ServerError(e.to_string()))
//             },
//         },
//         Err(e) => {
//             log::error!("Error: {:?}", e);
//             Err(ServerFnError::ServerError(e.to_string()))
//         },
//     }
// }
