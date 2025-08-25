#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::body::Body;
    use axum::extract::{FromRef, Json, State};
    use axum::http::{Request, StatusCode};
    use axum::middleware::{self, Next};
    use axum::response::{IntoResponse, Response};
    use axum::routing::{get, post};
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos::context::provide_context;
    use leptos_axum::{generate_route_list, render_app_to_stream_with_context};
    use serde::{Deserialize, Serialize};
    use shilohnova::app::*;
    use surrealdb::engine::local::{Db, RocksDb};
    use surrealdb::Surreal;
    use tower::ServiceExt;
    use tower_cookies::cookie::SameSite;
    use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
    use tower_http::services::ServeDir;
    use leptos_axum::handle_server_fns_with_context;
    // --- Axum State Struct ---
    #[derive(Clone, FromRef)]
    struct AppState {
        leptos_options: LeptosOptions,
        db: Surreal<Db>,
    }

    // --- Authentication Payload (from client form) ---
    #[derive(Debug, Deserialize, Clone)]
    pub struct AuthPayload {
        email: String,
        password: String,
    }

    // --- Data Structures for SurrealDB ---
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlogPost {
        pub title: String,
        pub content: String,
        // The `created_at` field will be set by SurrealDB's `time::now()`
        // so we don't need to pass it in from the client
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Project {
        pub title: String,
        pub content: String,
        pub link: String,
        // The `created_at` field will be set by SurrealDB's `time::now()`
    }

    // This is the SurrealDB connection
    async fn db_connect() -> Result<Surreal<Db>, surrealdb::Error> {
        let db = Surreal::new::<RocksDb>("./data/shilohnova.db").await?;
        db.use_ns("site").use_db("main").await?;
        Ok(db)
    }

    // --- Authentication Handler ---
    async fn login_handler(
        State(_db): State<Surreal<Db>>,
        cookies: Cookies,
        Json(payload): Json<AuthPayload>,
    ) -> Result<Json<String>, StatusCode> {
        log!("Received login request for: {}", payload.email);

        if payload.email == "test@example.com" && payload.password == "password123" {
            let mut cookie = Cookie::new("session_token", "mock_session_token_123");
            cookie.set_path("/");
            cookie.set_same_site(SameSite::Lax);
            cookie.set_http_only(true);
            cookies.add(cookie);
            log!("User authenticated, cookie set.");
            Ok(Json("Login successful!".to_string()))
        } else {
            log!("Authentication failed for: {}", payload.email);
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    // --- Protected Routes Middleware ---
    async fn auth_middleware(
        cookies: Cookies,
        request: Request<Body>,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let session_token = cookies.get("session_token").map(|c| c.value().to_string());

        if session_token == Some("mock_session_token_123".to_string()) {
            Ok(next.run(request).await)
        } else {
            log!("Unauthorized access attempt to a protected route.");
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    // --- API Handlers ---

    // Handler to publish a new blog post
    async fn publish_blog_post(
        State(db): State<Surreal<Db>>,
        Json(payload): Json<BlogPost>,
    ) -> Result<StatusCode, StatusCode> {
        log!("Received new blog post: {}", payload.title);

        // Correct type annotation for the result of `db.create`
        let created_post: Result<Option<BlogPost>, _> =
            db.create("blog_post").content(payload).await;

        created_post.map_err(|e| {
            log!("Failed to write to SurrealDB: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::CREATED)
    }

    // Handler to publish a new project
    async fn publish_project(
        State(db): State<Surreal<Db>>,
        Json(payload): Json<Project>,
    ) -> Result<StatusCode, StatusCode> {
        log!("Received new project: {}", payload.title);

        // Correct type annotation for the result of `db.create`
        let created_project: Result<Option<Project>, _> =
            db.create("project").content(payload).await;

        created_project.map_err(|e| {
            log!("Failed to write to SurrealDB: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::CREATED)
    }
    async fn server_fn_handler(
        State(app_state): State<AppState>,
        request: Request<Body>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            // Argument 1: The closure
            move || {
                provide_context(app_state.db.clone());
            },
            // Argument 2: The request
            request,
        )
            .await
    }

    // Serve the file system and handle errors

    async fn file_and_error_handler(
        State(app_state): State<AppState>,
        req: Request<Body>,
    ) -> Response {
        let root = app_state.leptos_options.site_root.clone();

        match ServeDir::new(root.to_string()).oneshot(req).await {
            Ok(res) => res.map(Body::new).into_response(),
            Err(e) => {
                log!("file_and_error_handler error: {e:?}");

                // In case of a file serving error, fall back to rendering the Leptos app
                // Clone leptos_options for the handler and the request builder separately
                let leptos_options_for_handler = app_state.leptos_options.clone();
                let leptos_options_for_req = app_state.leptos_options.clone();

                let handler = leptos_axum::render_app_to_stream_with_context(
                    move || provide_context(leptos_options_for_handler.clone()),
                    move || view! { <App/> },
                );

                let new_req = Request::builder()
                    .uri(leptos_options_for_req.site_addr.to_string())
                    .body(Body::empty())
                    .unwrap();

                handler(new_req).await.into_response()
            }
        }
    }

    // --- main ---
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    // Connect to the database
    let db = match db_connect().await {
        Ok(db) => db,
        Err(e) => {
            log!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };
    log!("Connected to SurrealDB at: {}", "./data/shilohnova.db");

    // Create the shared application state
    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        db:db.clone(),
    };

    // Create the protected router (requires auth)
    let leptos_options_clone_for_context_admin = leptos_options.clone();
    let leptos_options_clone_for_shell_admin = leptos_options.clone();
    let db_clone_for_admin = app_state.db.clone();

    let protected_routes = Router::new()
        .route("/api/publish-blog", post(publish_blog_post))
        .route("/api/publish-project", post(publish_project))
        .route("/api/admin/{*fn_name}", post(server_fn_handler))
        .route(
            "/adminpanel",
            get(render_app_to_stream_with_context(
                move || {
                    provide_context(leptos_options_clone_for_context_admin.clone());
                    provide_context(db_clone_for_admin.clone());
                },
                move || shell(leptos_options_clone_for_shell_admin.clone()),
            )),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // Create a router for the public routes
    let mut public_routes = Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/{*fn_name}", post(server_fn_handler));


    for route in routes
        .into_iter()
        .filter(|route| route.path() != "/adminpanel")
    {
        let leptos_options_clone_for_context = app_state.leptos_options.clone();
        let leptos_options_clone_for_shell = app_state.leptos_options.clone();
        let db_clone_for_context = app_state.db.clone();
        public_routes = public_routes.route(
            route.path(),
            get(render_app_to_stream_with_context(
                // We provide the database connection here, so the server functions can access it.
                move || {
                    let leptos_options = leptos_options_clone_for_context.clone();
                    // Provide the database connection as a context
                    provide_context(db_clone_for_context.clone());
                    provide_context(leptos_options);
                },
                move || shell(leptos_options_clone_for_shell.clone()),
            )),
        );
    }

    // Combine the routers, add state, fallback, and final layers
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(CookieManagerLayer::new());

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}