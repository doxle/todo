use dioxus::prelude::*;

/// Canvas Page With Sidebar
#[component]
pub fn Canvas() -> Element {
    rsx! {
        // PARENT DIV - FULL WIDTH
        div {
            class:"m-0 p-0 w-full h-screen bg-[rgb(245,245,245)] flex items-center justify-center ",
            // 75% WIDTH DIV
            div{
                class:"m-0 p-0 w-4/5 h-screen flex items-center justify-center bg-grid-pattern",
                h1{
                    class:"text-black",
                    "Canvas"
                }
            }
            // 25% WIDTH DIV
            div{
                class:"m-0 p-0 w-1/5 h-screen flex items-center justify-center bg-white",

            }
        }

    }
}
