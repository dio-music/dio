use yew::{html, Component};

// Use `wee_alloc` as the global allocator. this will produce a smaller WASM package
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub enum Msg {}

// pub enum TopLevelView {
//     WelcomePage,
//     DataPage,
// }

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        todo!()
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
