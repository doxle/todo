use dioxus::prelude::*;

/// Upload Page
#[component]
pub fn Upload() -> Element {
    rsx! {
        div{
            // style: "height: 100vh; width: 100%; background-color: blue; margin: 0; padding: 0;",
            // class:"m-0 p-0 w-full h-screen bg-[rgb(30,30,33)]",
            class:"m-0 p-0 w-full h-screen bg-[rgb(245,245,245)] flex items-center justify-center",
            h1{
                class:"text-black",
                "Upload"
            }
        }

    }
}
