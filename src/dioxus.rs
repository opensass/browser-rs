#![doc = include_str!("../DIOXUS.md")]

use crate::common::{ButtonType, Size, Variant};
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use std::rc::Rc;
use web_sys::{
    HtmlInputElement,
    wasm_bindgen::{JsCast, prelude::*},
    window,
};

#[derive(PartialEq, Props, Clone)]
pub struct BrowserContentProps {
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default = "Browser content area")]
    pub aria_label: &'static str,
    #[props(default)]
    pub aria_describedby: &'static str,
    children: Element,
}

#[component]
pub fn BrowserContent(props: BrowserContentProps) -> Element {
    rsx! {
        main {
            class: "{props.class}",
            style: "{props.style}",
            role: "main",
            aria_label: "{props.aria_label}",
            aria_describedby: "{props.aria_describedby}",
            tabindex: "-1",
            {props.children}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct AddressBarProps {
    #[props(default)]
    pub url: String,
    #[props(default = "Enter URL or search...")]
    pub placeholder: &'static str,
    #[props(default)]
    pub on_url_change: EventHandler<FormEvent>,
    #[props(default)]
    pub read_only: bool,
    #[props(default)]
    pub class: &'static str,
    #[props(
        default = "flex: 1; margin-left: 1rem; margin-right: 1rem; border: 1px solid #d1d5db; border-radius: 0.375rem; padding-left: 0.75rem; padding-right: 0.75rem; font-size: 0.875rem; position: relative;"
    )]
    pub style: &'static str,
    #[props(default = "Website address or search query")]
    pub label: &'static str,
    #[props(default = "Enter a website URL or search term. Press Enter to navigate.")]
    pub describedby: &'static str,
    #[props(default = "browser-url-input")]
    pub input_id: &'static str,
    #[props(default = "text-black dark:text-white")]
    pub input_class: &'static str,
    #[props(default)]
    pub container_class: &'static str,
    #[props(
        default = "position: absolute; top: 50%; right: 8px; transform: translateY(-50%); padding: 4px; background: none; border: none; box-shadow: none; outline: none; cursor: pointer;"
    )]
    pub refresh_button_style: &'static str,
    #[props(default = "Refresh")]
    pub refresh_button_aria_label: &'static str,
    #[props(
        default = "background-color: transparent; padding-right: 2rem; border: none; outline: none; box-shadow: none; height: 100%;"
    )]
    pub input_style: &'static str,
}

