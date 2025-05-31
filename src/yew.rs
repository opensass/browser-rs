#![doc = include_str!("../YEW.md")]

use crate::common::{ButtonType, Size, Variant};
use gloo_timers::callback::Timeout;
use web_sys::{
    Element, HtmlInputElement, KeyboardEvent,
    wasm_bindgen::{JsCast, prelude::*},
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AddressBarProps {
    #[prop_or_default]
    pub url: String,
    #[prop_or("Enter URL or search...")]
    pub placeholder: &'static str,
    #[prop_or_default]
    pub on_url_change: Callback<InputEvent>,
    #[prop_or(false)]
    pub read_only: bool,

    #[prop_or_default]
    pub class: &'static str,
    #[prop_or_default]
    pub style: &'static str,

    #[prop_or("Website address or search query")]
    pub label: &'static str,
    #[prop_or("Enter a website URL or search term. Press Enter to navigate.")]
    pub describedby: &'static str,
    #[prop_or("browser-url-input")]
    pub input_id: &'static str,

    #[prop_or("w-full text-gray-900 dark:text-white bg-white dark:bg-gray-700 pr-8")]
    pub input_class: &'static str,

    #[prop_or(
        "flex-1 mx-4 border border-gray-300 dark:border-gray-500 rounded-md px-3 py-1 bg-transparent text-sm relative bg-white dark:bg-gray-700"
    )]
    pub container_class: &'static str,

    #[prop_or(
        "position: absolute; top: 50%; right: 8px; transform: translateY(-50%); padding: 4px; border: none; cursor: pointer; color: #d1d5db;"
    )]
    pub refresh_button_style: &'static str,
    #[prop_or("Refresh")]
    pub refresh_button_aria_label: &'static str,
}

#[function_component(AddressBar)]
pub fn address_bar(props: &AddressBarProps) -> Html {
    let input_value = use_state(|| props.url.to_string());
    let is_focused = use_state(|| false);
    let input_ref = use_node_ref();

    {
        let input_value = input_value.clone();
        use_effect_with(props.url.clone(), move |url| {
            input_value.set(url.clone());
        });
    }

    let on_input_change = {
        let input_value = input_value.clone();
        let on_url_change = props.on_url_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let value = input.value();
                input_value.set(value.clone());
                on_url_change.emit(e);
            }
        })
    };

    let on_key_down = {
        let input_ref = input_ref.clone();
        let value = (*input_value).clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.blur().ok();
                }

                let document = web_sys::window().unwrap().document().unwrap();
                let live_region = document.create_element("div").unwrap();
                live_region.set_attribute("aria-live", "polite").unwrap();
                live_region.set_attribute("aria-atomic", "true").unwrap();
                live_region.set_class_name("sr-only");
                live_region.set_text_content(Some(&format!("Navigating to {}", value)));
                document.body().unwrap().append_child(&live_region).unwrap();

                let live_region_clone = live_region.clone();
                Timeout::new(1000, move || {
                    let _ = document.body().unwrap().remove_child(&live_region_clone);
                })
                .forget();
            }
        })
    };

    let on_focus = {
        let is_focused = is_focused.clone();
        Callback::from(move |_| {
            is_focused.set(true);
        })
    };

    let on_blur = {
        let is_focused = is_focused.clone();
        Callback::from(move |_| {
            is_focused.set(false);
        })
    };

    html! {
        <div class={format!("{} {}", props.container_class, props.class)} style={props.style}>
            <label for={props.input_id} class="sr-only">{ props.label }</label>
            <input
                ref={input_ref.clone()}
                id={props.input_id}
                type="text"
                value={(*input_value).clone()}
                oninput={on_input_change}
                onkeydown={on_key_down}
                onfocus={on_focus}
                onblur={on_blur}
                placeholder={props.placeholder}
                readonly={props.read_only}
                class={props.input_class}
                aria-describedby={props.describedby}
                autocomplete="url"
                spellcheck={Some("false")}
            />
            <button
                style={props.refresh_button_style}
                aria-label={props.refresh_button_aria_label}
                onclick={Callback::from(|_| {
                    let _ = web_sys::window().unwrap().location().reload();
                })}
            >
                <svg
                    width="11"
                    height="13"
                    viewBox="0 0 11 13"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M4.99385 1.00002L7.33006 3.33623L4.99385 5.67244M10 7.61925C10 10.1998 7.9081 12.2917 5.3276 12.2917C2.74709 12.2917 0.655182 10.1998 0.655182 7.61925C0.655182 5.03875 2.74709 2.94684 5.3276 2.94684C5.8737 2.94684 6.4957 2.94684 7.27443 3.33621"
                        stroke="#767676"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrowserContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: &'static str,
    #[prop_or_default]
    pub style: &'static str,
    #[prop_or("Browser content area")]
    pub aria_label: &'static str,
    #[prop_or_default]
    pub aria_describedby: &'static str,
}

#[function_component(BrowserContent)]
pub fn browser_content(props: &BrowserContentProps) -> Html {
    html! {
        <main
            class={props.class}
            style={props.style}
            role="main"
            aria-label={props.aria_label}
            aria-describedby={props.aria_describedby}
            tabindex={Some("-1")}
        >
            { for props.children.iter() }
        </main>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ControlButtonProps {
    pub r#type: ButtonType,

    #[prop_or_default]
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub on_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,

    #[prop_or(
        "w-5 h-5 flex items-center justify-center rounded-full transition-all duration-200 cursor-pointer"
    )]
    pub base_class: &'static str,
    #[prop_or_default]
    pub class: &'static str,
    #[prop_or_default]
    pub svg_class: &'static str,
    #[prop_or_default]
    pub path_class: &'static str,

    #[prop_or("button")]
    pub button_type: &'static str,
    #[prop_or_default]
    pub aria_label: &'static str,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or("0")]
    pub tabindex: &'static str,
}

