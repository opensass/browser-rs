use browser_rs::yew::BrowserFrame;
use browser_rs::{Size, Variant};
use theme::yew::use_theme;
use theme::Theme;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme_ctx = use_theme();

    let onclick = {
        let theme_ctx = theme_ctx.clone();
        Callback::from(move |_| {
            let new_theme = match *theme_ctx.theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
                _ => Theme::Light,
            };
            theme_ctx.set_theme.emit(new_theme);
        })
    };

    html! {
            <button
                {onclick}
                class="relative w-[50px] h-[26px] rounded-full bg-gray-300 dark:bg-gray-800 p-1 flex items-center justify-between transition-colors duration-300"
            >
                <span
                    class="absolute top-[2px] left-[2px] w-[22px] h-[22px] rounded-full bg-white transition-transform duration-300 transform translate-x-0 dark:translate-x-[24px]"
                />
                <span class="absolute inset-0 flex items-center justify-between px-2 text-xs z-0">
                    <i
                        class="fas fa-moon text-yellow-400 dark:opacity-100 opacity-0 transition-opacity duration-300"
                    />
                    <i
                        class="fas fa-sun text-yellow-600 dark:opacity-0 opacity-100 transition-opacity duration-300"
                    />
                </span>
            </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ControlsProps {
    pub url: UseStateHandle<String>,
    pub show_controls: UseStateHandle<bool>,
    pub show_address_bar: UseStateHandle<bool>,
    pub read_only: UseStateHandle<bool>,
}

#[function_component(ControlsPanel)]
pub fn controls_panel(props: &ControlsProps) -> Html {
    let on_url_change = {
        let url = props.url.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                url.set(input.value());
            }
        })
    };

    let on_show_controls_change = {
        let show_controls = props.show_controls.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                show_controls.set(input.checked());
            }
        })
    };

    let on_show_address_bar_change = {
        let show_address_bar = props.show_address_bar.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                show_address_bar.set(input.checked());
            }
        })
    };

    let on_read_only_change = {
        let read_only = props.read_only.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                read_only.set(input.checked());
            }
        })
    };
    html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4 rounded shadow bg-gray-50 mt-8">
            <label class="flex items-center gap-2">
                { "Theme:" }
                <ThemeToggle />
            </label>

            <label class="flex flex-col gap-1">
                <span class="text-sm font-semibold text-gray-700">{ "URL:" }</span>
                <input
                    type="text"
                    value={(*props.url).clone()}
                    oninput={on_url_change}
                    class="px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring focus:border-blue-500 w-full"
                />
            </label>

            <label class="flex items-center gap-2">
                <input
                    type="checkbox"
                    checked={*props.show_controls}
                    onchange={on_show_controls_change}
                    class="accent-blue-600"
                />
                { "Show Controls" }
            </label>

            <label class="flex items-center gap-2">
                <input
                    type="checkbox"
                    checked={*props.show_address_bar}
                    onchange={on_show_address_bar_change}
                    class="accent-blue-600"
                />
                { "Show Address Bar" }
            </label>

            <label class="flex items-center gap-2">
                <input
                    type="checkbox"
                    checked={*props.read_only}
                    onchange={on_read_only_change}
                    class="accent-blue-600"
                />
                { "Read Only" }
            </label>
        </div>
    }
}


