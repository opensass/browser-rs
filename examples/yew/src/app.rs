use theme::yew::ThemeProvider;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
          <ThemeProvider>
              <Switch<Route> render={switch} />
          </ThemeProvider>
      </BrowserRouter>
    }
}
