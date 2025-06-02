# 🧬 Browser RS Dioxus Usage

Adding Browser RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **browser-rs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add browser-rs --features=dio
   ```

1. Import the `BrowserFrame` component into your Dioxus application to simulate a browser window with rich customization.

## 🛠️ Usage

Follow these steps to integrate the `BrowserFrame` into your Dioxus application:

### Step 1: Import the Required Components

Import the `BrowserFrame` component and any required Dioxus types:

```rust
use dioxus::prelude::*;
use browser_rs::dioxus::BrowserFrame;
```

### Step 2: Basic Implementation

Use the `BrowserFrame` in your app to render content inside a simulated browser window:

```rust
use dioxus::prelude::*;
use browser_rs::dioxus::BrowserFrame;


fn app() -> Element {
    let on_close = Callback::new(|_| log::info!("Browser closed"));

    rsx! {
        BrowserFrame {
            url: "https://opensass.org",
            on_close: on_close,
            children: rsx! {
                p { "Your embedded content here." }
            }
        }
    }
}
```

### Step 3: Customize the BrowserFrame

You can customize the appearance and behavior of the browser frame using various props:

```rust
use dioxus::prelude::*;
use browser_rs::dioxus::BrowserFrame;


fn app() -> Element {
    let custom_button = rsx! {
        button { "Custom Button" }
    };

    rsx! {
        BrowserFrame {
            url: "https://opensass.org",
            class: "rounded-xl shadow-xl",
            input_class: "bg-gray-200 text-gray-900",
            container_class: "flex-1 mx-4",
            custom_buttons: vec![custom_button],
            on_close: Callback::new(|_| log::info!("Closed")),
            children: rsx! {
                p { "Customized browser frame!" }
            }
        }
    }
}
```

### Step 4: Handling Events and Interactions

Use event handlers to manage user interactions, such as closing or minimizing the browser window:

```rust
use dioxus::prelude::*;
use browser_rs::dioxus::BrowserFrame;


fn app() -> Element {
    rsx! {
        BrowserFrame {
            url: "https://opensass.org",
            on_close: Callback::new(|_| log::info!("Closed")),
            on_minimize: Callback::new(|_| log::info!("Minimized")),
            on_maximize: Callback::new(|_| log::info!("Maximized")),
            children: rsx! {
                p { "Interactive browser frame." }
            }
        }
    }
}
```

## 🔧 Props

### `BrowserFrameProps` Props

#### Main Props

| Property           | Type                              | Description                                                  | Default                        |
| ------------------ | --------------------------------- | ------------------------------------------------------------ | ------------------------------ |
| `children`         | `Element`                         | Child elements rendered inside the browser frame.            | `{}`                           |
| `url`              | `String`                          | The URL displayed in the address bar and used in the iframe. | `""`                           |
| `placeholder`      | `&'static str`                    | Placeholder text shown in the address bar.                   | `""`                           |
| `on_url_change`    | `Option<EventHandler<FormEvent>>` | Event handler for when the address bar URL changes.          | `None`                         |
| `on_close`         | `EventHandler<()>`                | Event handler for when the close button is clicked.          | No-op                          |
| `on_minimize`      | `EventHandler<()>`                | Event handler for when the minimize button is clicked.       | No-op                          |
| `on_maximize`      | `EventHandler<()>`                | Event handler for when the maximize button is clicked.       | No-op                          |
| `show_controls`    | `bool`                            | Whether to show control buttons (close, minimize, maximize). | `true`                         |
| `show_address_bar` | `bool`                            | Whether to show the address bar.                             | `true`                         |
| `read_only`        | `bool`                            | Whether the address bar is read-only.                        | `false`                        |
| `size`             | `Size`                            | Size of the browser frame container.                         | `Size::default()`              |
| `variant`          | `Variant`                         | Display variant for the frame (e.g., Tabs, Plain).           | `Variant::default()`           |
| `custom_buttons`   | `Vec<Element>`                    | Custom buttons displayed in the top bar.                     | `[]`                           |
| `class`            | `&'static str`                    | CSS class for the outermost container.                       | `"rounded-lg..."`              |
| `frame_class`      | `&'static str`                    | CSS class for the browser frame.                             | `""`                           |
| `style`            | `&'static str`                    | Inline styles for the outer container.                       | `""`                           |
| `id`               | `&'static str`                    | HTML id attribute for the browser container.                 | `""`                           |
| `aria_label`       | `&'static str`                    | ARIA label for accessibility.                                | `"Browser window"`             |
| `aria_describedby` | `&'static str`                    | ARIA description for additional accessibility context.       | `""`                           |
| `container_class`  | `&'static str`                    | Additional CSS class for the address bar container.          | `""`                           |
| `input_class`      | `&'static str`                    | CSS class for the address bar input element.                 | `"text-black dark:text-white"` |

#### Behavioral & Style Props

| Property                     | Type           | Description                                                       | Default                      |
| ---------------------------- | -------------- | ----------------------------------------------------------------- | ---------------------------- |
| `refresh_button_style`       | `&'static str` | Inline style for the refresh button.                              | `"position: absolute; ...;"` |
| `refresh_button_aria_label`  | `&'static str` | ARIA label for the refresh button.                                | `"Refresh"`                  |
| `icon_button_style`          | `&'static str` | Shared inline style for icon buttons (close, minimize, maximize). | `"padding: 4px; ...;"`       |
| `address_wrapper_base_style` | `&'static str` | Inline style for the wrapper around the address bar.              | `"flex: 1; ...;"`            |
| `header_base_style`          | `&'static str` | Inline style for the header container (controls and address bar). | `"display: flex; ...;"`      |

#### Control Button Props

Each control button (close, minimize, maximize) has customizable events and styles:

| Property              | Type                       | Description                                              | Default |
| --------------------- | -------------------------- | -------------------------------------------------------- | ------- |
| `on_close_mouse_over` | `EventHandler<()>`         | Mouse over event for close button.                       | No-op   |
| `on_close_mouse_out`  | `EventHandler<()>`         | Mouse out event for close button.                        | No-op   |
| `on_close_focus`      | `EventHandler<FocusEvent>` | Focus event for close button.                            | No-op   |
| `on_close_blur`       | `EventHandler<FocusEvent>` | Blur event for close button.                             | No-op   |
| `close_class`         | `&'static str`             | CSS class for the close button.                          | `""`    |
| ...                   | ...                        | _Similar props exist for minimize and maximize buttons._ |         |

#### Additional Custom Button Props

Props are also available for share, tabs, and more buttons:

| Property             | Type               | Description                                      | Default |
| -------------------- | ------------------ | ------------------------------------------------ | ------- |
| `share_button_style` | `&'static str`     | Inline style for the share button.               | `""`    |
| `share_onclick`      | `EventHandler<()>` | Click event for the share button.                | No-op   |
| ...                  | ...                | _Similar props exist for tabs and more buttons._ |         |

## 💡 Notes

1. **Accessible**: All elements support ARIA labels, roles, and keyboard navigation (`Escape` triggers close).

1. **Dark Mode Ready**: Default styles are compatible with Tailwind's dark theme classes.

1. **Customizable Controls**: All button elements (close, minimize, maximize, refresh, tabs, share, more) support individual style, label, and event customization.

1. **Component Structure**: Internally splits into header and content subcomponents (`BrowserHeader`, `BrowserContent`) for modular control.

1. **Use Anywhere**: Can be used to wrap iframes, widgets, editors, or any arbitrary HTML/RSX content.
