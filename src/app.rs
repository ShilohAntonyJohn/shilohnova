use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A,},
    StaticSegment,
};
use web_sys::WheelEvent;
use web_sys::Touch;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_location;
use leptos::*;
use leptos::server::ServerAction;

use leptos::ev;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize}; // Import Serialize here for client-side data structures

// --- Data Structures for Client-Side (MUST MATCH SERVER) ---
// These need to be accessible on the client side for sending data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub content: String,
    // Note: created_at will be set on the server, no need here for client-side input
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogRecord {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub content: String,
    pub link: String,
    // Note: created_at will be set on the server
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectRecord {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub content: String,
    pub link: String,
}

#[leptos::server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<ProjectRecord>, ServerFnError> {
    // The code inside this function only runs on the server.
    use surrealdb::engine::local::Db;
    use surrealdb::Surreal;
    use surrealdb::sql::Thing;
    use serde::Deserialize;

    // This struct is only used by the server function, so it's fine to keep it inside.
    #[derive(Deserialize, Debug)]
    struct ProjectFromDB {
        id: Thing,
        title: String,
        content: String,
        link: String,
    }

    let db = use_context::<Surreal<Db>>()
        .ok_or_else(||->ServerFnError{ ServerFnError::ServerError("Database not provided".to_string())})?;

    let projects1: Vec<ProjectFromDB> = db
        .select("project")
        .await
        .map_err(|e|->ServerFnError{ ServerFnError::ServerError(e.to_string())})?;

    let projects = projects1
        .into_iter()
        .map(|p| ProjectRecord {
            id: p.id.to_string(), // The crucial conversion!
            title: p.title,
            content: p.content,
            link: p.link,
        })
        .collect();

    Ok(projects)
}
#[leptos::server(GetBlogs, "/api")]
pub async fn get_blogs() -> Result<Vec<BlogRecord>, ServerFnError> {
    // The code inside this function only runs on the server.
    use surrealdb::engine::local::Db;
    use surrealdb::Surreal;
    use surrealdb::sql::Thing;
    use serde::Deserialize;

    // This struct is only used by the server function, so it's fine to keep it inside.
    #[derive(Deserialize, Debug)]
    struct BlogFromDB {
        id: Thing,
        title: String,
        content: String,
    }

    let db = use_context::<Surreal<Db>>()
        .ok_or_else(||->ServerFnError{ ServerFnError::ServerError("Database not provided".to_string())})?;

    let blogs1: Vec<BlogFromDB> = db
        .select("blog_post")
        .await
        .map_err(|e|->ServerFnError{ ServerFnError::ServerError(e.to_string())})?;

    let blogs = blogs1
        .into_iter()
        .map(|p| BlogRecord {
            id: p.id.to_string(), // The crucial conversion!
            title: p.title,
            content: p.content,
        })
        .collect();

    Ok(blogs)
}
#[leptos::server(DeleteProject, "/api")]
pub async fn delete_project(id: String) -> Result<(), ServerFnError> {
    use surrealdb::engine::local::Db;
    use surrealdb::Surreal;
    use surrealdb::RecordId;
    let db = use_context::<Surreal<Db>>()
        .ok_or_else(||->ServerFnError{ ServerFnError::ServerError("Database not provided".to_string())})?;
    let record_id = RecordId::from(("project", id));
    let _deleted: Option<ProjectRecord> = db.delete(record_id)
        .await?;

    Ok(())
}

#[leptos::server(DeleteBlog, "/api")]
pub async fn delete_blog(id: String) -> Result<(), ServerFnError> {
    use surrealdb::engine::local::Db;
    use surrealdb::Surreal;
    use surrealdb::RecordId;

    let db = use_context::<Surreal<Db>>()
        .ok_or_else(||->ServerFnError{ ServerFnError::ServerError("Database not provided".to_string())})?;
    let record_id = RecordId::from(("blog_post", id));
    let _deleted: Option<BlogRecord> = db.delete(record_id)
        .await?;

    Ok(())
}
#[derive(Clone, PartialEq)]
struct ContentSection {
    id: u32,
    paragraphs: Vec<String>,
    base_transform_style: String,
    initial_static_rotation_deg: f64,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/shilohnova.css"/>
        <Title text="Shiloh Antony John"/>

