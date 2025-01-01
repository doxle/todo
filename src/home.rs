use dioxus::prelude::*;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            // style: "height: 100vh; width: 100%; background-color: blue; margin: 0; padding: 0;",
            // class:"m-0 p-0 w-full h-screen bg-[rgb(30,30,33)]",
            class:"m-0 p-0 w-full h-screen bg-[rgb(245,245,245)] flex flex-col justify-center items-center",
            // h1{
            //     class:"text-black font-light",
            //     "Welcome to Doxle"
            // }
            h1{
                class:"text-black font-lexend font-extralight tracking-wide",
                "Welcome to Doxle"
            }
        }

    }
}
