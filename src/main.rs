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
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Symbols+2&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
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
                for i in 0..7 {
                    if i > 0 {
                        div {
                            width: "1rem",
                        }
                    }
                    div {
                        style: "place-items: center",
                        background_color: "#fff",
                        width: "11rem",
                        height: "13.2rem",
                        border: "0.5rem solid #000",
                        border_radius: "1.5rem",
                        display: "grid",
                        grid_template_columns: "auto auto",
                        font_size: "4.9rem",
                        text_align: "center",
                        padding: "0.5rem",
                        color: "#f70",

                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                        div {
                            font_family: "'Times New Roman'",
                            font_size: "110%",
                            "♦︎"
                        },
                        div {
                            font_family: "'Times New Roman'",
                            font_size: "110%",
                            "♦︎"
                        },
                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                    }
                }
                
            }
            
        }
    }
}
