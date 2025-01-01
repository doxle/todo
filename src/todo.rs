use crate::api::{
    add_todo, delete_todo, generate_time_based_id, get_all_todos, TodoItem, TodoType,
};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::collections::BTreeMap;

// USING BTREEMAP IN LIEU OF HASMAP FOR SORTING BY DEFAULT BUT MIGHT BE A PERFORMANCE BOTTLE NECK
static TODO: GlobalSignal<BTreeMap<u32, TodoItem>> = Global::new(|| BTreeMap::new());

//- CHECKLIST PAGE
#[component]
pub fn Todo() -> Element {
    // AWAITS THE FUTURE HERE, MAY INTIALLY BE NONE AND THEN FETCH THE DATA
    let todo_future = use_resource(move || async move { get_all_todos().await });

    match &*todo_future.read_unchecked() {
        // IF THERE IS NO ERROR WE START WRITING INTO THE GLOBAL SIGNAL SO IT CAN SHOWN IN THE TODOLIST COMPONENT
        Some(Ok(res)) => {
            //DEREFERENCE AND WRITE TO THE GLOBAL SIGNAL, WE NEED TO CONVERT VEC INTO BTREEMAP WITH KEY AS THE ID.
            *TODO.write() = res
                .into_iter()
                .map(|todo| (todo.id, todo.clone()))
                .collect();
            rsx! {
                div {
                    class: "m-0 p-0  w-full h-screen bg-[rgb(245,245,245)] flex flex-col items-center justify-start  ",
                    TodoHeader {}
                    TodoList {}
                }
            }
        }
        Some(Err(e)) => rsx! {
            div {
                class:"m-0 p-0 font-extralight font-lexend text-[18px] w-full h-screen bg-[rgb(245,245,245)] flex flex-col items-center justify-center text-blue-500",
                "Failed retrieving todos... {e}"
            }
        },
        None => rsx! {

            div {
                class:"m-0 p-0 font-extralight font-lexend text-[18px] w-full h-screen bg-[rgb(245,245,245)] flex flex-col items-center justify-center text-blue-500",
                "Loading todo's..."
            }
        },
    }
}

// HEADER ROW WHERE THE CHECKLIST ITEMS ARE INSERTED
#[component]
fn TodoHeader() -> Element {
    let mut input_value = use_signal(|| String::from(""));
    // let add_resource = use_resource(|| async { Ok::<u32, String>(0) });

    //-AFTER ENTR KEY IS PRESSED, CREATE A NEW CHECKLIST ITEM
    let onkeydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            tracing::info!("Enter has been pressed {:?}", &input_value.read());
            // ADD A NEW CHECKLIST ITEM
            let todo_id = generate_time_based_id();
            let newTodo: TodoItem = TodoItem {
                id: todo_id,
                checked: false,
                // todo_type: TodoType::Active,
                content: input_value().to_string(),
            };

            // -2 WRITE ALSO TO THE DDB, MAKE THE API CALL
            spawn(async move {
                match add_todo(newTodo.clone()).await {
                    Ok(()) => {
                        tracing::info!("Successfully addded : {}", &todo_id);
                        // -1 WRITE TO THE SIGNAL
                        TODO.write().insert(todo_id, newTodo);
                    }
                    Err(e) => {
                        tracing::error!("Failed to write todo to backend: {:?}", e);
                        // ROLLBACK OUR FAILURE AND REMOVE IT FROM THE SIGNAL
                        TODO.write().remove(&todo_id);
                    }
                }
            });
            //CLEAR THE TEXT INPUT
            input_value.set("".to_string());
        }
    };
    rsx! {

        div{
            class:"mt-[10px] font-lexend font-thin text-[100px] text-[rgb(169,202,248)]",
            // "notes"
            "todos"
        }
        //-HEADER INPUT - TO ADD ITEMS
        div { class: " mt-[0px] bg-blue-400  min-w-[300px] w-full max-w-[640px] h-[72px] md:h-[55px] flex",
            input {

                class: "placeholder-italic text-normal appearance-none focus:shadow-none focus:outline-none focus:ring-0 focus:border-blue-100
                font-lexend font-extralight tracking-wider px-4  flex flex-1 border-b
                border-b-[rgb(230,230,230)] text-black text-[24px] md:text-[16px] placeholder-gray-300 ",
                placeholder: "What needs to be done?",
                autofocus: true,
                value: "{input_value}",
                oninput: move |evt| input_value.set(evt.value()),
                onkeydown,
            }
        }
    }
}

