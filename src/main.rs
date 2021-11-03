use yew::prelude::*;
use yew_router::prelude::*;
mod pages;
use pages::home::Home;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Home,
}

struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <Router<AppRoute> render={Router::render(switch)} />
            </>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! {<Home />},
    }
}

fn main() {
    yew::start_app::<Model>();
}