        <Router>
            <div class="fixed top-0 left-0 w-full p-4 flex justify-between items-center bg-sandy-beige text-navy-blue-custom text-xl z-50 ">
                <div class="flex-1 text-left px-4">
                    <A href="/projects">"Projects"</A>
                </div>
                <div class="flex-1 text-center text-3xl font-bold px-4">
                    <A href="/">"Shiloh Antony John"</A>
                </div>
                <div class="flex-1 text-right px-4">
                    <A href="/views">"Views"</A>
                </div>
            </div>

            <main class="min-h-screen w-screen relative pt-20 flex justify-center items-start bg-sandy-beige text-navy-blue-custom overflow-hidden">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("projects") view=Projects/>
                    <Route path=StaticSegment("views") view=Views/>
                    <Route path=StaticSegment("contacts") view=Contacts/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("adminpanel") view=AdminPanel/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let main_container_rotation = RwSignal::new(0.0);
    let touch_start_y = RwSignal::new(0.0); // New signal to track touch start position

    let sections = vec![
        ContentSection {
            id: 1,
            paragraphs: vec![
                "Hi, I am Shiloh Antony John, a stoic programmer who aspires to be an entrepreneur to do good to humanity. I am obsessed with gaining knowledge and solving problems with true innovation! Continue scrolling!!!".to_string(),

            ],
            base_transform_style: "transform: translateY(0vh) rotate(0deg);".to_string(),
            initial_static_rotation_deg: 0.0,
        },
        ContentSection {
            id: 2,
            paragraphs: vec![
                "I operate at the digital frontier, where blockchain's revolutionary promise and emerging technologies like AI and smart contracts redefine human trust and commerce.".to_string(),

            ],
            base_transform_style: "transform: translateX(35vw) rotate(90deg);".to_string(),
            initial_static_rotation_deg: 90.0,
        },
        ContentSection {
            id: 3,
            paragraphs: vec![
                "I leverage the unparalleled efficiency of Rust and the elegant simplicity of Python to engineer cutting-edge solutions that redefine what's possible in the tech world.".to_string(),
            ],
            base_transform_style: "transform: translateY(50vh) rotate(180deg);".to_string(),
            initial_static_rotation_deg: 180.0,
        },
        ContentSection {
            id: 4,
            paragraphs: vec![
                "Linux is the bedrock of my development, fueling innovation and scaling my entrepreneurial ventures with its open-source power.".to_string(),
            ],
            base_transform_style: "transform: translateX(-37vw) translateY(7vh) rotate(270deg);".to_string(),
            initial_static_rotation_deg: 270.0,
        },
    ];
    let sections_for_mobile=sections.clone();
    let handle_scroll = move |event: WheelEvent| {
        let delta_y = event.delta_y();
        let rotation_speed_factor = 1.5;

        if delta_y > 0.0 {
            main_container_rotation.set(main_container_rotation.get_untracked() + rotation_speed_factor);
        } else if delta_y < 0.0 {
            main_container_rotation.set(main_container_rotation.get_untracked() - rotation_speed_factor);
        }
        event.prevent_default();
    };
    // New: Touch event handlers
    let handle_touch_start = move |event: web_sys::TouchEvent| {
        // Check if the screen width is above a certain threshold (e.g., 768px for tablets)
        // You can adjust this value to better suit your needs.
        if window().inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0) > 768.0 {
            let touch = event.touches().get(0).expect("should have a touch");
            touch_start_y.set(touch.client_y() as f64);
            event.prevent_default(); // Prevent default only on tablets
        }
    };

    let handle_touch_move = move |event: web_sys::TouchEvent| {
        // Only execute this logic on larger screens
        if window().inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0) > 768.0 {
            let touch = event.touches().item(0).expect("should have a touch");
            let current_y = touch.client_y() as f64;
            let delta_y = touch_start_y.get_untracked() - current_y;
            let rotation_speed_factor = 0.5; // A smaller factor for touch

            main_container_rotation.set(main_container_rotation.get_untracked() + delta_y * rotation_speed_factor);
            touch_start_y.set(current_y);
            event.prevent_default(); // Prevent default only on tablets
        }
    };

    view! {<div class="w-screen flex justify-center items-center bg-sandy-beige  overflow-hidden" on:wheel=handle_scroll on:touchstart=handle_touch_start on:touchmove=handle_touch_move>
            <div // desktop layout
                class="h-screen hidden md:flex transition-transform duration-100 ease-out h-[200vh] w-[200vw] relative flex justify-center items-center text-navy-blue-custom text-2xl md:text-2xl"
                style=move || format!("transform: rotate({}deg);", main_container_rotation.get())
            >
                <For
                    each=move || sections.clone()
                    key=|section| section.id
                    children=move |section| {
                        let current_main_rotation_signal = main_container_rotation;
                        view! {
                            <div
                                class="absolute p-5 text-center w-[50vw]"
                                style=section.base_transform_style.clone()
                            >
                                <For
                                    each=move || section.paragraphs.clone()
                                    key=|para_text| para_text.clone()
                                    children=move |para_text| {
                                        view! {
                                            <p
                                                class="block max-w-[40vw]"
                                                style=move || format!("transform: rotate({}deg);",
                                                    -(current_main_rotation_signal.get() + section.initial_static_rotation_deg)
                                                )
                                            >
                                                {para_text.clone()}
                                            </p>
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </div>
        // --- Mobile Layout ---
            // This container is visible by default and hidden on medium screens and up (`md:hidden`).

            <div class="md:hidden flex flex-col items-center p-8 gap-12 text-center text-xl pt-20 min-h-screen">
                <For
                    each=move || sections_for_mobile.clone()
                    key=|section| section.id
                    children=move |section| {
                        view! {
                            <div>
                                <For
                                    each=move || section.paragraphs.clone()
                                    key=|p| p.clone()
                                    children=|p| view! { <p>{p}</p> }
                                />
                            </div>
                        }
                    }
                />
            </div>
        </div>
        <div class="fixed bottom-2  left-1/2 -translate-x-1/2 text-xl font-bold z-10">
                <A href="/contacts">"Contacts"</A>
            </div>
    }
}

#[component]
fn Projects() -> impl IntoView {

    let location=use_location();

    let projects = Resource::new(
        move || location.pathname.get(),
        |_| async move {
            {
                get_projects().await
            }

        },
    );

    view! {
    <div class="container mx-auto p-4 md:p-8 min-h-screen pt-20">
        <Suspense fallback=|| view! { <p>"Loading projects..."</p> }>
            {move || {
                projects.read().clone().map(|res:Result<Vec<ProjectRecord>, ServerFnError>| match res {
                    Ok(vec) if !vec.is_empty() => view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <For
                                each=move || vec.clone()
                                key=|project| project.id.clone()
                                children=move |project: ProjectRecord| {
                                    view! {
                                        <div class="bg-charcoal-custom rounded-lg shadow-lg p-6">
                                            <h2 class="text-xl font-bold mb-2">{project.title}</h2>
                                            <p>{project.content}</p>
                                            <p><a href=project.link target="_blank"  rel="noopener noreferrer">cat working_and_more.txt</a></p>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any(),

                    Ok(_) => view! { <p>"No projects found"</p> }.into_any(),

                    Err(e) => view! {
                        <p>{format!("Error loading projects: {}", e)}</p>
                    }.into_any(),
                })
                .unwrap_or_else(|| view! { <p>"Loading..."</p> }.into_any())
            }}
        </Suspense>
    </div>
}


}


#[component]
fn Views() -> impl IntoView {

    let location=use_location();

    let blogs = Resource::new(
        move || location.pathname.get(),
        |_| async move {
            {
                get_blogs().await
            }

        },
    );

    view! {
    <div class="container mx-auto p-4 md:p-8 min-h-screen pt-20">
        <Suspense fallback=|| view! { <p>"Loading projects..."</p> }>
            {move || {
                blogs.read().clone().map(|res:Result<Vec<BlogRecord>, ServerFnError>| match res {
                    Ok(vec) if !vec.is_empty() => view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <For
                                each=move || vec.clone()
                                key=|blog| blog.id.clone()
                                children=move |blog: BlogRecord| {
                                    view! {
                                        <div class="bg-charcoal-custom rounded-lg shadow-lg p-6">
                                            <h2 class="text-xl font-bold mb-2">{blog.title}</h2>
                                            <p>{blog.content}</p>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any(),

                    Ok(_) => view! { <p>"No views found"</p> }.into_any(),

                    Err(e) => view! {
                        <p>{format!("Error loading views: {}", e)}</p>
                    }.into_any(),
                })
                .unwrap_or_else(|| view! { <p>"Loading..."</p> }.into_any())
            }}
        </Suspense>
    </div>
}


}

#[component]
fn Contacts() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col justify-center items-center bg-sandy-beige text-navy-blue-custom">
            <h1>"Contact Me!"</h1>
            <p>"Email- shilohantonyjohn@email.com"</p>
            <p>"X- @ShilohAJohn"</p>
            <p>"Github- ShilohAntonyJohn"</p>
        </div>
    }
}


#[component]
fn LoginPage() -> impl IntoView {
    let navigate = use_navigate();
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default(); // Prevent default form submission

        let current_email = email.get_untracked();
        let current_password = password.get_untracked();

        if current_email.is_empty() || current_password.is_empty() {
            message.set("Please fill in both fields.".to_string());
            return;
        }
        let navigate_for_submit = navigate.clone();

        spawn_local(async move {
            #[cfg(feature = "hydrate")]
            {
                use gloo_net::http::Request;

                let current_email = email.get_untracked();
                let current_password = password.get_untracked();

                let request_body = serde_json::json!({
                    "email": current_email,
                    "password": current_password,
                });

                let request = Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .json(&request_body);

                match request {
                    Ok(req_builder) => {
                        match req_builder.send().await {
                            Ok(response) => {
                                if response.status() == 200 {
                                    message.set("Login successful!".to_string());
                                    navigate_for_submit("/adminpanel", Default::default());
                                } else if response.status() == 401 {
                                    message.set("Invalid email or password.".to_string());
                                } else {
                                    message.set(format!("Login failed: Status {}", response.status()));
                                }
                            }
                            Err(e) => {
                                message.set(format!("Error during login: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        message.set(format!("Error building request: {}", e));
                    }
                }
            }
        });
    };

    view! {
        <div class="h-full w-full flex flex-col justify-center items-center bg-sandy-beige text-navy-blue-custom">
            <h1>"Login"</h1>
            <form on:submit=on_submit class="flex flex-col gap-4">
                <input
                    type="email"
                    placeholder="Email"
                    on:input=move |ev| email.set(event_target_value(&ev))
                    prop:value=email
                    class="p-2 border border-gray-300 rounded"
                />
                <input
                    type="password"
                    placeholder="Password"
                    on:input=move |ev| password.set(event_target_value(&ev))
                    prop:value=password
                    class="p-2 border border-gray-300 rounded"
                />
                <button type="submit" class="bg-navy-blue-custom text-sandy-beige p-2 rounded">
                    "Login"
                </button>
            </form>
        <p >{message}</p>
        </div>
    }
}

#[component]
fn AdminPanel() -> impl IntoView {
    let (blog_title, set_blog_title) = signal("".to_string()); // Renamed for clarity
    let (blog_content, set_blog_content) = signal("".to_string());
    let (project_title, set_project_title) = signal("".to_string()); // New signal for project title
    let (project_content, set_project_content) = signal("".to_string()); // New signal for project content
    let (project_link, set_project_link) = signal("".to_string()); // New signal for project link
    let (publish_status, set_publish_status) = signal("".to_string());


    // --- New signals for deletion ---
    let (project_id_to_delete, set_project_id_to_delete) = signal("".to_string());
    let (blog_id_to_delete, set_blog_id_to_delete) = signal("".to_string());

    // --- Action to trigger deletion ---
    let delete_project_action = ServerAction::<DeleteProject>::new();
    let delete_blog_action = ServerAction::<DeleteBlog>::new();


    let location=use_location();
    let location1=location.clone();
    let projects = Resource::new(
        move || (delete_project_action.version().get(), location.pathname.get()),
        |_| async move {
            {
                get_projects().await
            }

        },
    );
    let blogs = Resource::new(
        move || (delete_blog_action.version().get(), location1.pathname.get()),
        |_| async move {
            {
                get_blogs().await
            }

        },
    );
    let on_publish_blog = move |_| {
        let current_title = blog_title.get_untracked();
        let current_content = blog_content.get_untracked();

        if current_title.is_empty() || current_content.is_empty() {
            set_publish_status.set("Blog title or content cannot be empty.".to_string());
            return;
        }

        spawn_local(async move {
            #[cfg(feature = "hydrate")]
            {
                use gloo_net::http::Request;
                let blog_post_data = BlogPost {
                    title: current_title,
                    content: current_content,
                };

                let request_body = serde_json::to_string(&blog_post_data).expect("Failed to serialize blog post");

                let request = Request::post("/api/publish-blog")
                    .header("Content-Type", "application/json")
                    .body(request_body); // Use .body() for raw string

                match request {
                    Ok(req_builder) => {
                        match req_builder.send().await {
                            Ok(response) => {
                                if response.status() == 201 { // 201 Created is typical for successful creation
                                    set_publish_status.set("Blog post published successfully!".to_string());
                                    set_blog_title.set("".to_string()); // Clear fields
                                    set_blog_content.set("".to_string());
                                } else if response.status() == 401 {
                                    set_publish_status.set("Unauthorized. Please login again.".to_string());
                                } else {
                                    set_publish_status.set(format!("Failed to publish blog post: Status {}", response.status()));
                                }
                            }
                            Err(e) => {
                                set_publish_status.set(format!("Error sending blog publish request: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        set_publish_status.set(format!("Error building blog publish request: {}", e));
                    }
                }
            }
        });
    };

    let on_publish_project = move |_| {
        let current_title = project_title.get_untracked();
        let current_content = project_content.get_untracked();
        let current_link = project_link.get_untracked();

        if current_title.is_empty() || current_content.is_empty() {
            set_publish_status.set("Project title or content cannot be empty.".to_string());
            return;
        }

        spawn_local(async move {
            #[cfg(feature = "hydrate")]
            {
                use gloo_net::http::Request;
                let project_data = Project {
                    title: current_title,
                    content: current_content,
                    link: current_link,
                };

                let request_body = serde_json::to_string(&project_data).expect("Failed to serialize project");

                let request = Request::post("/api/publish-project")
                    .header("Content-Type", "application/json")
                    .body(request_body);

                match request {
                    Ok(req_builder) => {
                        match req_builder.send().await {
                            Ok(response) => {
                                if response.status() == 201 {
                                    set_publish_status.set("Project published successfully!".to_string());
                                    set_project_title.set("".to_string()); // Clear fields
                                    set_project_content.set("".to_string());
                                    set_project_link.set("".to_string());
                                } else if response.status() == 401 {
                                    set_publish_status.set("Unauthorized. Please login again.".to_string());
                                } else {
                                    set_publish_status.set(format!("Failed to publish project: Status {}", response.status()));
                                }
                            }
                            Err(e) => {
                                set_publish_status.set(format!("Error sending project publish request: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        set_publish_status.set(format!("Error building project publish request: {}", e));
                    }
                }
            }
        });
    };

    view! {
        <div class="container mx-auto p-4 md:p-8 min-h-screen text-navy-blue-custom-800">
            <h1 class="text-4xl font-bold mb-8 text-center text-navy-blue-custom-700">"Admin Dashboard"</h1>

            <section class="p-6 rounded-lg shadow-lg mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-navy-blue-custom-600">"Publish Blog Post"</h2>
                <div class="mb-4">
                    <label for="blog-title" class="block text-navy-blue-custom-700 text-sm font-bold mb-2">"Title:"</label>
                    <input
                        id="blog-title"
                        type="text"
                        prop:value=blog_title
                        on:input=move |ev| set_blog_title.set(event_target_value(&ev))
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-navy-blue-custom-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500"
                        placeholder="Enter your blog post title"
                    />
                </div>
                <div class="mb-6">
                    <label for="blog-content" class="block text-navy-blue-custom-700 text-sm font-bold mb-2">"Content:"</label>
                    <textarea
                        id="blog-content"
                        prop:value=blog_content
                        on:input=move |ev| set_blog_content.set(event_target_value(&ev))
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-navy-blue-custom-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500 h-32 resize-y" // Adjusted height
                        placeholder="Write your blog post content here..."
                    ></textarea>
                </div>
                <button
                    on:click=on_publish_blog
                    class="bg-navy-blue-custom text-sandy-beige font-bold py-2 px-4 rounded hover:bg-navy-blue-custom-dark focus:outline-none focus:shadow-outline"
                >
                    "Publish Blog Post"
                </button>
            </section>

            <section class="p-6 rounded-lg shadow-lg mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-navy-blue-custom-600">"Publish Project"</h2>
                <div class="mb-4">
                    <label for="project-title" class="block text-navy-blue-custom-700 text-sm font-bold mb-2">"Title:"</label>
                    <input
                        id="project-title"
                        type="text"
                        prop:value=project_title
                        on:input=move |ev| set_project_title.set(event_target_value(&ev))
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-navy-blue-custom-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500"
                        placeholder="Enter your project title"
                    />
                </div>
                <div class="mb-6">
                    <label for="project-content" class="block text-navy-blue-custom-700 text-sm font-bold mb-2">"Content:"</label>
                    <textarea
                        id="project-content"
                        prop:value=project_content
                        on:input=move |ev| set_project_content.set(event_target_value(&ev))
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-navy-blue-custom-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500 h-32 resize-y" // Adjusted height
                        placeholder="Write your project content here..."
                    ></textarea>
                </div>
            <div class="mb-4">
                    <label for="project-link" class="block text-navy-blue-custom-700 text-sm font-bold mb-2">"Link:"</label>
                    <input
                        id="project-link"
                        type="text"
                        prop:value=project_link
                        on:input=move |ev| set_project_link.set(event_target_value(&ev))
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-navy-blue-custom-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500"
                        placeholder="Enter your project's link"
                    />
                </div>
                <button
                    on:click=on_publish_project
                    class="bg-navy-blue-custom text-sandy-beige font-bold py-2 px-4 rounded hover:bg-navy-blue-custom-dark focus:outline-none focus:shadow-outline"
                >
                    "Publish Project"
                </button>
            </section>

            <p class="mt-4 text-sm text-navy-blue-custom-600 text-center">{publish_status}</p>
            // deletion code
            <div class="mb-6">
                    <h3 class="text-xl font-semibold mb-2">"Delete a Project"</h3>
                    <div class="flex gap-2">
                        <input
                            type="text"
                            on:input=move |ev| set_project_id_to_delete.set(event_target_value(&ev))
                            prop:value=project_id_to_delete
                            class="shadow appearance-none border rounded w-full py-2 px-3"
                            placeholder="Paste Project ID to delete"
                        />
                        <button
                            on:click=move |_| {
                                delete_project_action.dispatch(DeleteProject { id: project_id_to_delete.get_untracked() });
                                set_project_id_to_delete.set("".to_string()); // Clear input after dispatch
                            }
                            class="bg-red-600 text-white font-bold py-2 px-4 rounded hover:bg-red-700"
                        >"Delete"</button>
                    </div>
                </div>
                    // delete blog
                <div class="mb-6">
                    <h3 class="text-xl font-semibold mb-2">"Delete a Blog Post"</h3>
                     <div class="flex gap-2">
                        <input
                            type="text"
                            on:input=move |ev| set_blog_id_to_delete.set(event_target_value(&ev))
                            prop:value=blog_id_to_delete
                            class="shadow appearance-none border rounded w-full py-2 px-3"
                            placeholder="Paste Blog ID to delete"
                        />
                        <button
                           on:click=move |_| {
                                delete_blog_action.dispatch(DeleteBlog { id: blog_id_to_delete.get_untracked() });
                                set_blog_id_to_delete.set("".to_string()); // Clear input after dispatch
                            }
                            class="bg-red-600 text-white font-bold py-2 px-4 rounded hover:bg-red-700"
                        >"Delete"</button>
                    </div>
                </div>

        <Suspense>
        <div class="">
            {move || {
                projects.read().clone().map(|res:Result<Vec<ProjectRecord>, ServerFnError>| match res {
                    Ok(vec) if !vec.is_empty() => view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <For
                                each=move || vec.clone()
                                key=|project| project.id.clone()
                                children=move |project: ProjectRecord| {
                                    view! {
                                        <div class="bg-charcoal-custom rounded-lg shadow-lg p-6">
                                            <h1 class="text-xl font-bold mb-2">{project.id}</h1>
                                            <h2 class="text-xl font-bold mb-2">{project.title}</h2>
                                            <p>{project.content}</p>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any(),

                    Ok(_) => view! { <p>"No projects found"</p> }.into_any(),

                    Err(e) => view! {
                        <p>{format!("Error loading projects: {}", e)}</p>
                    }.into_any(),
                })
                .unwrap_or_else(|| view! { <p>"Loading..."</p> }.into_any())
            }}
    </div>
        <div class="pt-20">
            {move || {
                blogs.read().clone().map(|res:Result<Vec<BlogRecord>, ServerFnError>| match res {
                    Ok(vec) if !vec.is_empty() => view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <For
                                each=move || vec.clone()
                                key=|blog| blog.id.clone()
                                children=move |blog: BlogRecord| {
                                    view! {
                                        <div class="bg-charcoal-custom rounded-lg shadow-lg p-6">
                                            <h1 class="text-xl font-bold mb-2">{blog.id}</h1>
                                            <h2 class="text-xl font-bold mb-2">{blog.title}</h2>
                                            <p>{blog.content}</p>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any(),

                    Ok(_) => view! { <p>"No views found"</p> }.into_any(),

                    Err(e) => view! {
                        <p>{format!("Error loading views: {}", e)}</p>
                    }.into_any(),
                })
                .unwrap_or_else(|| view! { <p>"Loading..."</p> }.into_any())
            }}
    </div>
        </Suspense>
        </div>
    }
}