#[function_component(ControlButton)]
pub fn control_button(props: &ControlButtonProps) -> Html {
    let ControlButtonProps {
        r#type,
        on_click,
        on_mouse_over,
        on_mouse_out,
        on_focus,
        on_blur,
        base_class,
        class,
        svg_class,
        path_class,
        button_type,
        aria_label,
        title,
        tabindex,
    } = props.clone();

    let full_class = format!("{} {}", base_class, class);

    let aria_label = if aria_label.is_empty() {
        r#type.default_aria_label()
    } else {
        aria_label
    };

    let title = if title.is_empty() {
        r#type.default_title()
    } else {
        title
    };

    let (fill, stroke) = match r#type {
        ButtonType::Close => ("#FF5F57", "#E14640"),
        ButtonType::Minimize => ("#FFBD2E", "#DFA123"),
        ButtonType::Maximize => ("#28CA42", "#1DAD2C"),
    };
    let onclick = Callback::from(move |_| on_click.emit(()));
    let onmouseover = Callback::from(move |_| on_mouse_over.emit(()));
    let onmouseout = Callback::from(move |_| on_mouse_out.emit(()));

    html! {
        <button
            type={button_type}
            class={full_class}
            onclick={onclick}
            onmouseover={onmouseover}
            onmouseout={onmouseout}
            onfocus={on_focus}
            onblur={on_blur}
            aria-label={aria_label}
            title={title}
            tabindex={tabindex}
        >
            <svg
                class={svg_class}
                width="12"
                height="12"
                viewBox="0 0 12 12"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    class={path_class}
                    d="M6 0.5C9.03757 0.5 11.5 2.96243 11.5 6C11.5 9.03757 9.03757 11.5 6 11.5C2.96243 11.5 0.5 9.03757 0.5 6C0.5 2.96243 2.96243 0.5 6 0.5Z"
                    fill={fill}
                    stroke={stroke}
                />
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrowserControlsProps {
    #[prop_or_default]
    pub show_controls: bool,
    #[prop_or("flex items-center px-3 py-2 bg-gray-200 rounded-t-lg")]
    pub class: &'static str,

    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub on_close_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_close_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_close_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_close_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub close_class: &'static str,
    #[prop_or_default]
    pub close_svg_class: &'static str,
    #[prop_or_default]
    pub close_path_class: &'static str,
    #[prop_or("button")]
    pub close_button_type: &'static str,
    #[prop_or_default]
    pub close_aria_label: &'static str,
    #[prop_or_default]
    pub close_title: &'static str,
    #[prop_or("0")]
    pub close_tabindex: &'static str,

    #[prop_or_default]
    pub on_minimize: Callback<()>,
    #[prop_or_default]
    pub on_minimize_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_minimize_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_minimize_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_minimize_blur: Callback<FocusEvent>,
    #[prop_or("ml-2")]
    pub minimize_class: &'static str,
    #[prop_or_default]
    pub minimize_svg_class: &'static str,
    #[prop_or_default]
    pub minimize_path_class: &'static str,
    #[prop_or("button")]
    pub minimize_button_type: &'static str,
    #[prop_or_default]
    pub minimize_aria_label: &'static str,
    #[prop_or_default]
    pub minimize_title: &'static str,
    #[prop_or("0")]
    pub minimize_tabindex: &'static str,

    #[prop_or_default]
    pub on_maximize: Callback<()>,
    #[prop_or_default]
    pub on_maximize_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_maximize_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_maximize_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_maximize_blur: Callback<FocusEvent>,
    #[prop_or("ml-2")]
    pub maximize_class: &'static str,
    #[prop_or_default]
    pub maximize_svg_class: &'static str,
    #[prop_or_default]
    pub maximize_path_class: &'static str,
    #[prop_or("button")]
    pub maximize_button_type: &'static str,
    #[prop_or_default]
    pub maximize_aria_label: &'static str,
    #[prop_or_default]
    pub maximize_title: &'static str,
    #[prop_or("0")]
    pub maximize_tabindex: &'static str,
}

#[function_component(BrowserControls)]
pub fn browser_controls(props: &BrowserControlsProps) -> Html {
    if !props.show_controls {
        return html! {};
    }

    html! {
        <nav class={props.class} role="toolbar" aria-label="Browser window controls">
            <ControlButton
                r#type={ButtonType::Close}
                on_click={props.on_close.clone()}
                on_mouse_over={props.on_close_mouse_over.clone()}
                on_mouse_out={props.on_close_mouse_out.clone()}
                on_focus={props.on_close_focus.clone()}
                on_blur={props.on_close_blur.clone()}
                class={props.close_class}
                svg_class={props.close_svg_class}
                path_class={props.close_path_class}
                button_type={props.close_button_type}
                aria_label={props.close_aria_label}
                title={props.close_title}
                tabindex={props.close_tabindex}
            />
            <ControlButton
                r#type={ButtonType::Minimize}
                on_click={props.on_minimize.clone()}
                on_mouse_over={props.on_minimize_mouse_over.clone()}
                on_mouse_out={props.on_minimize_mouse_out.clone()}
                on_focus={props.on_minimize_focus.clone()}
                on_blur={props.on_minimize_blur.clone()}
                class={props.minimize_class}
                svg_class={props.minimize_svg_class}
                path_class={props.minimize_path_class}
                button_type={props.minimize_button_type}
                aria_label={props.minimize_aria_label}
                title={props.minimize_title}
                tabindex={props.minimize_tabindex}
            />
            <ControlButton
                r#type={ButtonType::Maximize}
                on_click={props.on_maximize.clone()}
                on_mouse_over={props.on_maximize_mouse_over.clone()}
                on_mouse_out={props.on_maximize_mouse_out.clone()}
                on_focus={props.on_maximize_focus.clone()}
                on_blur={props.on_maximize_blur.clone()}
                class={props.maximize_class}
                svg_class={props.maximize_svg_class}
                path_class={props.maximize_path_class}
                button_type={props.maximize_button_type}
                aria_label={props.maximize_aria_label}
                title={props.maximize_title}
                tabindex={props.maximize_tabindex}
            />
        </nav>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrowserHeaderProps {
    #[prop_or_default]
    pub url: String,
    #[prop_or_default]
    pub placeholder: &'static str,
    #[prop_or_default]
    pub on_url_change: Option<Callback<InputEvent>>,
    #[prop_or(true)]
    pub show_controls: bool,
    #[prop_or(true)]
    pub show_address_bar: bool,
    #[prop_or(false)]
    pub read_only: bool,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub custom_buttons: Vec<Html>,
    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "flex-1 mx-4 border border-gray-300 dark:border-gray-500 rounded-md px-3 py-1 bg-transparent text-sm relative bg-white dark:bg-gray-700"
    )]
    pub container_class: &'static str,
    #[prop_or("w-full text-gray-900 dark:text-white bg-white dark:bg-gray-700 pr-8")]
    pub input_class: &'static str,
    #[prop_or(
        "position: absolute; top: 50%; right: 8px; transform: translateY(-50%); padding: 4px; border: none; cursor: pointer; color: #d1d5db;"
    )]
    pub refresh_button_style: &'static str,
    #[prop_or("Refresh")]
    pub refresh_button_aria_label: &'static str,

    #[prop_or("padding: 4px; border: none; cursor: pointer; color: #d1d5db;")]
    pub icon_button_style: &'static str,

    #[prop_or("flex: 1; display: flex; justify-content: center; padding-right: 8px;")]
    pub address_wrapper_base_style: &'static str,

    #[prop_or("display: flex; align-items: center; position: relative;")]
    pub header_base_style: &'static str,

    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub on_close_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_close_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_close_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_close_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub close_class: &'static str,
    #[prop_or_default]
    pub close_svg_class: &'static str,
    #[prop_or_default]
    pub close_path_class: &'static str,
    #[prop_or("button")]
    pub close_button_type: &'static str,
    #[prop_or_default]
    pub close_aria_label: &'static str,
    #[prop_or_default]
    pub close_title: &'static str,
    #[prop_or("0")]
    pub close_tabindex: &'static str,

    #[prop_or_default]
    pub on_minimize: Callback<()>,
    #[prop_or_default]
    pub on_minimize_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_minimize_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_minimize_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_minimize_blur: Callback<FocusEvent>,
    #[prop_or("ml-2")]
    pub minimize_class: &'static str,
    #[prop_or_default]
    pub minimize_svg_class: &'static str,
    #[prop_or_default]
    pub minimize_path_class: &'static str,
    #[prop_or("button")]
    pub minimize_button_type: &'static str,
    #[prop_or_default]
    pub minimize_aria_label: &'static str,
    #[prop_or_default]
    pub minimize_title: &'static str,
    #[prop_or("0")]
    pub minimize_tabindex: &'static str,

    #[prop_or_default]
    pub on_maximize: Callback<()>,
    #[prop_or_default]
    pub on_maximize_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_maximize_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_maximize_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_maximize_blur: Callback<FocusEvent>,
    #[prop_or("ml-2")]
    pub maximize_class: &'static str,
    #[prop_or_default]
    pub maximize_svg_class: &'static str,
    #[prop_or_default]
    pub maximize_path_class: &'static str,
    #[prop_or("button")]
    pub maximize_button_type: &'static str,
    #[prop_or_default]
    pub maximize_aria_label: &'static str,
    #[prop_or_default]
    pub maximize_title: &'static str,
    #[prop_or("0")]
    pub maximize_tabindex: &'static str,

    #[prop_or_default]
    pub share_button_style: &'static str,
    #[prop_or_default]
    pub share_onclick: Callback<()>,
    #[prop_or_default]
    pub share_onmouseover: Callback<()>,
    #[prop_or_default]
    pub share_onmouseout: Callback<()>,
    #[prop_or_default]
    pub share_onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub share_onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub share_tabindex: &'static str,

    #[prop_or_default]
    pub tabs_button_style: &'static str,
    #[prop_or_default]
    pub tabs_onclick: Callback<()>,
    #[prop_or_default]
    pub tabs_onmouseover: Callback<()>,
    #[prop_or_default]
    pub tabs_onmouseout: Callback<()>,
    #[prop_or_default]
    pub tabs_onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub tabs_onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub tabs_tabindex: &'static str,

    #[prop_or_default]
    pub more_button_style: &'static str,
    #[prop_or_default]
    pub more_onclick: Callback<()>,
    #[prop_or_default]
    pub more_onmouseover: Callback<()>,
    #[prop_or_default]
    pub more_onmouseout: Callback<()>,
    #[prop_or_default]
    pub more_onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub more_onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub more_tabindex: &'static str,
}