#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let url = use_state(|| "https://opensass.org".to_string());
    let show_controls = use_state(|| true);
    let show_address_bar = use_state(|| true);
    let read_only = use_state(|| false);

    let on_url_change = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                url.set(input.value());
            }
        })
    };

    html! {
        <section class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Browser RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "BrowserFrame Sizes" }
                    </h2>
                    <pre class="w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto">
{ r#"use yew::prelude::*;
use web_sys::HtmlInputElement;
use browser_rs::yew::BrowserFrame;
use browser_rs::{Size, Variant};


#[function_component(Example)]
pub fn example() -> Html {
    let url = use_state(|| "https://opensass.org".to_string());
    let on_url_change = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                url.set(input.value());
            }
        })
    };

    html! {
        <BrowserFrame
            url={(*url).clone()}
            size={Size::Small}
            show_controls={true}
            show_address_bar={true}
            read_only={false}
            variant={Variant::Ios}
            on_url_change={Some(on_url_change)}
        />
    }
}"# }
                    </pre>
                    <BrowserFrame
                        url={(*url).clone()}
                        size={Size::Small}
                        show_controls={*show_controls}
                        show_address_bar={*show_address_bar}
                        read_only={*read_only}
                        variant={Variant::Ios}
                        on_url_change={Some(on_url_change.clone())}
                    />
                </div>

                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Medium BrowserFrame With Content" }
                    </h2>
                    <pre class="w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto">
{ r#"use yew::prelude::*;
use web_sys::HtmlInputElement;
use browser_rs::yew::BrowserFrame;
use browser_rs::{Size, Variant};


#[function_component(Example)]
pub fn example() -> Html {
    let url = use_state(|| "https://opensass.org".to_string());
    let on_url_change = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                url.set(input.value());
            }
        })
    };

    html! {
        <BrowserFrame
            url={(*url).clone()}
            size={Size::Medium}
            show_controls={true}
            show_address_bar={true}
            read_only={false}
            variant={Variant::Ios}
            on_url_change={Some(on_url_change)}
        >
            <div class="space-y-4 p-6">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                    { "Custom Content" }
                </h3>
                <p class="text-gray-700 dark:text-gray-300">
                    { "This BrowserFrame includes custom children content." }
                </p>
            </div>
        </BrowserFrame>
    }
}"# }
                    </pre>
                    <BrowserFrame
                        url={(*url).clone()}
                        size={Size::Medium}
                        show_controls={*show_controls}
                        show_address_bar={*show_address_bar}
                        read_only={*read_only}
                        variant={Variant::Ios}
                        on_url_change={Some(on_url_change.clone())}
                    >
                        <div class="space-y-4 p-6">
                            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                                { "Custom Content" }
                            </h3>
                            <p class="text-gray-700 dark:text-gray-300">
                                { "This BrowserFrame includes custom children content." }
                            </p>
                        </div>
                    </BrowserFrame>
                </div>

                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Full BrowserFrame with Content" }
                    </h2>
                    <pre class="w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto">
{ r#"use yew::prelude::*;
use web_sys::HtmlInputElement;
use browser_rs::yew::BrowserFrame;
use browser_rs::{Size, Variant};


#[function_component(Example)]
pub fn example() -> Html {
    let url = use_state(|| "https://opensass.org".to_string());

    html! {
        <BrowserFrame
            url={(*url).clone()}
            size={Size::Full}
            show_controls={true}
            show_address_bar={true}
            read_only={false}
            variant={Variant::Tabs}
            on_close={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Close clicked").unwrap())}
            on_minimize={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Minimize clicked").unwrap())}
            on_maximize={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Maximize clicked").unwrap())}
        >
            <iframe
                class="w-full min-h-full"
                src="https://opensass.org"
                scrolling="auto"
                style="border: none; width: 100%; height: 50vh; overflow: hidden;"
                frameborder="0"
            />
        </BrowserFrame>
    }
}"# }
                    </pre>
                    <BrowserFrame
                        url={(*url).clone()}
                        size={Size::Full}
                        show_controls={*show_controls}
                        show_address_bar={*show_address_bar}
                        read_only={*read_only}
                        variant={Variant::Tabs}
                        on_close={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Close clicked").unwrap())}
                        on_minimize={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Minimize clicked").unwrap())}
                        on_maximize={Callback::from(|_| web_sys::window().unwrap().alert_with_message("Maximize clicked").unwrap())}
                    >
                        <iframe
                            class="w-full min-h-full"
                            src="https://opensass.org"
                            scrolling="auto"
                            style="border: none; width: 100%; height: 50vh; overflow: hidden;"
                            frameborder="0"
                        />
                    </BrowserFrame>
                </div>
            </div>

            <ControlsPanel
                url={url.clone()}
                show_controls={show_controls.clone()}
                show_address_bar={show_address_bar.clone()}
                read_only={read_only.clone()}
            />
        </section>
    }
}
