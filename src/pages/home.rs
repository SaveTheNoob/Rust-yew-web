use yew::prelude::*;

pub struct Home;

impl Component for Home {
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
            <div class="banner">
            <div class="navbar">
                    <img src="imgs/Ase.png" class="logo"/> 
                <ul>
                    <li><a href="/">{"Home"}</a></li>
                    <li><a href="about">{"About"}</a></li>
                    <li><a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">{"Click Me"}</a></li>
                    
                </ul>
            </div>
                <div class="content">
                    <h1>{"Ase"}</h1>
                    <h3>
                     {"SaveTheNoob"}
                    </h3>
                    <div>   
                        <a href="https://github.com/SaveTheNoob" target="_blank">
                            <button type="button"><span></span>{"Github"}</button>
                        </a>
                    </div>
                </div>

        </div>
        }
    }
}
