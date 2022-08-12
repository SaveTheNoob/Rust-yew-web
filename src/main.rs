use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use pages::{pagenotfound::PageNotFound, home::Home, about::About,};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}


struct App {}

fn switch(routes: &Route) -> Html {
    return match routes.clone() {
        Route::Home => {
            html! { <Home/> }
        }
        Route::NotFound => {
            html! { <PageNotFound/> }
        }
        Route::About => {
            html! { <About/>}
        }
    }
}



impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
         Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        };
    }
}


fn main() {
    yew::start_app::<App>();
}