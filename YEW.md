# Browser RS Yew Usage

Adding Browser RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Browser RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add browser-rs --features=yew
   ```

1. Import the `BrowserFrame` component into your Yew component and start using it in your app.

## 🛠️ Usage

Follow these steps to integrate `BrowserFrame` into your Yew application:

### Import the Required Component

```rust
use yew::prelude::*;
use browser_rs::yew::BrowserFrame;
```

### Basic Example

Wrap any content inside the `BrowserFrame` and customize its behavior with props:

```rust
use yew::prelude::*;
use browser_rs::yew::BrowserFrame;

#[function_component(App)]
pub fn app() -> Html {
    let on_close = Callback::from(|_| log::info!("Browser closed"));

    html! {
        <BrowserFrame
            url={"https://opensass.org".to_string()}
            on_close={on_close}
        >
            <p>{ "Your embedded content here." }</p>
        </BrowserFrame>
    }
}
```

### Add Custom Buttons

You can include custom buttons in the header using the `custom_buttons` prop:

```rust
use yew::prelude::*;
use browser_rs::yew::BrowserFrame;

#[function_component(App)]
pub fn app() -> Html {
    let custom_button = html! {
        <button>{ "Custom Button" }</button>
    };

    html! {
        <BrowserFrame
            url={"https://opensass.org".to_string()}
            custom_buttons={vec![custom_button]}
        >
            <p>{ "Custom button in the header!" }</p>
        </BrowserFrame>
    }
}
```

### Customize Styling

Override default styles and classes to match your app's design:

```rust
use yew::prelude::*;
use browser_rs::yew::BrowserFrame;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserFrame
            url={"https://opensass.org".to_string()}
            class={"rounded-xl shadow-xl"}
            input_class={"bg-gray-200 text-gray-900"}
            container_class={"flex-1 mx-4"}
        >
            <p>{ "Styled browser frame!" }</p>
        </BrowserFrame>
    }
}
```

## 🔧 Props

### `BrowserFrameProps`

| Property                              | Type                           | Description                                    | Default            |
| ------------------------------------- | ------------------------------ | ---------------------------------------------- | ------------------ |
| `children`                            | `Children`                     | Content inside the browser frame               | `""`               |
| `url`                                 | `String`                       | The address bar's displayed URL                | `""`               |
| `placeholder`                         | `&'static str`                 | Address bar placeholder                        | `""`               |
| `on_url_change`                       | `Option<Callback<InputEvent>>` | Triggered when user edits the address bar      | `None`             |
| `on_close`                            | `Callback<()>`                 | Called when close button is clicked            | `noop`             |
| `on_minimize`                         | `Callback<()>`                 | Called when minimize button is clicked         | `noop`             |
| `on_maximize`                         | `Callback<()>`                 | Called when maximize button is clicked         | `noop`             |
| `show_controls`                       | `bool`                         | Toggles window controls                        | `true`             |
| `show_address_bar`                    | `bool`                         | Toggles visibility of the address bar          | `true`             |
| `read_only`                           | `bool`                         | Makes the address bar read-only                | `false`            |
| `size`                                | `Size`                         | Sets browser frame size                        | `Medium`           |
| `variant`                             | `Variant`                      | Sets visual variant                            | `Default`          |
| `custom_buttons`                      | `Vec<Html>`                    | Custom buttons rendered in the header          | `[]`               |
| `class`                               | `&'static str`                 | Outer container classes                        | See source         |
| `style`                               | `&'static str`                 | Outer container inline style                   | `""`               |
| `id`                                  | `&'static str`                 | Optional container ID                          | `""`               |
| `aria_label`                          | `&'static str`                 | ARIA label for accessibility                   | `"Browser window"` |
| `aria_describedby`                    | `&'static str`                 | ARIA description                               | `""`               |
| `container_class`                     | `&'static str`                 | Address bar container classes                  | See source         |
| `input_class`                         | `&'static str`                 | Address input classes                          | See source         |
| `refresh_button_style`                | `&'static str`                 | Inline style for refresh button                | See source         |
| `refresh_button_aria_label`           | `&'static str`                 | ARIA label for refresh button                  | `"Refresh"`        |
| `icon_button_style`                   | `&'static str`                 | Icon button inline style                       | See source         |
| `address_wrapper_base_style`          | `&'static str`                 | Address bar wrapper style                      | See source         |
| `header_base_style`                   | `&'static str`                 | Header wrapper style                           | See source         |
| `close_*`, `minimize_*`, `maximize_*` | Various                        | Complete control over all icon buttons         | See source         |
| `share_*`, `tabs_*`, `more_*`         | Various                        | Control optional share, tabs, and menu buttons | See source         |

## 💡 Notes

1. **Accessible**: All elements support ARIA labels, roles, and keyboard navigation (`Escape` triggers close).

1. **Dark Mode Ready**: Default styles are compatible with Tailwind's dark theme classes.

1. **Customizable Controls**: All button elements (close, minimize, maximize, refresh, tabs, share, more) support individual style, label, and event customization.

1. **Component Structure**: Internally splits into header and content subcomponents (`BrowserHeader`, `BrowserContent`) for modular control.

1. **Use Anywhere**: Can be used to wrap iframes, widgets, editors, or any arbitrary HTML/Yew content.