#[function_component(BrowserHeader)]
pub fn browser_header(props: &BrowserHeaderProps) -> Html {
    let is_ios = props.variant == Variant::Ios;
    let is_tabs = props.variant == Variant::Tabs;

    let base_style = {
        let padding = match props.size {
            Size::Small => "4px 6px",
            Size::Large => "10px 16px",
            _ => "6px 12px",
        };
        let height = match (props.variant.clone(), props.size.clone()) {
            (Variant::Tabs, _) => "40px",
            (Variant::Ios, _) => "56px",
            (_, Size::Large) => "60px",
            (_, Size::Small) => "38px",
            _ => "48px",
        };
        let border_radius = if is_tabs {
            "6px"
        } else if props.variant == Variant::Default {
            "8px 8px 0 0"
        } else {
            "0"
        };
        let border = if is_tabs { "1px solid #d1d5db" } else { "none" };
        let box_shadow = if props.variant == Variant::Default {
            "0 2px 6px rgba(0,0,0,0.1)"
        } else {
            "none"
        };

        format!(
            "{} justify-content: {}; padding: {}; height: {}; border-radius: {}; border: {}; box-shadow: {};",
            props.header_base_style,
            if is_ios {
                "space-between"
            } else {
                "flex-start"
            },
            padding,
            height,
            border_radius,
            border,
            box_shadow
        )
    };

    let address_wrapper_style = format!(
        "{} padding-left: {};",
        props.address_wrapper_base_style,
        if props.show_controls { "8px" } else { "0" }
    );
    let share_onclick = props.share_onclick.clone();
    let share_onmouseover = props.share_onmouseover.clone();
    let share_onmouseout = props.share_onmouseout.clone();

    let tabs_onclick = props.tabs_onclick.clone();
    let tabs_onmouseover = props.tabs_onmouseover.clone();
    let tabs_onmouseout = props.tabs_onmouseout.clone();

    let more_onclick = props.more_onclick.clone();
    let more_onmouseover = props.more_onmouseover.clone();
    let more_onmouseout = props.more_onmouseout.clone();

    let share_onclick = Callback::from(move |_| share_onclick.emit(()));
    let share_onmouseover = Callback::from(move |_| share_onmouseover.emit(()));
    let share_onmouseout = Callback::from(move |_| share_onmouseout.emit(()));

    let tabs_onclick = Callback::from(move |_| tabs_onclick.emit(()));
    let tabs_onmouseover = Callback::from(move |_| tabs_onmouseover.emit(()));
    let tabs_onmouseout = Callback::from(move |_| tabs_onmouseout.emit(()));

    let more_onclick = Callback::from(move |_| more_onclick.emit(()));
    let more_onmouseover = Callback::from(move |_| more_onmouseover.emit(()));
    let more_onmouseout = Callback::from(move |_| more_onmouseout.emit(()));

    html! {
        <header style={base_style} class={props.class} aria-label="Browser window header">
            <div style="display: flex; align-items: center; gap: 6px;">
                if props.show_controls {
                    <BrowserControls
                        on_close={props.on_close.clone()}
                        on_minimize={props.on_minimize.clone()}
                        on_maximize={props.on_maximize.clone()}
                        show_controls={props.show_controls}
                        on_close={props.on_close.clone()}
                        on_close_mouse_over={props.on_close_mouse_over.clone()}
                        on_close_mouse_out={props.on_close_mouse_out.clone()}
                        on_close_focus={props.on_close_focus.clone()}
                        on_close_blur={props.on_close_blur.clone()}
                        close_class={props.close_class}
                        close_svg_class={props.close_svg_class}
                        close_path_class={props.close_path_class}
                        close_button_type={props.close_button_type}
                        close_aria_label={props.close_aria_label}
                        close_title={props.close_title}
                        close_tabindex={props.close_tabindex}
                        on_minimize={props.on_minimize.clone()}
                        on_minimize_mouse_over={props.on_minimize_mouse_over.clone()}
                        on_minimize_mouse_out={props.on_minimize_mouse_out.clone()}
                        on_minimize_focus={props.on_minimize_focus.clone()}
                        on_minimize_blur={props.on_minimize_blur.clone()}
                        minimize_class={props.minimize_class}
                        minimize_svg_class={props.minimize_svg_class}
                        minimize_path_class={props.minimize_path_class}
                        minimize_button_type={props.minimize_button_type}
                        minimize_aria_label={props.minimize_aria_label}
                        minimize_title={props.minimize_title}
                        minimize_tabindex={props.minimize_tabindex}
                        on_maximize={props.on_maximize.clone()}
                        on_maximize_mouse_over={props.on_maximize_mouse_over.clone()}
                        on_maximize_mouse_out={props.on_maximize_mouse_out.clone()}
                        on_maximize_focus={props.on_maximize_focus.clone()}
                        on_maximize_blur={props.on_maximize_blur.clone()}
                        maximize_class={props.maximize_class}
                        maximize_svg_class={props.maximize_svg_class}
                        maximize_path_class={props.maximize_path_class}
                        maximize_button_type={props.maximize_button_type}
                        maximize_aria_label={props.maximize_aria_label}
                        maximize_title={props.maximize_title}
                        maximize_tabindex={props.maximize_tabindex}
                    />
                }
                if props.show_controls {
                    if !is_ios {
                        <button style={props.icon_button_style} aria-label="Sidebar">
                            <svg
                                width="20"
                                height="15"
                                viewBox="0 0 20 15"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M2.62346 15H16.4609C18.2202 15 19.0844 14.1358 19.0844 12.4074V2.59259C19.0844 0.864204 18.2202 0 16.4609 0H2.62346C0.874483 0 0 0.864204 0 2.59259V12.4074C0 14.1358 0.874483 15 2.62346 15ZM2.64404 13.5082C1.90329 13.5082 1.48149 13.1173 1.48149 12.3354V2.66461C1.48149 1.89301 1.90329 1.49177 2.64404 1.49177H6.22427V13.5082H2.64404ZM16.4403 1.49177C17.1811 1.49177 17.6029 1.89301 17.6029 2.66461V12.3354C17.6029 13.1173 17.1811 13.5082 16.4403 13.5082H7.67489V1.49177H16.4403ZM4.67078 4.47532C4.94857 4.47532 5.18518 4.2284 5.18518 3.9609C5.18518 3.69341 4.94857 3.46708 4.67078 3.46708H3.05556C2.78806 3.46708 2.55144 3.69341 2.55144 3.9609C2.55144 4.2284 2.78806 4.47532 3.05556 4.47532H4.67078ZM4.67078 6.53293C4.94857 6.53293 5.18518 6.29629 5.18518 6.01853C5.18518 5.75102 4.94857 5.52469 4.67078 5.52469H3.05556C2.78806 5.52469 2.55144 5.75102 2.55144 6.01853C2.55144 6.29629 2.78806 6.53293 3.05556 6.53293H4.67078ZM4.67078 8.59054C4.94857 8.59054 5.18518 8.35392 5.18518 8.08642C5.18518 7.81893 4.94857 7.5926 4.67078 7.5926H3.05556C2.78806 7.5926 2.55144 7.81893 2.55144 8.08642C2.55144 8.35392 2.78806 8.59054 3.05556 8.59054H4.67078Z"
                                    fill="#767676"
                                />
                            </svg>
                        </button>
                        <button style={props.icon_button_style} aria-label="Back">
                            <svg
                                width="9"
                                height="16"
                                viewBox="0 0 9 16"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M7.5 1.5L1 8L7.5 14.5"
                                    stroke="#737373"
                                    stroke-width="1.5"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                />
                            </svg>
                        </button>
                        <button style={props.icon_button_style} aria-label="Forward">
                            <svg
                                width="9"
                                height="16"
                                viewBox="0 0 9 16"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M1 14.5L7.5 8L1 1.5"
                                    stroke="#BFBFBF"
                                    stroke-width="1.5"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                />
                            </svg>
                        </button>
                    }
                }
            </div>
            if props.show_address_bar {
                <div style={address_wrapper_style}>
                    <AddressBar
                        url={props.url.clone()}
                        placeholder={props.placeholder}
                        on_url_change={props.on_url_change.clone().unwrap_or_default()}
                        read_only={props.read_only}
                        input_class={props.input_class}
                        container_class={props.container_class}
                        refresh_button_style={props.refresh_button_style}
                        refresh_button_aria_label={props.refresh_button_aria_label}
                    />
                </div>
            }
            <div style="display: flex; align-items: center; gap: 6px; margin-left: auto;">
                if props.show_controls {
                    { for props.custom_buttons.iter().cloned() }
                    <button
                        style={props.icon_button_style}
                        onclick={share_onclick.clone()}
                        onmouseover={share_onmouseover.clone()}
                        onmouseout={share_onmouseout.clone()}
                        onfocus={props.share_onfocus.clone()}
                        onblur={props.share_onblur.clone()}
                        aria-label="Share"
                        title="Share"
                        tabindex={props.share_tabindex}
                    >
                        <svg
                            width="15"
                            height="19"
                            viewBox="0 0 15 19"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M7.49467 12.3969C7.91045 12.3969 8.26225 12.056 8.26225 11.6513V3.34416L8.1983 2.06613L8.64605 2.55604L9.81876 3.82343C9.95736 3.97254 10.1493 4.04709 10.3305 4.04709C10.7356 4.04709 11.0341 3.77017 11.0341 3.38676C11.0341 3.17377 10.9488 3.02467 10.7996 2.88621L8.04905 0.255589C7.85715 0.0638861 7.69722 0 7.49467 0C7.30277 0 7.14286 0.0638861 6.94029 0.255589L4.18977 2.88621C4.05117 3.02467 3.96589 3.17377 3.96589 3.38676C3.96589 3.77017 4.25372 4.04709 4.65885 4.04709C4.84009 4.04709 5.04264 3.97254 5.18124 3.82343L6.35395 2.55604L6.80171 2.06613L6.73774 3.34416V11.6513C6.73774 12.056 7.08955 12.3969 7.49467 12.3969ZM2.71855 19H12.2814C14.1045 19 15 18.1054 15 16.3161V8.12611C15 6.33688 14.1045 5.44225 12.2814 5.44225H9.98934V6.98654H12.2601C13.0171 6.98654 13.4648 7.4019 13.4648 8.20066V16.2416C13.4648 17.051 13.0171 17.4557 12.2601 17.4557H2.73988C1.97228 17.4557 1.53519 17.051 1.53519 16.2416V8.20066C1.53519 7.4019 1.97228 6.98654 2.73988 6.98654H5.01065V5.44225H2.71855C0.906181 5.44225 0 6.33688 0 8.12611V16.3161C0 18.1054 0.906181 19 2.71855 19Z"
                                fill="#767676"
                            />
                        </svg>
                    </button>
                    <button
                        style={props.icon_button_style}
                        onclick={tabs_onclick.clone()}
                        onmouseover={tabs_onmouseover.clone()}
                        onmouseout={tabs_onmouseout.clone()}
                        onfocus={props.tabs_onfocus.clone()}
                        onblur={props.tabs_onblur.clone()}
                        aria-label="Tabs"
                        title="Tabs"
                        tabindex={props.tabs_tabindex}
                    >
                        <svg
                            width="15"
                            height="15"
                            viewBox="0 0 15 15"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M7.01662 14.6401C7.4887 14.6401 7.87493 14.2646 7.87493 13.7925V8.3745H13.1642C13.6255 8.3745 14.0225 7.97755 14.0225 7.50547C14.0225 7.03341 13.6255 6.63643 13.1642 6.63643H7.87493V1.20768C7.87493 0.735619 7.4887 0.360107 7.01662 0.360107C6.54456 0.360107 6.14758 0.735619 6.14758 1.20768V6.63643H0.869031C0.396973 6.63643 0 7.03341 0 7.50547C0 7.97755 0.396973 8.3745 0.869031 8.3745H6.14758V13.7925C6.14758 14.2646 6.54456 14.6401 7.01662 14.6401Z"
                                fill="#767676"
                            />
                        </svg>
                    </button>
                    <button
                        style={props.icon_button_style}
                        onclick={more_onclick.clone()}
                        onmouseover={more_onmouseover.clone()}
                        onmouseout={more_onmouseout.clone()}
                        onfocus={props.more_onfocus.clone()}
                        onblur={props.more_onblur.clone()}
                        aria-label="More options"
                        title="More options"
                        tabindex={props.more_tabindex}
                    >
                        <svg
                            width="18"
                            height="19"
                            viewBox="0 0 18 19"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M2.67776 14.2898H3.97934V15.5914C3.97934 17.3407 4.85401 18.205 6.63458 18.205H14.8189C16.5891 18.205 17.4742 17.3407 17.4742 15.5914V7.32373C17.4742 5.5744 16.5891 4.71016 14.8189 4.71016H13.5174V3.40857C13.5174 1.65923 12.6323 0.794983 10.8621 0.794983H2.67776C0.897191 0.794983 0.022522 1.65923 0.022522 3.40857V11.6762C0.022522 13.4256 0.897191 14.2898 2.67776 14.2898ZM2.69859 12.7904C1.94886 12.7904 1.52195 12.3843 1.52195 11.5929V3.49187C1.52195 2.70051 1.94886 2.29442 2.69859 2.29442H10.8413C11.591 2.29442 12.0179 2.70051 12.0179 3.49187V4.71016H6.63458C4.85401 4.71016 3.97934 5.5744 3.97934 7.32373V12.7904H2.69859ZM6.65539 16.7056C5.90568 16.7056 5.47878 16.2995 5.47878 15.5081V7.40704C5.47878 6.61567 5.90568 6.20957 6.65539 6.20957H14.7981C15.5478 6.20957 15.9747 6.61567 15.9747 7.40704V15.5081C15.9747 16.2995 15.5478 16.7056 14.7981 16.7056H6.65539Z"
                                fill="#767676"
                            />
                        </svg>
                    </button>
                }
            </div>
        </header>
    }
}

