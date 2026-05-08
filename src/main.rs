use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/drag-drop-touch@1.3.1/DragDropTouch.min.js?autoload",
            type: "module"
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            // img { src: HEADER_SVG, id: "header", draggable: true }
            // div { id: "links",
            //     a { href: "https://dioxuslabs.com/learn/0.7/", draggable: true, "📚 Learn Dioxus" }
            //     a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            //     a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            //     a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            //     a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            //     a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            // }

            div {
                margin: "2rem",
                display: "flex",
                flex_direction: "row",
                for i in 0..8 {
                    if i > 0 {
                        div {
                            width: "1rem",
                        }
                    }
                    div {
                        style: "place-items: center",
                        background_color: "#fff",
                        width: "9.5rem",
                        height: "12rem",
                        border: "0.5rem solid #000",
                        border_radius: "1.5rem",
                        display: "grid",
                        grid_template_columns: "auto auto",
                        font_size: "4rem",
                        text_align: "center",
                        padding: "0.5rem",

                        div {
                            font_family: "KaTeX_Main",
                            text_align: "center",
                            "13"
                        },
                        div {
                            font_family: "KaTeX_Main",
                            text_align: "center",
                            "♠"
                        },
                        div {
                            font_family: "KaTeX_Main",
                            "♠"
                        },
                        div {
                            font_family: "KaTeX_Main",
                            "13"
                        },
                    }
                }
                
            }
            
        }
    }
}
