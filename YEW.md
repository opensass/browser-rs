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

| Property                     | Type                           | Default Value                          | Description                                                      |
| ---------------------------- | ------------------------------ | -------------------------------------- | ---------------------------------------------------------------- |
| `children`                   | `Children`                     | `""`                                   | The child components to render inside the browser frame.         |
| `url`                        | `String`                       | `""`                                   | The current URL displayed in the address bar.                    |
| `placeholder`                | `&'static str`                 | `""`                                   | Placeholder text for the address bar input.                      |
| `on_url_change`              | `Option<Callback<InputEvent>>` | `None`                                 | Triggered when the address bar's URL is edited by the user.      |
| `on_close`                   | `Callback<()>`                 | No-op callback                         | Called when the close button is clicked.                         |
| `on_minimize`                | `Callback<()>`                 | No-op callback                         | Called when the minimize button is clicked.                      |
| `on_maximize`                | `Callback<()>`                 | No-op callback                         | Called when the maximize button is clicked.                      |
| `show_controls`              | `bool`                         | `true`                                 | Whether to show the window controls (close, minimize, maximize). |
| `show_address_bar`           | `bool`                         | `true`                                 | Whether to display the address bar.                              |
| `read_only`                  | `bool`                         | `false`                                | If `true`, the address bar input is read-only.                   |
| `size`                       | `Size`                         | `Medium`                               | Sets the browser frame size (`Small`, `Medium`, `Large`).        |
| `variant`                    | `Variant`                      | `Default`                              | Visual variant of the browser frame.                             |
| `custom_buttons`             | `Vec<Html>`                    | `[]`                                   | Optional custom buttons to render in the header.                 |
| `class`                      | `&'static str`                 | `"rounded-lg border shadow-lg..."`     | Outer container CSS classes.                                     |
| `frame_class`                | `&'static str`                 | `""`                                   | Additional CSS classes for the frame element.                    |
| `style`                      | `&'static str`                 | `""`                                   | Inline styles for the outer container.                           |
| `id`                         | `&'static str`                 | `""`                                   | Optional container ID.                                           |
| `aria_label`                 | `&'static str`                 | `"Browser window"`                     | ARIA label for the browser frame container.                      |
| `aria_describedby`           | `&'static str`                 | `""`                                   | ARIA description for the browser frame.                          |
| `container_class`            | `&'static str`                 | `""`                                   | CSS classes for the address bar container.                       |
| `input_class`                | `&'static str`                 | `"text-black dark:text-white"`         | CSS classes for the address input element.                       |
| `refresh_button_style`       | `&'static str`                 | `"position: absolute; ..."`            | Inline styles for the refresh button.                            |
| `refresh_button_aria_label`  | `&'static str`                 | `"Refresh"`                            | ARIA label for the refresh button.                               |
| `icon_button_style`          | `&'static str`                 | `"padding: 4px; cursor: pointer; ..."` | Inline styles for icon buttons (close, minimize, maximize).      |
| `address_wrapper_base_style` | `&'static str`                 | `"flex: 1; display: ..."`              | Style for the address bar wrapper.                               |
| `header_base_style`          | `&'static str`                 | `"display: flex; align-items: ..."`    | Style for the header container.                                  |

#### Close button (`close_*`)

| Property              | Type                   | Default    | Description                                        |
| --------------------- | ---------------------- | ---------- | -------------------------------------------------- |
| `on_close_mouse_over` | `Callback<()>`         | No-op      | Called on mouse over the close button.             |
| `on_close_mouse_out`  | `Callback<()>`         | No-op      | Called on mouse out of the close button.           |
| `on_close_focus`      | `Callback<FocusEvent>` | No-op      | Called when the close button gains focus.          |
| `on_close_blur`       | `Callback<FocusEvent>` | No-op      | Called when the close button loses focus.          |
| `close_class`         | `&'static str`         | `""`       | CSS class for the close button.                    |
| `close_svg_class`     | `&'static str`         | `""`       | CSS class for the close button's SVG element.      |
| `close_path_class`    | `&'static str`         | `""`       | CSS class for the close button's SVG path.         |
| `close_button_type`   | `&'static str`         | `"button"` | The `type` attribute for the close button element. |
| `close_aria_label`    | `&'static str`         | `""`       | ARIA label for the close button.                   |
| `close_title`         | `&'static str`         | `""`       | Title attribute for the close button.              |
| `close_tabindex`      | `&'static str`         | `"0"`      | Tab index for keyboard navigation.                 |

#### Minimize button (`minimize_*`)

_(Same structure as above)_

#### Maximize button (`maximize_*`)

_(Same structure as above)_

#### Share button (`share_*`)

| Property             | Type                   | Default | Description                         |
| -------------------- | ---------------------- | ------- | ----------------------------------- |
| `share_button_style` | `&'static str`         | `""`    | Inline styles for the share button. |
| `share_onclick`      | `Callback<()>`         | No-op   | Called on click.                    |
| `share_onmouseover`  | `Callback<()>`         | No-op   | Called on mouse over.               |
| `share_onmouseout`   | `Callback<()>`         | No-op   | Called on mouse out.                |
| `share_onfocus`      | `Callback<FocusEvent>` | No-op   | Called on focus.                    |
| `share_onblur`       | `Callback<FocusEvent>` | No-op   | Called on blur.                     |
| `share_tabindex`     | `&'static str`         | `""`    | Tab index for accessibility.        |

#### Tabs button (`tabs_*`)

_(Same structure as share button)_

#### More button (`more_*`)

_(Same structure as share button)_

## 💡 Notes

1. **Accessible**: All elements support ARIA labels, roles, and keyboard navigation (`Escape` triggers close).

1. **Dark Mode Ready**: Default styles are compatible with Tailwind's dark theme classes.

1. **Customizable Controls**: All button elements (close, minimize, maximize, refresh, tabs, share, more) support individual style, label, and event customization.

1. **Component Structure**: Internally splits into header and content subcomponents (`BrowserHeader`, `BrowserContent`) for modular control.

1. **Use Anywhere**: Can be used to wrap iframes, widgets, editors, or any arbitrary HTML/Yew content.
