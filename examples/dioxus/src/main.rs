use browser_rs::dioxus::BrowserFrame;
use browser_rs::{Size, Variant};
use dioxus::prelude::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use theme::dioxus::{use_theme, ThemeProvider};
use theme::Theme;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
// const MAIN_CSS: Asset = asset!("/assets/styles.css");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Script { src: "https://kit.fontawesome.com/8f223ead6e.js" },
            ThemeProvider {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
                Examples {}
            }
        }
    }
}

#[component]
fn ThemeToggle() -> Element {
    let theme_ctx = use_theme();

    let onclick = {
        move |_| {
            let new_theme = match (theme_ctx.theme)() {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
                _ => Theme::Light,
            };
            theme_ctx.set_theme.call(new_theme);
        }
    };

    rsx! {
        div { class: "flex items-center justify-center",
            button {
                onclick: onclick,
                class: "relative w-[50px] h-[26px] rounded-full bg-gray-300 dark:bg-gray-800 p-1 flex items-center justify-between transition-colors duration-300",
                span {
                    class: "absolute top-[2px] left-[2px] w-[22px] h-[22px] rounded-full bg-white transition-transform duration-300 transform translate-x-0 dark:translate-x-[24px]"
                }
                span {
                    class: "absolute inset-0 flex items-center justify-between px-2 text-xs z-0",
                    i {
                        class: "fas fa-moon text-yellow-400 dark:opacity-100 opacity-0 transition-opacity duration-300"
                    }
                    i {
                        class: "fas fa-sun text-yellow-600 dark:opacity-0 opacity-100 transition-opacity duration-300"
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
struct ControlsProps {
    url: Signal<String>,
    show_controls: Signal<bool>,
    show_address_bar: Signal<bool>,
    read_only: Signal<bool>,
}

#[component]
fn ControlsPanel(mut props: ControlsProps) -> Element {
    let on_url_change = move |e: Event<FormData>| {
        props.url.set(e.value());
    };

    let on_show_controls_change = move |e: Event<FormData>| {
        props.show_controls.set(!(props.show_controls)());
    };

    let on_show_address_bar_change = move |e: Event<FormData>| {
        props.show_address_bar.set(!(props.show_controls)());
    };

    let on_read_only_change = move |e: Event<FormData>| {
        props.read_only.set(!(props.show_controls)());
    };

    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4 rounded shadow bg-gray-50 mt-8",
            label {
                class: "flex items-center gap-2",
                "Theme:"
                ThemeToggle {}
            }
            label {
                class: "flex flex-col gap-1",
                span {
                    class: "text-sm font-semibold text-gray-700",
                    "URL:"
                }
                input {
                    r#type: "text",
                    value: "{props.url}",
                    oninput: on_url_change,
                    class: "px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring focus:border-blue-500 w-full"
                }
            }
            label {
                class: "flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    checked: "{props.show_controls}",
                    onchange: on_show_controls_change,
                    class: "accent-blue-600"
                }
                "Show Controls"
            }
            label {
                class: "flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    checked: "{props.show_address_bar}",
                    onchange: on_show_address_bar_change,
                    class: "accent-blue-600"
                }
                "Show Address Bar"
            }
            label {
                class: "flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    checked: "{props.read_only}",
                    onchange: on_read_only_change,
                    class: "accent-blue-600"
                }
                "Read Only"
            }
        }
    }
}

#[component]
fn Examples() -> Element {
    let mut url = use_signal(|| "https://opensass.org".to_string());
    let show_controls = use_signal(|| true);
    let show_address_bar = use_signal(|| true);
    let read_only = use_signal(|| false);

    let on_url_change = move |e: Event<FormData>| {
        url.set(e.value());
    };

    rsx! {
        section {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 {
                class: "text-3xl font-bold mb-8 text-white",
                "Browser RS Dioxus Examples"
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 {
                        class: "text-xl font-semibold mb-4 text-gray-800",
                        "Small BrowserFrame"
                    }
                    pre {
                        class: "w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto",
                        r##"use dioxus::prelude::*;
use browser_rs::BrowserFrame;
use browser_rs::{{Size, Variant}};

#[component]
fn Example() -> Element {{
    let url = use_signal(|| "https://opensass.org".to_string());
    let on_url_change = move |e: Event<FormData>| {{
        if let Some(value) = e.value() {{
            url.set(value);
        }}
    }};

    rsx! {{
        BrowserFrame {{
            url: url(),
            size: Size::Small,
            show_controls: true,
            show_address_bar: true,
            read_only: false,
            variant: Variant::Ios,
            on_url_change: on_url_change,
        }}
    }}
}}"##
                    }
                    BrowserFrame {
                        url: url(),
                        size: Size::Small,
                        show_controls: show_controls(),
                        show_address_bar: show_address_bar(),
                        read_only: read_only(),
                        variant: Variant::Ios,
                        on_url_change: on_url_change.clone(),
                    }
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 {
                        class: "text-xl font-semibold mb-4 text-gray-800",
                        "Medium BrowserFrame With Content"
                    }
                    pre {
                        class: "w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto",
                        r##"use dioxus::prelude::*;
use browser_rs::BrowserFrame;
use browser_rs::{{Size, Variant}};

#[component]
fn Example() -> Element {{
    let url = use_signal(|| "https://opensass.org".to_string());
    let on_url_change = move |e: Event<FormData>| {{
        if let Some(value) = e.value() {{
            url.set(value);
        }}
    }};

    rsx! {{
        BrowserFrame {{
            url: url(),
            size: Size::Medium,
            show_controls: true,
            show_address_bar: true,
            read_only: false,
            variant: Variant::Ios,
            on_url_change: on_url_change,
            div {{
                class: "space-y-4 p-6",
                h3 {{
                    class: "text-lg font-semibold text-gray-900 dark:text-white",
                    "Custom Content"
                }}
                p {{
                    class: "text-gray-700 dark:text-gray-300",
                    "This BrowserFrame includes custom children content."
                }}
            }}
        }}
    }}
}}"##
                    }
                    BrowserFrame {
                        url: url(),
                        size: Size::Medium,
                        show_controls: show_controls(),
                        show_address_bar: show_address_bar(),
                        read_only: read_only(),
                        variant: Variant::Ios,
                        on_url_change: on_url_change.clone(),
                        div {
                            class: "space-y-4 p-6",
                            h3 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white",
                                "Custom Content"
                            }
                            p {
                                class: "text-gray-700 dark:text-gray-300",
                                "This BrowserFrame includes custom children content."
                            }
                        }
                    }
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 {
                        class: "text-xl font-semibold mb-4 text-gray-800",
                        "Full BrowserFrame with Content"
                    }
                    pre {
                        class: "w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto",
                        r##"use dioxus::prelude::*;
use browser_rs::BrowserFrame;
use browser_rs::{{Size, Variant}};

#[component]
fn Example() -> Element {{
    let url = use_signal(|| "https://opensass.org".to_string());

    rsx! {{
        BrowserFrame {{
            url: url(),
            size: Size::Full,
            show_controls: true,
            show_address_bar: true,
            read_only: false,
            variant: Variant::Tabs,
            on_close: move |_| {{
                web_sys::window().unwrap().alert_with_message("Close clicked").unwrap();
            }},
            on_minimize: move |_| {{
                web_sys::window().unwrap().alert_with_message("Minimize clicked").unwrap();
            }},
            on_maximize: move |_| {{
                web_sys::window().unwrap().alert_with_message("Maximize clicked").unwrap();
            }},
            iframe {{
                class: "w-full h-[80vh]",
                src: "{url()}",
                allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture",
                allowfullscreen: true
            }}
        }}
    }}
}}"##
                    }
                    BrowserFrame {
                        url: url(),
                        size: Size::Full,
                        show_controls: show_controls(),
                        show_address_bar: show_address_bar(),
                        read_only: read_only(),
                        variant: Variant::Tabs,
                        on_close: move |_| {
                            web_sys::window().unwrap().alert_with_message("Close clicked").unwrap();
                        },
                        on_minimize: move |_| {
                            web_sys::window().unwrap().alert_with_message("Minimize clicked").unwrap();
                        },
                        on_maximize: move |_| {
                            web_sys::window().unwrap().alert_with_message("Maximize clicked").unwrap();
                        },
                        iframe {
                            class: "w-full h-[80vh]",
                            src: "{url()}",
                            allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture",
                            allowfullscreen: true
                        }
                    }
                }
            }

            ControlsPanel {
                url: url,
                show_controls: show_controls,
                show_address_bar: show_address_bar,
                read_only: read_only
            }
        }
    }
}
