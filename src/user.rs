use crate::crud;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use firebase_rs::Firebase;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub admin: bool,
}

crud!("users", User);

/// Renders the all the users and their admin status.\
/// Also allows the admins to insert new users and delete existing users.\
/// The new users could be admins or not.\
#[component]
pub fn UsersPage(cx: Scope) -> impl IntoView {
    let insert_user = create_server_action::<InsertUser>(cx);
    let delete_user = create_server_action::<DeleteUser>(cx);

    let users = create_resource(
        cx,
        move || (insert_user.version().get(), delete_user.version().get()),
        move |_| async { get_users().await.unwrap() },
    );

    view! { cx,
        <Title text="Users - LeptosFire"/>

        <h1>"Users Page"</h1>
        <ActionForm action=insert_user>
            <label for="admin">"Admin?"</label>
            <input type="checkbox" name="admin" placeholder=true/>
            <input type="submit" value="Submit"/>
        </ActionForm>
        <Transition fallback=move || view! { cx, <p>"loading..."</p> }>
            <ul>
                {
                    move || {
                        match users.read(cx) {
                            Some(users) => {
                                users
                                    .into_iter()
                                    .map(move |(id, u)| {
                                        view! { cx,
                                            <li>
                                                {id.clone()}" "{u.admin}
                                                <ActionForm action=delete_user>
                                                    <input type="hidden" name="id" value={id}/>
                                                    <input type="submit" value="Delete"/>
                                                </ActionForm>
                                            </li>
                                        }

                                    })
                                    .collect::<Vec<_>>()
                            }
                            None => {
                                vec![view! { cx,
                                    <li>"No Users"</li>
                                }]
                            }
                        }
                    }
                }
            </ul>
        </Transition>
    }
}