#[derive(Clone, PartialEq)]
pub struct KeyboardNavigationOptions {
    pub on_escape: Option<Callback<()>>,
    pub on_enter: Option<Callback<()>>,
    pub trap_focus: bool,
}

#[hook]
pub fn use_keyboard(options: KeyboardNavigationOptions) -> NodeRef {
    let container_ref = use_node_ref();

    {
        let options = options.clone();
        let container_ref = container_ref.clone();

        use_effect(move || {
            let closure = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(
                move |event: KeyboardEvent| {
                    let key = event.key();
                    let target = event.target();

                    match key.as_str() {
                        "Escape" => {
                            if let Some(callback) = &options.on_escape {
                                event.prevent_default();
                                callback.emit(());
                            }
                        }
                        "Enter" => {
                            if let Some(callback) = &options.on_enter {
                                if let Some(target_elem) =
                                    target.and_then(|t| t.dyn_into::<Element>().ok())
                                {
                                    if Some(target_elem) == container_ref.cast::<Element>() {
                                        event.prevent_default();
                                        callback.emit(());
                                    }
                                }
                            }
                        }
                        "Tab" if options.trap_focus => {
                            if let Some(container) = container_ref.cast::<Element>() {
                                let selector = "button, [href], input, select, textarea, [tabindex]:not([tabindex=\"-1\"])";
                                let focusables = container.query_selector_all(selector).unwrap();

                                let length = focusables.length();
                                if length == 0 {
                                    return;
                                }

                                let first = focusables
                                    .item(0)
                                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());
                                let last = focusables
                                    .item(length - 1)
                                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());

                                let document = web_sys::window().unwrap().document().unwrap();
                                let active = document.active_element();

                                if event.shift_key() {
                                    if active == first.as_ref().map(|e| e.clone().into()) {
                                        event.prevent_default();
                                        if let Some(elem) = last {
                                            elem.focus().ok();
                                        }
                                    }
                                } else if active == last.as_ref().map(|e| e.clone().into()) {
                                    event.prevent_default();
                                    if let Some(elem) = first {
                                        elem.focus().ok();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },
            )
                as Box<dyn Fn(KeyboardEvent)>);

            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();

            move || {
                web_sys::window()
                    .unwrap()
                    .remove_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                drop(closure);
            }
        });
    }

    container_ref
}

/// Properties for the `BrowserFrame` component.
///
/// This struct provides a wide range of customization options and handlers
/// for the browser frame's appearance and behavior.
#[derive(Properties, PartialEq, Clone)]
pub struct BrowserFrameProps {
    /// Child components to render inside the browser frame.
    #[prop_or_default]
    pub children: Children,

    /// The current URL displayed in the address bar.
    #[prop_or_default]
    pub url: String,

    /// Placeholder text for the address bar input.
    #[prop_or_default]
    pub placeholder: &'static str,

    /// Callback for when the URL is changed by the user.
    #[prop_or_default]
    pub on_url_change: Option<Callback<InputEvent>>,

    /// Callback when the close button is clicked.
    #[prop_or_default]
    pub on_close: Callback<()>,

    /// Callback when the minimize button is clicked.
    #[prop_or_default]
    pub on_minimize: Callback<()>,

    /// Callback when the maximize button is clicked.
    #[prop_or_default]
    pub on_maximize: Callback<()>,

    /// Whether to show the window controls (close, minimize, maximize).
    ///
    /// Defaults to `true`.
    #[prop_or(true)]
    pub show_controls: bool,

    /// Whether to show the address bar.
    ///
    /// Defaults to `true`.
    #[prop_or(true)]
    pub show_address_bar: bool,

    /// Whether the address bar is read-only.
    ///
    /// Defaults to `false`.
    #[prop_or(false)]
    pub read_only: bool,

    /// The size of the browser frame (e.g., small, medium, large).
    #[prop_or_default]
    pub size: Size,

    /// The visual variant of the browser frame.
    #[prop_or_default]
    pub variant: Variant,

    /// Custom buttons to render in the browser header.
    #[prop_or_default]
    pub custom_buttons: Vec<Html>,

    /// CSS classes for styling the outer container of the browser frame.
    ///
    /// Defaults to: `"rounded-lg border shadow-lg overflow-hidden bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700"`.
    #[prop_or(
        "rounded-lg border shadow-lg overflow-hidden bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700"
    )]
    pub class: &'static str,

    /// Inline styles for the outer container.
    #[prop_or_default]
    pub style: &'static str,

    /// Optional ID for the outer container.
    #[prop_or_default]
    pub id: &'static str,

    /// ARIA label for the browser frame container.
    ///
    /// Defaults to `"Browser window"`.
    #[prop_or("Browser window")]
    pub aria_label: &'static str,

    /// ARIA description for the browser frame container.
    #[prop_or_default]
    pub aria_describedby: &'static str,

    /// CSS classes for the address bar container.
    ///
    /// Defaults to: `"flex-1 mx-4 border border-gray-300 dark:border-gray-500 rounded-md px-3 py-1 bg-transparent text-sm relative bg-white dark:bg-gray-700"`.
    #[prop_or(
        "flex-1 mx-4 border border-gray-300 dark:border-gray-500 rounded-md px-3 py-1 bg-transparent text-sm relative bg-white dark:bg-gray-700"
    )]
    pub container_class: &'static str,

    /// CSS classes for the address bar input element.
    ///
    /// Defaults to: `"w-full text-gray-900 dark:text-white bg-white dark:bg-gray-700 pr-8"`.
    #[prop_or("w-full text-gray-900 dark:text-white bg-white dark:bg-gray-700 pr-8")]
    pub input_class: &'static str,

    /// Inline styles for the refresh button.
    ///
    /// Defaults to: `"position: absolute; top: 50%; right: 8px; transform: translateY(-50%); padding: 4px; border: none; cursor: pointer; color: #d1d5db;"`.
    #[prop_or(
        "position: absolute; top: 50%; right: 8px; transform: translateY(-50%); padding: 4px; border: none; cursor: pointer; color: #d1d5db;"
    )]
    pub refresh_button_style: &'static str,

    /// ARIA label for the refresh button.
    ///
    /// Defaults to `"Refresh"`.
    #[prop_or("Refresh")]
    pub refresh_button_aria_label: &'static str,

    /// Inline styles for icon buttons (close, minimize, maximize).
    ///
    /// Defaults to: `"padding: 4px; border: none; cursor: pointer; color: #d1d5db;"`.
    #[prop_or("padding: 4px; border: none; cursor: pointer; color: #d1d5db;")]
    pub icon_button_style: &'static str,

    /// Inline styles for the address bar wrapper.
    ///
    /// Defaults to: `"flex: 1; display: flex; justify-content: center; padding-right: 8px;"`.
    #[prop_or("flex: 1; display: flex; justify-content: center; padding-right: 8px;")]
    pub address_wrapper_base_style: &'static str,

    /// Inline styles for the header container.
    ///
    /// Defaults to: `"display: flex; align-items: center; position: relative;"`.
    #[prop_or("display: flex; align-items: center; position: relative;")]
    pub header_base_style: &'static str,

    /// Callbacks and styles for the close button and related elements.
    #[prop_or_default]
    pub on_close_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_close_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_close_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_close_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub close_class: &'static str,
    #[prop_or_default]
    pub close_svg_class: &'static str,
    #[prop_or_default]
    pub close_path_class: &'static str,
    #[prop_or("button")]
    pub close_button_type: &'static str,
    #[prop_or_default]
    pub close_aria_label: &'static str,
    #[prop_or_default]
    pub close_title: &'static str,
    #[prop_or("0")]
    pub close_tabindex: &'static str,

    /// Callbacks and styles for the minimize button and related elements.
    #[prop_or_default]
    pub on_minimize_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_minimize_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_minimize_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_minimize_blur: Callback<FocusEvent>,
    #[prop_or("ml-2")]
    pub minimize_class: &'static str,
    #[prop_or_default]
    pub minimize_svg_class: &'static str,
    #[prop_or_default]
    pub minimize_path_class: &'static str,
    #[prop_or("button")]
    pub minimize_button_type: &'static str,
    #[prop_or_default]
    pub minimize_aria_label: &'static str,
    #[prop_or_default]
    pub minimize_title: &'static str,
    #[prop_or("0")]
    pub minimize_tabindex: &'static str,

    /// Callbacks and styles for the maximize button and related elements.
    #[prop_or_default]
    pub on_maximize_mouse_over: Callback<()>,
    #[prop_or_default]
    pub on_maximize_mouse_out: Callback<()>,
    #[prop_or_default]
    pub on_maximize_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_maximize_blur: Callback<FocusEvent>,
    #[prop_or("ml-2")]
    pub maximize_class: &'static str,
    #[prop_or_default]
    pub maximize_svg_class: &'static str,
    #[prop_or_default]
    pub maximize_path_class: &'static str,
    #[prop_or("button")]
    pub maximize_button_type: &'static str,
    #[prop_or_default]
    pub maximize_aria_label: &'static str,
    #[prop_or_default]
    pub maximize_title: &'static str,
    #[prop_or("0")]
    pub maximize_tabindex: &'static str,

    /// Style and callbacks for the share button.
    #[prop_or_default]
    pub share_button_style: &'static str,
    #[prop_or_default]
    pub share_onclick: Callback<()>,
    #[prop_or_default]
    pub share_onmouseover: Callback<()>,
    #[prop_or_default]
    pub share_onmouseout: Callback<()>,
    #[prop_or_default]
    pub share_onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub share_onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub share_tabindex: &'static str,

    /// Style and callbacks for the tabs button.
    #[prop_or_default]
    pub tabs_button_style: &'static str,
    #[prop_or_default]
    pub tabs_onclick: Callback<()>,
    #[prop_or_default]
    pub tabs_onmouseover: Callback<()>,
    #[prop_or_default]
    pub tabs_onmouseout: Callback<()>,
    #[prop_or_default]
    pub tabs_onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub tabs_onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub tabs_tabindex: &'static str,

    /// Style and callbacks for the more button.
    #[prop_or_default]
    pub more_button_style: &'static str,
    #[prop_or_default]
    pub more_onclick: Callback<()>,
    #[prop_or_default]
    pub more_onmouseover: Callback<()>,
    #[prop_or_default]
    pub more_onmouseout: Callback<()>,
    #[prop_or_default]
    pub more_onfocus: Callback<FocusEvent>,
    #[prop_or_default]
    pub more_onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub more_tabindex: &'static str,
}
/// BrowserFrame Component
///
/// A Yew component that emulates a browser window, complete with customizable controls (close, minimize, maximize),
/// an address bar, and optional custom buttons. It wraps its child components in a browser-like interface and provides
/// various hooks for interaction events such as focus, hover, and clicks.
///
/// # Features
/// - Configurable address bar and controls (show/hide, read-only).
/// - Fully customizable styling and classes for different parts of the frame.
/// - Emits callbacks for URL changes and control interactions (close, minimize, maximize).
/// - Supports additional custom buttons and slots for user-defined functionality.
/// - Keyboard navigation support (Escape to close).
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use browser_rs::yew::BrowserFrame;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let on_close = Callback::from(|_| log::info!("Browser closed"));
///
///     html! {
///         <BrowserFrame
///             url={"https://opensass.org".to_string()}
///             on_close={on_close}
///         >
///             <p>{ "Your embedded content here." }</p>
///         </BrowserFrame>
///     }
/// }
/// ```
///
/// ## With Custom Buttons
/// ```rust
/// use yew::prelude::*;
/// use browser_rs::yew::BrowserFrame;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let custom_button = html! {
///         <button>{ "Custom Button" }</button>
///     };
///
///     html! {
///         <BrowserFrame
///             url={"https://opensass.org".to_string()}
///             custom_buttons={vec![custom_button]}
///         >
///             <p>{ "Custom button in the header!" }</p>
///         </BrowserFrame>
///     }
/// }
/// ```
///
/// ## Styling and Class Customization
/// ```rust
/// use yew::prelude::*;
/// use browser_rs::yew::BrowserFrame;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <BrowserFrame
///             url={"https://opensass.org".to_string()}
///             class={"rounded-xl shadow-xl"}
///             input_class={"bg-gray-200 text-gray-900"}
///             container_class={"flex-1 mx-4"}
///         >
///             <p>{ "Styled browser frame!" }</p>
///         </BrowserFrame>
///     }
/// }
/// ```
///
/// # Behavior
/// - The `BrowserFrame` uses a `BrowserHeader` subcomponent for controls and an address bar,
///   and a `BrowserContent` subcomponent for rendering child content.
/// - The `on_url_change` callback is called when the address bar's URL changes.
/// - Control buttons (close, minimize, maximize) emit their respective callbacks when interacted with.
/// - Keyboard navigation is enabled: Escape key triggers the `on_close` callback.
///
/// # Notes
/// - Supports both light and dark themes through provided classes and styles.
/// - Default styling can be customized via `class`, `style`, and other related props.
/// - Accessibility attributes (`aria-*`) are provided.
#[function_component(BrowserFrame)]
pub fn browser_frame(props: &BrowserFrameProps) -> Html {
    let on_close = props.on_close.clone();
    let container_ref = use_keyboard(KeyboardNavigationOptions {
        on_escape: Some(Callback::from(move |_| on_close.emit(()))),
        on_enter: None,
        trap_focus: false,
    });

    let size_style = props.size.to_style();
    let combined_style = format!("{} {}", size_style, props.style);

    html! {
        <article
            ref={container_ref}
            id={props.id}
            class={props.class}
            style={combined_style}
            role="application"
            aria-label={props.aria_label}
            aria-describedby={props.aria_describedby}
            tabindex={Some("-1")}
        >
            <BrowserHeader
                url={props.url.clone()}
                placeholder={props.placeholder}
                on_url_change={props.on_url_change.clone()}
                on_close={props.on_close.clone()}
                on_minimize={props.on_minimize.clone()}
                on_maximize={props.on_maximize.clone()}
                show_controls={props.show_controls}
                show_address_bar={props.show_address_bar}
                read_only={props.read_only}
                variant={props.variant.clone()}
                size={props.size.clone()}
                custom_buttons={props.custom_buttons.clone()}
                class=""
                container_class={props.container_class}
                input_class={props.input_class}
                refresh_button_style={props.refresh_button_style}
                refresh_button_aria_label={props.refresh_button_aria_label}
                icon_button_style={props.icon_button_style}
                address_wrapper_base_style={props.address_wrapper_base_style}
                header_base_style={props.header_base_style}
                on_close_mouse_over={props.on_close_mouse_over.clone()}
                on_close_mouse_out={props.on_close_mouse_out.clone()}
                on_close_focus={props.on_close_focus.clone()}
                on_close_blur={props.on_close_blur.clone()}
                close_class={props.close_class}
                close_svg_class={props.close_svg_class}
                close_path_class={props.close_path_class}
                close_button_type={props.close_button_type}
                close_aria_label={props.close_aria_label}
                close_title={props.close_title}
                close_tabindex={props.close_tabindex}
                on_minimize_mouse_over={props.on_minimize_mouse_over.clone()}
                on_minimize_mouse_out={props.on_minimize_mouse_out.clone()}
                on_minimize_focus={props.on_minimize_focus.clone()}
                on_minimize_blur={props.on_minimize_blur.clone()}
                minimize_class={props.minimize_class}
                minimize_svg_class={props.minimize_svg_class}
                minimize_path_class={props.minimize_path_class}
                minimize_button_type={props.minimize_button_type}
                minimize_aria_label={props.minimize_aria_label}
                minimize_title={props.minimize_title}
                minimize_tabindex={props.minimize_tabindex}
                on_maximize_mouse_over={props.on_maximize_mouse_over.clone()}
                on_maximize_mouse_out={props.on_maximize_mouse_out.clone()}
                on_maximize_focus={props.on_maximize_focus.clone()}
                on_maximize_blur={props.on_maximize_blur.clone()}
                maximize_class={props.maximize_class}
                maximize_svg_class={props.maximize_svg_class}
                maximize_path_class={props.maximize_path_class}
                maximize_button_type={props.maximize_button_type}
                maximize_aria_label={props.maximize_aria_label}
                maximize_title={props.maximize_title}
                maximize_tabindex={props.maximize_tabindex}
                share_button_style={props.share_button_style}
                share_onclick={props.share_onclick.clone()}
                share_onmouseover={props.share_onmouseover.clone()}
                share_onmouseout={props.share_onmouseout.clone()}
                share_onfocus={props.share_onfocus.clone()}
                share_onblur={props.share_onblur.clone()}
                share_tabindex={props.share_tabindex}
                tabs_button_style={props.tabs_button_style}
                tabs_onclick={props.tabs_onclick.clone()}
                tabs_onmouseover={props.tabs_onmouseover.clone()}
                tabs_onmouseout={props.tabs_onmouseout.clone()}
                tabs_onfocus={props.tabs_onfocus.clone()}
                tabs_onblur={props.tabs_onblur.clone()}
                tabs_tabindex={props.tabs_tabindex}
                more_button_style={props.more_button_style}
                more_onclick={props.more_onclick.clone()}
                more_onmouseover={props.more_onmouseover.clone()}
                more_onmouseout={props.more_onmouseout.clone()}
                more_onfocus={props.more_onfocus.clone()}
                more_onblur={props.more_onblur.clone()}
                more_tabindex={props.more_tabindex}
            />
            <BrowserContent aria_describedby={props.aria_describedby}>
                { for props.children.iter() }
            </BrowserContent>
        </article>
    }
}
