use yew::prelude::*;

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                <div>
                     <h1>
                          { "Page not found" }  
                     </h1>
                      <a href="/">{"Home"}</a>
                       <h2>{"Pages HERE"}</h2>
                  <a href="/about">{"about"}</a>
            </div>
        }
    }
}