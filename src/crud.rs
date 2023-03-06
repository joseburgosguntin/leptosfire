/// Declares that a struct is has the a [server function](leptos_server) for each `crud` operation. This means that
/// the body of each function will only run on the server, i.e., when the `ssr` feature is enabled.
///
/// If you call one theses server functions from the client (i.e., when the `csr` or `hydrate` features
/// are enabled), it will instead make a network request to the server.
///
/// # Example
///
/// ```ignore
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct User {
///     pub admin: bool,
/// }
///
/// crud!("users", User);
///
/// #[component]
/// pub fn UsersPage(cx: Scope) -> impl IntoView {
///     let insert_user = create_server_action::<InsertUser>(cx);
///     let delete_user = create_server_action::<DeleteUser>(cx);
///     let users = create_resource(
///         cx,
///         move || (
///             insert_user.version().get(),
///             delete_user.version().get()
///         ),
///         move |_| async { get_users().await.unwrap() },
///     );
///     view! { cx,
///         todo!()
///     }
/// }
/// ```
///
/// Note the following:
/// - You must **register** the server function by calling `T::register()` somewhere in your main function.
/// - **The [Scope](leptos_reactive::Scope) comes from the server.** Optionally, the first argument of a server function
///   can be a Leptos [Scope](leptos_reactive::Scope). This scope can be used to inject dependencies like the HTTP request
///   or response or other server-only dependencies, but it does *not* have access to reactive state that exists in the client.
///
/// For more information on server function, see [`leptos::server`].
#[macro_export]
macro_rules! crud {
    ($col:expr, $t:ty) => {
        paste! {
            #[server([<Get $t s>], "/api")]
            async fn [<get_ $t:snake s>]() -> Result<HashMap<String, $t>, ServerFnError> {
                match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
                    Ok(fb) => match fb.at($col).get::<HashMap<String, $t>>().await {
                        Ok(result) => Ok(result),
                        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
                    },
                    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
                }
            }
            #[server([<Get $t>], "/api")]
            async fn [<get_ $t:snake>](id: String) -> Result<$t, ServerFnError> {
                match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
                    Ok(fb) => match fb.at($col).at(&id).get::<$t>().await {
                        Ok(result) => Ok(result),
                        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
                    },
                    Err(e) => Err(ServerFnError::ServerError(e.to_string())),
                }
            }
            #[doc = "Inserts a new " " into " $col "."]
            #[server([<Insert $t>], "/api")]
            async fn [<insert_ $t:snake>](doc: $t) -> Result<(), ServerFnError> {
                match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
                    Ok(fb) => match fb.at($col).set(&doc).await {
                        Ok(_) => {
                            log::info!("User inserted");
                            Ok(())
                        }
                        Err(e) => {
                            log::error!("Error: {:?}", e);
                            Err(ServerFnError::ServerError(e.to_string()))
                        }
                    },
                    Err(e) => {
                        log::error!("Error: {:?}", e);
                        Err(ServerFnError::ServerError(e.to_string()))
                    }
                }
            }
            #[server([<Delete $t>], "/api")]
            async fn [<delete_ $t:snake>](id: String) -> Result<(), ServerFnError> {
                match Firebase::new("https://rust-subject-default-rtdb.firebaseio.com/") {
                    Ok(fb) => match fb.at($col).at(&id).delete().await {
                        Ok(_) => {
                            log::info!("User deleted");
                            Ok(())
                        }
                        Err(e) => {
                            log::error!("Error: {:?}", e);
                            Err(ServerFnError::ServerError(e.to_string()))
                        }
                    },
                    Err(e) => {
                        log::error!("Error: {:?}", e);
                        Err(ServerFnError::ServerError(e.to_string()))
                    }
                }
            }
        }
    };
}
