use yew::prelude::*;

pub struct About;

impl Component for About  {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <div>
                <h1>
                {"This is About  Page"}
                </h1>
                <a href="/">{"Home"}</a>
            </div>
        }
    }
}