// DISPLAYS THE ENTIRE TODO LIST IN A FOR LOOP
#[component]
fn TodoList() -> Element {
    let mut checked_signal = use_signal(|| false);

    rsx! {
        div { class: " min-w-[300px] w-full max-w-[640px] min-h-[72px] md:min-h-[63px] flex flex-col",
            ul { class: " flex flex-col flex-1",
                for (id, item) in TODO.read().iter() {

                    li { class: "min-w-[300px] w-full max-w-[640px]  font-extralight tracking-wider text-[15px] bg-white text-black flex items-center px-4 cursor-pointer hover:bg-blue-50 border-b border-b-[rgb(230,230,230)]",

                        // ENTIRE ROW  OR CHECKLIST ITEM
                        div { class: "min-h-[72px] md:min-h-[63px] w-full h-full flex items-center group py-3 ",
                             //- CHECKBOX WITH CUSTOM CHECK ICON
                            div {
                                class:" relative w-6 h-6 cursor-pointer",
                                onclick:
                                {
                                     let todo_id = item.id.clone();

                                    move |_| {


                                        tracing::info!("checkbox before toggling{:?}", checked_signal());
                                        checked_signal.set(!checked_signal());
                                        tracing::info!("checkbox after toggling{:?}", checked_signal());

                                        // if let Some(todo_item) = TODO.write().get_mut(id.as_str()) {
                                        //         todo_item.checked = !todo_item.checked;
                                        //     }
                                        TODO.write().entry(todo_id).and_modify(|item| {
                                            item.checked = !item.checked;
                                        });
                                        tracing::info!("checkbox after toggling{:?}", TODO);

                                    }
                                },


                                input {
                                    class: "cursor-pointer w-6 h-6 border border-gray-300 rounded-xl
                                            checked:bg-blue-400 checked:border-blue-500 flex-shrink-0 peer appearance-none ",
                                    r#type: "checkbox",
                                    checked: "{item.checked}",

                                }
                                img {
                                    class:"absolute inset-0 w-1/2 h-1/2 m-auto hidden peer-checked:block",
                                    src:"/assets/check-white.svg",
                                    alt:"checked"
                                }
                            },
                            // ITEM CONTENTS DISPLAYED HERE
                            span {
                                class: "font-lexend text-[24px] md:text-[16px] ml-2 flex-1  peer-checked:line-through peer-checked:bg-gray-400",
                                class: if item.checked {"line-through text-gray-500"},
                                "{item.content}" }

                            // DELETE BUTTON APPEAR ONLY ON HOVER
                            button {

                                    onclick: {
                                    let id = item.id.clone();
                                     move |evt:Event<MouseData>| {
                                         evt.prevent_default();
                                        tracing::info!("delete button is clicked  for id {:?}", &id);
                                        //- REMOVING THE CHECKLIST ITEM

                                        // REMOVE FROM DB, MAKE API CALL
                                        spawn(async move {
                                            match delete_todo(id).await {
                                                Ok(()) => {
                                                    tracing::info!("Successfully addded : {}", &id);
                                                      TODO.write().remove(&id);

                                                }
                                                Err(e) => {
                                                    tracing::error!("Failed to write todo to backend: {:?}", e);

                                                }
                                            }
                                        });
                                    }

                                    },
                                    class: "ml-auto px-2 py-1 rounded-lg hover:bg-blue-100",
                                    img {
                                        class:"opacity-0 group-hover:opacity-100 transition-opacity duration-200",
                                        src:"/assets/trash-black.svg",
                                        alt:"delete"
                                    },

                            }
                        }


                    }
                }
            }
        }
    }
}

// FOOTER CONTENTS THAT DISPLAYS THE TYPES ETC
#[component]
fn footer() -> Element {
    rsx! {
        footer {
            p{
                class:"",
                "footer"
            }
        }
    }
}