#[component]
pub fn AddressBar(props: AddressBarProps) -> Element {
    let mut input_value = use_signal(|| props.url.clone());
    let mut is_focused = use_signal(|| false);
    let mut input_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        input_value.set(props.url.clone());
    });

    let on_input_change = move |evt: FormEvent| {
        input_value.set(evt.value());
        props.on_url_change.call(evt);
    };
    let on_key_down = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            evt.prevent_default();

            if let Some(node) = &*input_ref.read() {
                if let Some(input) = node.downcast::<HtmlInputElement>() {
                    let _ = input.blur();
                }
            }

            if let Some(document) = window().and_then(|w| w.document()) {
                let live_region = document.create_element("div").unwrap();
                live_region.set_attribute("aria-live", "polite").unwrap();
                live_region.set_attribute("aria-atomic", "true").unwrap();
                live_region.set_class_name("sr-only");
                live_region
                    .set_text_content(Some(&format!("Navigating to {}", input_value.read())));
                document.body().unwrap().append_child(&live_region).unwrap();

                let clone = live_region.clone();
                Timeout::new(1000, move || {
                    let _ = document.body().unwrap().remove_child(&clone);
                })
                .forget();
            }
        }
    };

    rsx! {
        div {
            class: "{props.container_class} {props.class}",
            style: "{props.style}",
            label {
                r#for: "{props.input_id}",
                class: "sr-only",
                "{props.label}"
            }
            input {
                id: "{props.input_id}",
                r#type: "text",
                class: "{props.input_class}",
                style: "{props.input_style}",
                value: "{input_value}",
                oninput: on_input_change,
                onkeydown: on_key_down,
                onfocus: move |_| is_focused.set(true),
                onblur: move |_| is_focused.set(false),
                placeholder: "{props.placeholder}",
                readonly: props.read_only,
                aria_describedby: "{props.describedby}",
                autocomplete: "url",
                spellcheck: "false",
                onmounted: move |cx| input_ref.set(Some(cx.data())),
            }
            button {
                style: "{props.refresh_button_style}",
                aria_label: "{props.refresh_button_aria_label}",
                onclick: move |_| {
                    let _ = window().unwrap().location().reload();
                },
                svg {
                    width: "11",
                    height: "13",
                    view_box: "0 0 11 13",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M4.99385 1.00002L7.33006 3.33623L4.99385 5.67244M10 7.61925C10 10.1998 7.9081 12.2917 5.3276 12.2917C2.74709 12.2917 0.655182 10.1998 0.655182 7.61925C0.655182 5.03875 2.74709 2.94684 5.3276 2.94684C5.8737 2.94684 6.4957 2.94684 7.27443 3.33621",
                        stroke: "#767676",
                        stroke_linecap: "round",
                        stroke_linejoin: "round"
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ControlButtonProps {
    pub r#type: ButtonType,
    #[props(default)]
    pub on_click: EventHandler<()>,
    #[props(default)]
    pub on_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_blur: EventHandler<FocusEvent>,
    #[props(
        default = "width: 1rem; height: 1rem; display: flex; align-items: center; justify-content: center; transition: all 0.2s ease; cursor: pointer; background: none; border: none; padding: 0; margin-right: 0.5rem;"
    )]
    pub style: &'static str,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub svg_class: &'static str,
    #[props(default)]
    pub path_class: &'static str,
    #[props(default = "button")]
    pub button_type: &'static str,
    #[props(default)]
    pub aria_label: &'static str,
    #[props(default)]
    pub title: &'static str,
    #[props(default = "0")]
    pub tabindex: &'static str,
}

#[component]
pub fn ControlButton(props: ControlButtonProps) -> Element {
    let (fill, stroke) = match props.r#type {
        ButtonType::Close => ("#FF5F57", "#E14640"),
        ButtonType::Minimize => ("#FFBD2E", "#DFA123"),
        ButtonType::Maximize => ("#28CA42", "#1DAD2C"),
    };

    let aria_label = if props.aria_label.is_empty() {
        props.r#type.default_aria_label()
    } else {
        props.aria_label
    };

    let title = if props.title.is_empty() {
        props.r#type.default_title()
    } else {
        props.title
    };

    rsx! {
        button {
            r#type: "{props.button_type}",
            class: "{props.class}",
            style: "{props.style}",
            aria_label: "{aria_label}",
            title: "{title}",
            tabindex: "{props.tabindex}",
            onclick: move |_| props.on_click.call(()),
            onmouseover: move |_| props.on_mouse_over.call(()),
            onmouseout: move |_| props.on_mouse_out.call(()),
            onfocus: props.on_focus,
            onblur: props.on_blur,

            svg {
                class: "{props.svg_class}",
                width: "12",
                height: "12",
                view_box: "0 0 12 12",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    class: "{props.path_class}",
                    d: "M6 0.5C9.03757 0.5 11.5 2.96243 11.5 6C11.5 9.03757 9.03757 11.5 6 11.5C2.96243 11.5 0.5 9.03757 0.5 6C0.5 2.96243 2.96243 0.5 6 0.5Z",
                    fill: "{fill}",
                    stroke: "{stroke}"
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct BrowserControlsProps {
    #[props(default)]
    pub show_controls: bool,
    #[props(default)]
    pub class: &'static str,
    #[props(default = "display: flex; align-items: center; background: none; padding-left: 10px;")]
    pub style: &'static str,

    #[props(default)]
    pub on_close: EventHandler<()>,
    #[props(default)]
    pub on_close_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_close_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_close_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_close_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub close_class: &'static str,
    #[props(default)]
    pub close_svg_class: &'static str,
    #[props(default)]
    pub close_path_class: &'static str,
    #[props(default = "button")]
    pub close_button_type: &'static str,
    #[props(default)]
    pub close_aria_label: &'static str,
    #[props(default)]
    pub close_title: &'static str,
    #[props(default = "0")]
    pub close_tabindex: &'static str,

    #[props(default)]
    pub on_minimize: EventHandler<()>,
    #[props(default)]
    pub on_minimize_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_minimize_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_minimize_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_minimize_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub minimize_class: &'static str,
    #[props(default)]
    pub minimize_svg_class: &'static str,
    #[props(default)]
    pub minimize_path_class: &'static str,
    #[props(default = "button")]
    pub minimize_button_type: &'static str,
    #[props(default)]
    pub minimize_aria_label: &'static str,
    #[props(default)]
    pub minimize_title: &'static str,
    #[props(default = "0")]
    pub minimize_tabindex: &'static str,

    #[props(default)]
    pub on_maximize: EventHandler<()>,
    #[props(default)]
    pub on_maximize_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_maximize_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_maximize_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_maximize_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub maximize_class: &'static str,
    #[props(default)]
    pub maximize_svg_class: &'static str,
    #[props(default)]
    pub maximize_path_class: &'static str,
    #[props(default = "button")]
    pub maximize_button_type: &'static str,
    #[props(default)]
    pub maximize_aria_label: &'static str,
    #[props(default)]
    pub maximize_title: &'static str,
    #[props(default = "0")]
    pub maximize_tabindex: &'static str,
}

#[component]
pub fn BrowserControls(props: BrowserControlsProps) -> Element {
    if !props.show_controls {
        return rsx! {};
    }

    rsx! {
        nav {
            class: "{props.class}",
            style: "{props.style}",
            role: "toolbar",
            aria_label: "Browser window controls",
            ControlButton {
                r#type: ButtonType::Close,
                on_click: props.on_close,
                on_mouse_over: props.on_close_mouse_over,
                on_mouse_out: props.on_close_mouse_out,
                on_focus: props.on_close_focus,
                on_blur: props.on_close_blur,
                class: props.close_class,
                svg_class: props.close_svg_class,
                path_class: props.close_path_class,
                button_type: props.close_button_type,
                aria_label: props.close_aria_label,
                title: props.close_title,
                tabindex: props.close_tabindex,
            }
            ControlButton {
                r#type: ButtonType::Minimize,
                on_click: props.on_minimize,
                on_mouse_over: props.on_minimize_mouse_over,
                on_mouse_out: props.on_minimize_mouse_out,
                on_focus: props.on_minimize_focus,
                on_blur: props.on_minimize_blur,
                class: props.minimize_class,
                svg_class: props.minimize_svg_class,
                path_class: props.minimize_path_class,
                button_type: props.minimize_button_type,
                aria_label: props.minimize_aria_label,
                title: props.minimize_title,
                tabindex: props.minimize_tabindex,
            }
            ControlButton {
                r#type: ButtonType::Maximize,
                on_click: props.on_maximize,
                on_mouse_over: props.on_maximize_mouse_over,
                on_mouse_out: props.on_maximize_mouse_out,
                on_focus: props.on_maximize_focus,
                on_blur: props.on_maximize_blur,
                class: props.maximize_class,
                svg_class: props.maximize_svg_class,
                path_class: props.maximize_path_class,
                button_type: props.maximize_button_type,
                aria_label: props.maximize_aria_label,
                title: props.maximize_title,
                tabindex: props.maximize_tabindex,
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct BrowserHeaderProps {
    #[props(default)]
    pub url: String,
    #[props(default)]
    pub placeholder: &'static str,
    #[props(default)]
    pub on_url_change: Option<EventHandler<FormEvent>>,
    #[props(default = true)]
    pub show_controls: bool,
    #[props(default = true)]
    pub show_address_bar: bool,
    #[props(default = false)]
    pub read_only: bool,
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub custom_buttons: Vec<Element>,
    #[props(default)]
    pub class: &'static str,

    #[props(default)]
    pub container_class: &'static str,
    #[props(default = "text-black dark:text-white")]
    pub input_class: &'static str,
    #[props(default)]
    pub refresh_button_style: &'static str,
    #[props(default = "Refresh")]
    pub refresh_button_aria_label: &'static str,

    #[props(
        default = "padding: 4px; cursor: pointer; background: none; border: none; box-shadow: none; outline: none;"
    )]
    pub icon_button_style: &'static str,

    #[props(default = "flex: 1; display: flex; justify-content: center; padding-right: 8px;")]
    pub address_wrapper_base_style: &'static str,

    #[props(default = "display: flex; align-items: center; position: relative;")]
    pub header_base_style: &'static str,

    #[props(default)]
    pub on_close: EventHandler<()>,
    #[props(default)]
    pub on_close_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_close_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_close_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_close_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub close_class: &'static str,
    #[props(default)]
    pub close_svg_class: &'static str,
    #[props(default)]
    pub close_path_class: &'static str,
    #[props(default = "button")]
    pub close_button_type: &'static str,
    #[props(default)]
    pub close_aria_label: &'static str,
    #[props(default)]
    pub close_title: &'static str,
    #[props(default = "0")]
    pub close_tabindex: &'static str,

    #[props(default)]
    pub on_minimize: EventHandler<()>,
    #[props(default)]
    pub on_minimize_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_minimize_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_minimize_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_minimize_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub minimize_class: &'static str,
    #[props(default)]
    pub minimize_svg_class: &'static str,
    #[props(default)]
    pub minimize_path_class: &'static str,
    #[props(default = "button")]
    pub minimize_button_type: &'static str,
    #[props(default)]
    pub minimize_aria_label: &'static str,
    #[props(default)]
    pub minimize_title: &'static str,
    #[props(default = "0")]
    pub minimize_tabindex: &'static str,

    #[props(default)]
    pub on_maximize: EventHandler<()>,
    #[props(default)]
    pub on_maximize_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_maximize_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_maximize_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_maximize_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub maximize_class: &'static str,
    #[props(default)]
    pub maximize_svg_class: &'static str,
    #[props(default)]
    pub maximize_path_class: &'static str,
    #[props(default = "button")]
    pub maximize_button_type: &'static str,
    #[props(default)]
    pub maximize_aria_label: &'static str,
    #[props(default)]
    pub maximize_title: &'static str,
    #[props(default = "0")]
    pub maximize_tabindex: &'static str,

    #[props(default)]
    pub share_button_style: &'static str,
    #[props(default)]
    pub share_onclick: EventHandler<()>,
    #[props(default)]
    pub share_onmouseover: EventHandler<()>,
    #[props(default)]
    pub share_onmouseout: EventHandler<()>,
    #[props(default)]
    pub share_onfocus: EventHandler<FocusEvent>,
    #[props(default)]
    pub share_onblur: EventHandler<FocusEvent>,
    #[props(default)]
    pub share_tabindex: &'static str,

    #[props(default)]
    pub tabs_button_style: &'static str,
    #[props(default)]
    pub tabs_onclick: EventHandler<()>,
    #[props(default)]
    pub tabs_onmouseover: EventHandler<()>,
    #[props(default)]
    pub tabs_onmouseout: EventHandler<()>,
    #[props(default)]
    pub tabs_onfocus: EventHandler<FocusEvent>,
    #[props(default)]
    pub tabs_onblur: EventHandler<FocusEvent>,
    #[props(default)]
    pub tabs_tabindex: &'static str,

    #[props(default)]
    pub more_button_style: &'static str,
    #[props(default)]
    pub more_onclick: EventHandler<()>,
    #[props(default)]
    pub more_onmouseover: EventHandler<()>,
    #[props(default)]
    pub more_onmouseout: EventHandler<()>,
    #[props(default)]
    pub more_onfocus: EventHandler<FocusEvent>,
    #[props(default)]
    pub more_onblur: EventHandler<FocusEvent>,
    #[props(default)]
    pub more_tabindex: &'static str,
}

#[component]
pub fn BrowserHeader(props: BrowserHeaderProps) -> Element {
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

    let share_onclick = move |_| props.share_onclick.call(());
    let share_onmouseover = move |_| props.share_onmouseover.call(());
    let share_onmouseout = move |_| props.share_onmouseout.call(());

    let tabs_onclick = move |_| props.tabs_onclick.call(());
    let tabs_onmouseover = move |_| props.tabs_onmouseover.call(());
    let tabs_onmouseout = move |_| props.tabs_onmouseout.call(());

    let more_onclick = move |_| props.more_onclick.call(());
    let more_onmouseover = move |_| props.more_onmouseover.call(());
    let more_onmouseout = move |_| props.more_onmouseout.call(());

    rsx! {
        header {
            style: "{base_style}",
            class: "{props.class}",
            "aria-label": "Browser window header",

            div {
                style: "display: flex; align-items: center; gap: 6px;",
                if props.show_controls {
                    BrowserControls {
                        on_close: props.on_close,
                        on_minimize: props.on_minimize,
                        on_maximize: props.on_maximize,
                        show_controls: props.show_controls,
                        on_close_mouse_over: props.on_close_mouse_over,
                        on_close_mouse_out: props.on_close_mouse_out,
                        on_close_focus: props.on_close_focus,
                        on_close_blur: props.on_close_blur,
                        close_class: props.close_class,
                        close_svg_class: props.close_svg_class,
                        close_path_class: props.close_path_class,
                        close_button_type: props.close_button_type,
                        close_aria_label: props.close_aria_label,
                        close_title: props.close_title,
                        close_tabindex: props.close_tabindex,
                        on_minimize_mouse_over: props.on_minimize_mouse_over,
                        on_minimize_mouse_out: props.on_minimize_mouse_out,
                        on_minimize_focus: props.on_minimize_focus,
                        on_minimize_blur: props.on_minimize_blur,
                        minimize_class: props.minimize_class,
                        minimize_svg_class: props.minimize_svg_class,
                        minimize_path_class: props.minimize_path_class,
                        minimize_button_type: props.minimize_button_type,
                        minimize_aria_label: props.minimize_aria_label,
                        minimize_title: props.minimize_title,
                        minimize_tabindex: props.minimize_tabindex,
                        on_maximize_mouse_over: props.on_maximize_mouse_over,
                        on_maximize_mouse_out: props.on_maximize_mouse_out,
                        on_maximize_focus: props.on_maximize_focus,
                        on_maximize_blur: props.on_maximize_blur,
                        maximize_class: props.maximize_class,
                        maximize_svg_class: props.maximize_svg_class,
                        maximize_path_class: props.maximize_path_class,
                        maximize_button_type: props.maximize_button_type,
                        maximize_aria_label: props.maximize_aria_label,
                        maximize_title: props.maximize_title,
                        maximize_tabindex: props.maximize_tabindex,
                    }
                    if !is_ios {
                        button {
                            style: "{props.icon_button_style}",
                            "aria-label": "Sidebar",
                            svg {
                                width: "20",
                                height: "15",
                                view_box: "0 0 20 15",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M2.62346 15H16.4609C18.2202 15 19.0844 14.1358 19.0844 12.4074V2.59259C19.0844 0.864204 18.2202 0 16.4609 0H2.62346C0.874483 0 0 0.864204 0 2.59259V12.4074C0 14.1358 0.874483 15 2.62346 15ZM2.64404 13.5082C1.90329 13.5082 1.48149 13.1173 1.48149 12.3354V2.66461C1.48149 1.89301 1.90329 1.49177 2.64404 1.49177H6.22427V13.5082H2.64404ZM16.4403 1.49177C17.1811 1.49177 17.6029 1.89301 17.6029 2.66461V12.3354C17.6029 13.1173 17.1811 13.5082 16.4403 13.5082H7.67489V1.49177H16.4403ZM4.67078 4.47532C4.94857 4.47532 5.18518 4.2284 5.18518 3.9609C5.18518 3.69341 4.94857 3.46708 4.67078 3.46708H3.05556C2.78806 3.46708 2.55144 3.69341 2.55144 3.9609C2.55144 4.2284 2.78806 4.47532 3.05556 4.47532H4.67078ZM4.67078 6.53293C4.94857 6.53293 5.18518 6.29629 5.18518 6.01853C5.18518 5.75102 4.94857 5.52469 4.67078 5.52469H3.05556C2.78806 5.52469 2.55144 5.75102 2.55144 6.01853C2.55144 6.29629 2.78806 6.53293 3.05556 6.53293H4.67078ZM4.67078 8.59054C4.94857 8.59054 5.18518 8.35392 5.18518 8.08642C5.18518 7.81893 4.94857 7.5926 4.67078 7.5926H3.05556C2.78806 7.5926 2.55144 7.81893 2.55144 8.08642C2.55144 8.35392 2.78806 8.59054 3.05556 8.59054H4.67078Z",
                                    fill: "#767676",
                                }
                            }
                        }
                        button {
                            style: "{props.icon_button_style}",
                            "aria-label": "Back",
                            svg {
                                width: "9",
                                height: "16",
                                view_box: "0 0 9 16",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M7.5 1.5L1 8L7.5 14.5",
                                    stroke: "#737373",
                                    stroke_width: "1.5",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                }
                            }
                        }
                        button {
                            style: "{props.icon_button_style}",
                            "aria-label": "Forward",
                            svg {
                                width: "9",
                                height: "16",
                                view_box: "0 0 9 16",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M1 14.5L7.5 8L1 1.5",
                                    stroke: "#BFBFBF",
                                    stroke_width: "1.5",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                }
                            }
                        }
                    }
                }
            }

            if props.show_address_bar {
                div {
                    style: "{address_wrapper_style}",
                    AddressBar {
                        url: props.url,
                        placeholder: props.placeholder,
                        on_url_change: props.on_url_change.unwrap_or_default(),
                        read_only: props.read_only,
                        input_class: props.input_class,
                        container_class: props.container_class,
                        refresh_button_style: props.refresh_button_style,
                        refresh_button_aria_label: props.refresh_button_aria_label,
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 6px; margin-left: auto;",
                if props.show_controls {
                    for btn in &props.custom_buttons {
                        {btn}
                    }
                    button {
                        style: "{props.icon_button_style}",
                        onclick: share_onclick,
                        onmouseover: share_onmouseover,
                        onmouseout: share_onmouseout,
                        onfocus: props.share_onfocus,
                        onblur: props.share_onblur,
                        "aria-label": "Share",
                        title: "Share",
                        tabindex: "{props.share_tabindex}",
                        svg {
                            width: "15",
                            height: "19",
                            view_box: "0 0 15 19",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M7.49467 12.3969C7.91045 12.3969 8.26225 12.056 8.26225 11.6513V3.34416L8.1983 2.06613L8.64605 2.55604L9.81876 3.82343C9.95736 3.97254 10.1493 4.04709 10.3305 4.04709C10.7356 4.04709 11.0341 3.77017 11.0341 3.38676C11.0341 3.17377 10.9488 3.02467 10.7996 2.88621L8.04905 0.255589C7.85715 0.0638861 7.69722 0 7.49467 0C7.30277 0 7.14286 0.0638861 6.94029 0.255589L4.18977 2.88621C4.05117 3.02467 3.96589 3.17377 3.96589 3.38676C3.96589 3.77017 4.25372 4.04709 4.65885 4.04709C4.84009 4.04709 5.04264 3.97254 5.18124 3.82343L6.35395 2.55604L6.80171 2.06613L6.73774 3.34416V11.6513C6.73774 12.056 7.08955 12.3969 7.49467 12.3969ZM2.71855 19H12.2814C14.1045 19 15 18.1054 15 16.3161V8.12611C15 6.33688 14.1045 5.44225 12.2814 5.44225H9.98934V6.98654H12.2601C13.0171 6.98654 13.4648 7.4019 13.4648 8.20066V16.2416C13.4648 17.051 13.0171 17.4557 12.2601 17.4557H2.73988C1.97228 17.4557 1.53519 17.051 1.53519 16.2416V8.20066C1.53519 7.4019 1.97228 6.98654 2.73988 6.98654H5.01065V5.44225H2.71855C0.906181 5.44225 0 6.33688 0 8.12611V16.3161C0 18.1054 0.906181 19 2.71855 19Z",
                                fill: "#767676",
                            }
                        }
                    }
                    button {
                        style: "{props.icon_button_style}",
                        onclick: tabs_onclick,
                        onmouseover: tabs_onmouseover,
                        onmouseout: tabs_onmouseout,
                        onfocus: props.tabs_onfocus,
                        onblur: props.tabs_onblur,
                        "aria-label": "Tabs",
                        title: "Tabs",
                        tabindex: "{props.tabs_tabindex}",
                        svg {
                            width: "15",
                            height: "15",
                            view_box: "0 0 15 15",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M7.01662 14.6401C7.4887 14.6401 7.87493 14.2646 7.87493 13.7925V8.3745H13.1642C13.6255 8.3745 14.0225 7.97755 14.0225 7.50547C14.0225 7.03341 13.6255 6.63643 13.1642 6.63643H7.87493V1.20768C7.87493 0.735619 7.4887 0.360107 7.01662 0.360107C6.54456 0.360107 6.14758 0.735619 6.14758 1.20768V6.63643H0.869031C0.396973 6.63643 0 7.03341 0 7.50547C0 7.97755 0.396973 8.3745 0.869031 8.3745H6.14758V13.7925C6.14758 14.2646 6.54456 14.6401 7.01662 14.6401Z",
                                fill: "#767676",
                            }
                        }
                    }
                    button {
                        style: "{props.icon_button_style}",
                        onclick: more_onclick,
                        onmouseover: more_onmouseover,
                        onmouseout: more_onmouseout,
                        onfocus: props.more_onfocus,
                        onblur: props.more_onblur,
                        "aria-label": "More options",
                        title: "More options",
                        tabindex: "{props.more_tabindex}",
                        svg {
                            width: "18",
                            height: "19",
                            view_box: "0 0 18 19",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M2.67776 14.2898H3.97934V15.5914C3.97934 17.3407 4.85401 18.205 6.63458 18.205H14.8189C16.5891 18.205 17.4742 17.3407 17.4742 15.5914V7.32373C17.4742 5.5744 16.5891 4.71016 14.8189 4.71016H13.5174V3.40857C13.5174 1.65923 12.6323 0.794983 10.8621 0.794983H2.67776C0.897191 0.794983 0.022522 1.65923 0.022522 3.40857V11.6762C0.022522 13.4256 0.897191 14.2898 2.67776 14.2898ZM2.69859 12.7904C1.94886 12.7904 1.52195 12.3843 1.52195 11.5929V3.49187C1.52195 2.70051 1.94886 2.29442 2.69859 2.29442H10.8413C11.591 2.29442 12.0179 2.70051 12.0179 3.49187V4.71016H6.63458C4.85401 4.71016 3.97934 5.5744 3.97934 7.32373V12.7904H2.69859ZM6.65539 16.7056C5.90568 16.7056 5.47878 16.2995 5.47878 15.5081V7.40704C5.47878 6.61567 5.90568 6.20957 6.65539 6.20957H14.7981C15.5478 6.20957 15.9747 6.61567 15.9747 7.40704V15.5081C15.9747 16.2995 15.5478 16.7056 14.7981 16.7056H6.65539Z",
                                fill: "#767676",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct KeyboardNavigationOptions {
    pub on_escape: Option<EventHandler<()>>,
    pub on_enter: Option<EventHandler<()>>,
    pub trap_focus: bool,
}

pub fn use_keyboard(options: KeyboardNavigationOptions) -> Signal<Option<Rc<MountedData>>> {
    let container_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    {
        let options = options.clone();

        use_effect(move || {
            let closure = Closure::<dyn Fn(web_sys::KeyboardEvent)>::wrap(Box::new(
                move |event: web_sys::KeyboardEvent| {
                    let key = event.key();
                    let target = event.target();

                    match key.as_str() {
                        "Escape" => {
                            if let Some(callback) = &options.on_escape {
                                event.prevent_default();
                                callback.call(());
                            }
                        }
                        "Enter" => {
                            if let Some(callback) = &options.on_enter {
                                if let Some(_target_elem) = target
                                    .clone()
                                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                {
                                    if let Some(container) = container_ref
                                        .read()
                                        .as_ref()
                                        .and_then(|r| r.downcast::<web_sys::Element>())
                                    {
                                        if target.unwrap() == ***container {
                                            event.prevent_default();
                                            callback.call(());
                                        }
                                    }
                                }
                            }
                        }
                        "Tab" if options.trap_focus => {
                            if let Some(container) = container_ref
                                .read()
                                .as_ref()
                                .and_then(|r| r.downcast::<web_sys::Element>())
                            {
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
                as Box<dyn Fn(web_sys::KeyboardEvent)>);

            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
        });
    }

    container_ref
}
/// Properties for the `BrowserFrame` component.
///
/// This component simulates a web browser window with controls like
/// close, minimize, maximize, and supports rich customization via props.
#[derive(PartialEq, Props, Clone)]
pub struct BrowserFrameProps {
    /// Child elements rendered inside the browser frame body.
    #[props(default)]
    pub children: Element,

    /// The URL displayed in the address bar and used in the iframe.
    #[props(default)]
    pub url: String,

    /// Placeholder text shown in the address input field.
    #[props(default)]
    pub placeholder: &'static str,

    /// Event handler for when the address bar URL changes.
    #[props(default)]
    pub on_url_change: Option<EventHandler<FormEvent>>,

    /// Event handler triggered when the close button is clicked.
    #[props(default)]
    pub on_close: EventHandler<()>,

    /// Event handler triggered when the minimize button is clicked.
    #[props(default)]
    pub on_minimize: EventHandler<()>,

    /// Event handler triggered when the maximize button is clicked.
    #[props(default)]
    pub on_maximize: EventHandler<()>,

    /// Whether to show the top-right control buttons (close, minimize, maximize).
    ///
    /// Defaults to `true`.
    #[props(default = true)]
    pub show_controls: bool,

    /// Whether to show the address bar.
    ///
    /// Defaults to `true`.
    #[props(default = true)]
    pub show_address_bar: bool,

    /// Whether the address bar is read-only.
    ///
    /// Defaults to `false`.
    #[props(default = false)]
    pub read_only: bool,

    /// Size of the browser frame container.
    #[props(default)]
    pub size: Size,

    /// Display variant for the frame (e.g., Tabs, Plain, etc.).
    #[props(default)]
    pub variant: Variant,

    /// Optional list of custom buttons displayed in the top bar.
    #[props(default)]
    pub custom_buttons: Vec<Element>,

    /// CSS class applied to the outermost container.
    ///
    /// Defaults to:
    /// `"rounded-lg border shadow-lg overflow-hidden bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700"`
    #[props(
        default = "rounded-lg border shadow-lg overflow-hidden bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700"
    )]
    pub class: &'static str,

    /// CSS class applied to the browser frame.
    #[props(default)]
    pub frame_class: &'static str,

    /// Inline style string applied to the outermost container.
    #[props(default)]
    pub style: &'static str,

    /// HTML id attribute for the browser container.
    #[props(default)]
    pub id: &'static str,

    /// ARIA label for accessibility.
    ///
    /// Defaults to `"Browser window"`.
    #[props(default = "Browser window")]
    pub aria_label: &'static str,

    /// ARIA description for additional accessibility context.
    #[props(default)]
    pub aria_describedby: &'static str,

    /// Additional CSS class for the address bar container.
    #[props(default)]
    pub container_class: &'static str,

    /// Additional CSS class for the input element in the address bar.
    ///
    /// Defaults to `"text-black dark:text-white"`.
    #[props(default = "text-black dark:text-white")]
    pub input_class: &'static str,

    /// Inline style for the refresh button inside the address bar.
    #[props(
        default = "position: absolute; top: 50%; right: 8px; transform: translateY(-50%); padding: 4px; background: none; border: none; box-shadow: none; outline: none; cursor: pointer;"
    )]
    pub refresh_button_style: &'static str,

    /// ARIA label for the refresh button.
    ///
    /// Defaults to `"Refresh"`.
    #[props(default = "Refresh")]
    pub refresh_button_aria_label: &'static str,

    /// Shared inline style for all icon buttons (close, minimize, maximize).
    #[props(
        default = "padding: 4px; cursor: pointer; background: none; border: none; box-shadow: none; outline: none;"
    )]
    pub icon_button_style: &'static str,

    /// Inline style for the wrapper around the address bar.
    #[props(default = "flex: 1; display: flex; justify-content: center; padding-right: 8px;")]
    pub address_wrapper_base_style: &'static str,

    /// Inline style for the header container (holds address bar and controls).
    #[props(default = "display: flex; align-items: center; position: relative;")]
    pub header_base_style: &'static str,

    // Close button props
    #[props(default)]
    pub on_close_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_close_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_close_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_close_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub close_class: &'static str,
    #[props(default)]
    pub close_svg_class: &'static str,
    #[props(default)]
    pub close_path_class: &'static str,
    #[props(default = "button")]
    pub close_button_type: &'static str,
    #[props(default)]
    pub close_aria_label: &'static str,
    #[props(default)]
    pub close_title: &'static str,
    #[props(default = "0")]
    pub close_tabindex: &'static str,

    // Minimize button props
    #[props(default)]
    pub on_minimize_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_minimize_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_minimize_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_minimize_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub minimize_class: &'static str,
    #[props(default)]
    pub minimize_svg_class: &'static str,
    #[props(default)]
    pub minimize_path_class: &'static str,
    #[props(default = "button")]
    pub minimize_button_type: &'static str,
    #[props(default)]
    pub minimize_aria_label: &'static str,
    #[props(default)]
    pub minimize_title: &'static str,
    #[props(default = "0")]
    pub minimize_tabindex: &'static str,

    // Maximize button props
    #[props(default)]
    pub on_maximize_mouse_over: EventHandler<()>,
    #[props(default)]
    pub on_maximize_mouse_out: EventHandler<()>,
    #[props(default)]
    pub on_maximize_focus: EventHandler<FocusEvent>,
    #[props(default)]
    pub on_maximize_blur: EventHandler<FocusEvent>,
    #[props(default)]
    pub maximize_class: &'static str,
    #[props(default)]
    pub maximize_svg_class: &'static str,
    #[props(default)]
    pub maximize_path_class: &'static str,
    #[props(default = "button")]
    pub maximize_button_type: &'static str,
    #[props(default)]
    pub maximize_aria_label: &'static str,
    #[props(default)]
    pub maximize_title: &'static str,
    #[props(default = "0")]
    pub maximize_tabindex: &'static str,

    // Share button props
    #[props(default)]
    pub share_button_style: &'static str,
    #[props(default)]
    pub share_onclick: EventHandler<()>,
    #[props(default)]
    pub share_onmouseover: EventHandler<()>,
    #[props(default)]
    pub share_onmouseout: EventHandler<()>,
    #[props(default)]
    pub share_onfocus: EventHandler<FocusEvent>,
    #[props(default)]
    pub share_onblur: EventHandler<FocusEvent>,
    #[props(default)]
    pub share_tabindex: &'static str,

    // Tabs button props
    #[props(default)]
    pub tabs_button_style: &'static str,
    #[props(default)]
    pub tabs_onclick: EventHandler<()>,
    #[props(default)]
    pub tabs_onmouseover: EventHandler<()>,
    #[props(default)]
    pub tabs_onmouseout: EventHandler<()>,
    #[props(default)]
    pub tabs_onfocus: EventHandler<FocusEvent>,
    #[props(default)]
    pub tabs_onblur: EventHandler<FocusEvent>,
    #[props(default)]
    pub tabs_tabindex: &'static str,

    // More button props
    #[props(default)]
    pub more_button_style: &'static str,
    #[props(default)]
    pub more_onclick: EventHandler<()>,
    #[props(default)]
    pub more_onmouseover: EventHandler<()>,
    #[props(default)]
    pub more_onmouseout: EventHandler<()>,
    #[props(default)]
    pub more_onfocus: EventHandler<FocusEvent>,
    #[props(default)]
    pub more_onblur: EventHandler<FocusEvent>,
    #[props(default)]
    pub more_tabindex: &'static str,
}

/// BrowserFrame Component
///
/// A Dioxus component that emulates a browser window, complete with customizable controls (close, minimize, maximize),
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
/// use dioxus::prelude::*;
/// use browser_rs::dioxus::BrowserFrame;
///
/// fn app() -> Element {
///     let on_close = Callback::new(|_| log::info!("Browser closed"));
///
///     rsx! {
///         BrowserFrame {
///             url: "https://opensass.org",
///             on_close: on_close,
///             children: rsx! {
///                 p { "Your embedded content here." }
///             }
///         }
///     }
/// }
/// ```
///
/// ## With Custom Buttons
/// ```rust
/// use dioxus::prelude::*;
/// use browser_rs::dioxus::BrowserFrame;
///
/// fn app() -> Element {
///     let custom_button = rsx! {
///         button { "Custom Button" }
///     };
///
///     rsx! {
///         BrowserFrame {
///             url: "https://opensass.org",
///             custom_buttons: vec![custom_button],
///             children: rsx! {
///                 p { "Custom button in the header!" }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling and Class Customization
/// ```rust
/// use dioxus::prelude::*;
/// use browser_rs::dioxus::BrowserFrame;
///
/// fn app() -> Element {
///     rsx! {
///         BrowserFrame {
///             url: "https://opensass.org",
///             class: "rounded-xl shadow-xl",
///             input_class: "bg-gray-200 text-gray-900",
///             container_class: "flex-1 mx-4",
///             children: rsx! {
///                 p { "Styled browser frame!" }
///             }
///         }
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
#[component]
pub fn BrowserFrame(props: BrowserFrameProps) -> Element {
    let on_close = props.on_close;

    let mut container_ref = use_keyboard(KeyboardNavigationOptions {
        on_escape: Some(EventHandler::new(move |_| {
            on_close.call(());
        })),
        on_enter: None,
        trap_focus: false,
    });

    let size_style = props.size.to_style();
    let combined_style = format!("{} {}", size_style, props.style);

    rsx! {
        article {
            id: "{props.id}",
            class: "{props.class}",
            style: "{combined_style}",
            role: "application",
            aria_label: "{props.aria_label}",
            aria_describedby: "{props.aria_describedby}",
            tabindex: "-1",
            onmounted: move |cx| container_ref.set(Some(cx.data())),

            BrowserHeader {
                url: props.url,
                placeholder: props.placeholder,
                on_url_change: props.on_url_change,
                on_close: props.on_close,
                on_minimize: props.on_minimize,
                on_maximize: props.on_maximize,
                show_controls: props.show_controls,
                show_address_bar: props.show_address_bar,
                read_only: props.read_only,
                variant: props.variant,
                size: props.size,
                custom_buttons: props.custom_buttons,
                class: props.frame_class,
                container_class: props.container_class,
                input_class: props.input_class,
                refresh_button_style: props.refresh_button_style,
                refresh_button_aria_label: props.refresh_button_aria_label,
                icon_button_style: props.icon_button_style,
                address_wrapper_base_style: props.address_wrapper_base_style,
                header_base_style: props.header_base_style,
                on_close_mouse_over: props.on_close_mouse_over,
                on_close_mouse_out: props.on_close_mouse_out,
                on_close_focus: props.on_close_focus,
                on_close_blur: props.on_close_blur,
                close_class: props.close_class,
                close_svg_class: props.close_svg_class,
                close_path_class: props.close_path_class,
                close_button_type: props.close_button_type,
                close_aria_label: props.close_aria_label,
                close_title: props.close_title,
                close_tabindex: props.close_tabindex,
                on_minimize_mouse_over: props.on_minimize_mouse_over,
                on_minimize_mouse_out: props.on_minimize_mouse_out,
                on_minimize_focus: props.on_minimize_focus,
                on_minimize_blur: props.on_minimize_blur,
                minimize_class: props.minimize_class,
                minimize_svg_class: props.minimize_svg_class,
                minimize_path_class: props.minimize_path_class,
                minimize_button_type: props.minimize_button_type,
                minimize_aria_label: props.minimize_aria_label,
                minimize_title: props.minimize_title,
                minimize_tabindex: props.minimize_tabindex,
                on_maximize_mouse_over: props.on_maximize_mouse_over,
                on_maximize_mouse_out: props.on_maximize_mouse_out,
                on_maximize_focus: props.on_maximize_focus,
                on_maximize_blur: props.on_maximize_blur,
                maximize_class: props.maximize_class,
                maximize_svg_class: props.maximize_svg_class,
                maximize_path_class: props.maximize_path_class,
                maximize_button_type: props.maximize_button_type,
                maximize_aria_label: props.maximize_aria_label,
                maximize_title: props.maximize_title,
                maximize_tabindex: props.maximize_tabindex,
                share_button_style: props.share_button_style,
                share_onclick: props.share_onclick,
                share_onmouseover: props.share_onmouseover,
                share_onmouseout: props.share_onmouseout,
                share_onfocus: props.share_onfocus,
                share_onblur: props.share_onblur,
                share_tabindex: props.share_tabindex,
                tabs_button_style: props.tabs_button_style,
                tabs_onclick: props.tabs_onclick,
                tabs_onmouseover: props.tabs_onmouseover,
                tabs_onmouseout: props.tabs_onmouseout,
                tabs_onfocus: props.tabs_onfocus,
                tabs_onblur: props.tabs_onblur,
                tabs_tabindex: props.tabs_tabindex,
                more_button_style: props.more_button_style,
                more_onclick: props.more_onclick,
                more_onmouseover: props.more_onmouseover,
                more_onmouseout: props.more_onmouseout,
                more_onfocus: props.more_onfocus,
                more_onblur: props.more_onblur,
                more_tabindex: props.more_tabindex,
            }
            BrowserContent {
                aria_describedby: props.aria_describedby,
                {props.children}
            }
        }
    }
}
