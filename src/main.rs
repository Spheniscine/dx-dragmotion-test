use dioxus::prelude::*;
use dioxus_motion::{AnimationManager, prelude::{AnimationConfig, AnimationMode, Tween}, use_motion};

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
            href: "https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Noto+Sans+Symbols+2&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style {
            r#"
            @font-face {{
                font-family: KaTeX_Main;
                font-style: normal;
                font-weight: 700;
                src: url({asset!("assets/KaTeX_Suits.woff2")}) format("woff2");
            }}
            "#,
        }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let suits = ["♦︎", "♣", "♥", "♠"];
    let suits_alt = ["⬥", "▲", "●", "★"];
    let suits_animals = ["🦁", "🐰", "🦊", "🐧"];
    let colors = ["#d60", "#050", "#f00", "#00c",];

    // let mut anim_x = use_motion(3f32);
    // use_effect(move || {
    //     anim_x.animate_to(70., AnimationConfig::new(AnimationMode::Tween(Tween::default())));
    // });

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
                transform: "scale(0.875)",
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
                        width: "11rem",
                        height: "13rem",
                        border: "0.25rem solid #000",
                        border_radius: "1.5rem",
                        display: "grid",
                        grid_template_columns: "50% 50%",
                        font_size: "5rem",
                        text_align: "center",
                        padding: "0.5rem",
                        color: colors[i % 4],

                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                        div {
                            font_family: "KaTeX_Main",
                            line_height: "1",
                            "{suits[i % 4]}",
                        },
                        div {
                            font_family: "KaTeX_Main",
                            line_height: "1",
                            "{suits[i % 4]}",
                        },
                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                    }
                }
                
            }

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
                        border: "0.25rem solid #000",
                        border_radius: "1.5rem",
                        display: "grid",
                        grid_template_columns: "50% 50%",
                        font_size: "5rem",
                        text_align: "center",
                        padding: "0.5rem",
                        color: colors[i % 4],

                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                        div {
                            font_family: "'Noto Sans Symbols 2'",
                            position: "relative",
                            top: "0.1em",
                            line_height: "1",
                            "{suits_alt[i % 4]}",
                        },
                        div {
                            font_family: "'Noto Sans Symbols 2'",
                            position: "relative",
                            top: "0.1em",
                            line_height: "1",
                            "{suits_alt[i % 4]}",
                        },
                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                    }
                }
            }

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
                        border: "0.25rem solid #000",
                        border_radius: "1.5rem",
                        display: "grid",
                        grid_template_columns: "50% 50%",
                        font_size: "5rem",
                        text_align: "center",
                        padding: "0.5rem",
                        color: colors[i % 4],

                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                        div {
                            font_family: "'Noto Color Emoji'",
                            line_height: "1",
                            "{suits_animals[i % 4]}",
                        },
                        div {
                            font_family: "'Noto Color Emoji'",
                            line_height: "1",
                            "{suits_animals[i % 4]}",
                        },
                        div {
                            font_family: "KaTeX_Main",
                            "10"
                        },
                    }
                }
            }

            div {
                margin: "2rem",
                display: "flex",
                flex_direction: "row",
                div {
                    style: "place-items: center",
                    width: "11rem",
                    height: "13.2rem",
                    border: "0.25rem solid #000",
                    border_radius: "1.5rem",
                    font_size: "5rem",
                    text_align: "center",
                    padding: "0.5rem",
                    class: "card-pattern-1",
                }
            }

            div {
                style: "place-items: center; --tx: -30rem; --ty: 10rem;",
                animation: "0.2s movement",
                background_color: "#fff",
                position: "absolute",
                top: "75rem",
                left: "44rem",
                width: "11rem",
                height: "13.2rem",
                border: "0.25rem solid #000",
                border_radius: "1.5rem",
                display: "grid",
                grid_template_columns: "50% 50%",
                font_size: "5rem",
                text_align: "center",
                padding: "0.5rem",
                color: colors[3],

                div {
                    font_family: "KaTeX_Main",
                    "A"
                },
                div {
                    font_family: "KaTeX_Main",
                    line_height: "1",
                    "{suits[3]}",
                },
                div {
                    font_family: "KaTeX_Main",
                    line_height: "1",
                    "{suits[3]}",
                },
                div {
                    font_family: "KaTeX_Main",
                    "A"
                },
            }
            
        }
    }
}
