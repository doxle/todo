mod canvas;
use canvas::Canvas;
mod todo;
use todo::Todo;
mod home;
use home::Home;
mod upload;
use upload::Upload;
mod api;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Todo {},
    #[route("/checklist")]
    Home{},
    #[route("/upload")]
    Upload {},
    #[route("/canvas")]
    Canvas {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        // PARENT DIV SECTION
        div { class: "hidden md:flex h-[55px] w-full items-center justify-between bg-[rgb(255,255,255)] border-b border-bg-rgb(230,230,230)",
            //LEFT DIV SECTION
            div { class: "px-4 flex items-center text-[13px] font-extralight font-lexend tracking-wider",
                img { src: "/assets/dog2.svg", alt: "logo" }
            }

            //CENTER DIV SECTION
            div {
                // id: "navbar",
                class: "bg-[rgb(255,255,255)] h-[55px] flex flex-1 items-center justify-center space-x-0 px-2  ",


                //HOME LINK
                Link {
                    to: Route::Home {},
                    active_class: Some("bg-blue-50".to_string()),
                    div {
                        // This has to be relative as there is abolsute inside refferring to the relative
                        class: "px-0 group w-[50px] h-[55px] hover:bg-blue-100 hover:bg-opacity-100 flex items-center justify-center",

                        img { src: "/assets/home-black.svg", alt: "Home" }
                                        // img{
                    //     src: "/assets/home-skinny.svg",
                    //     alt:"Home Hover",
                    //     class:"absolute inset-0 m-auto opacity-0 group-hover:opacity-100 transition-opacity duration-200"
                    // }
                    }
                }

                //CHECKLIST LINK
                Link {
                    to: Route::Todo {},
                    active_class: Some("bg-blue-50".to_string()),
                    div {
                        // This has to be relative as there is absolute inside refferring to the relative
                        class: "px-0 group w-[50px] h-[55px] hover:bg-blue-100 hover:bg-opacity-100 flex items-center justify-center",

                        img { src: "/assets/circle-black.svg", alt: "Checklist" }
                    }
                }

                //UPLOAD LINK
                Link {
                    to: Route::Upload {},
                    active_class: Some("bg-blue-50".to_string()),
                    div {
                        // Ensure this container is relative
                        class: " overflow-visible relative group px-0 w-[50px] h-[55px] hover:bg-blue-100 hover:bg-opacity-100  active:bg-blue-500  flex items-center justify-center",

                        img { src: "/assets/upload-black.svg", alt: "Upload" }

                        // Tooltip container
                        div { class: "absolute bottom-full  left-1/2 transform -translate-x-1/2 mb-2 bg-red-500 text-white text-sm rounded px-2 py-1 shadow-md opacity-0 group-hover:opacity-100 transition-opacity",
                            "Upload Plans"
                        }
                    }
                }

                //CANVAS LINK
                Link {
                    to: Route::Canvas {},
                    active_class: Some("bg-blue-50".to_string()),
                    div {
                        // Ensure this container is relative
                        class: " overflow-visible relative group px-0 w-[50px] h-[55px] hover:bg-blue-100 hover:bg-opacity-100 flex items-center justify-center",

                        img { src: "/assets/plan-black.svg", alt: "Canvas" }

                        // Tooltip container
                        div { class: "absolute bottom-full  left-1/2 transform -translate-x-1/2 mb-2 bg-red-500 text-white text-sm rounded px-2 py-1 shadow-md opacity-0 group-hover:opacity-100 transition-opacity",
                            "Canvas"
                        }
                    }
                }

                //BLOG LINK
                Link { to: Route::Blog { id: 1 },
                    div {
                        // Ensure this container is relative
                        class: " overflow-visible relative group px-0 w-[50px] h-[55px] hover:bg-blue-100 hover:bg-opacity-100 flex items-center justify-center",

                        // img {
                        //     src: "/assets/upload.svg",
                        //     alt: "Blog",
                        //     // class: "h-[24px] w-[24px]"
                        // }

                        // Tooltip container
                        div { class: "absolute bottom-full  left-1/2 transform -translate-x-1/2 mb-2 bg-red-500 text-white text-sm rounded px-2 py-1 shadow-md opacity-0 group-hover:opacity-100 transition-opacity",
                            "Upload Plans"
                        }
                    }
                }
            }

            //RIGHT DIV SECTION
            div { class: "px-4 flex items-center text-[13px] font-extralight font-lexend",
                "Acme Architects"
            }
        }
        Outlet::<Route> {}
    }
}
