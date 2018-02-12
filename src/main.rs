#[macro_use]
extern crate yew;

use yew::prelude::*;

type Context = ();

enum Scene {
    Home,
    Settings,
}

enum Msg {
    SetScene(Scene),
}

struct Model {
    scene: Scene,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Self {
            scene: Scene::Home
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetScene(scene) => self.scene = scene,
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div class="container-fluid",>
                <div class="row",>
                    <div class="col",>
                        <nav class="navbar navbar-expand-lg navbar-light bg-light",>
                            <a class="navbar-brand", href="#", onclick=|_| Msg::SetScene(Scene::Home),>{ "Home" }</a>
                            <button class="navbar-toggler", type="button", data-toggle="collapse", data-target="#navbar", aria-controls="navbar", aria-expanded="false", aria-label="Toggle navigation",>
                                <span class="navbar-toggler-icon",></span>
                            </button>
                            <div class="collapse navbar-collapse", id="navbar",>
                                <ul class="navbar-nav mr-auto",>
                                    <li class="nav-item",>
                                        <a class="nav-link", href="#", onclick=|_| Msg::SetScene(Scene::Home),>{"Home"}</a>
                                    </li>
                                    <li class="nav-item",>
                                        <a class="nav-link", href="#", onclick=|_| Msg::SetScene(Scene::Settings),>{"Settings"}</a>
                                    </li>
                                </ul>
                            </div>
                        </nav>
                    </div>
                </div>
                <main class="row",>
                    <div class="col",>{
                        match self.scene {
                            Scene::Home => self.home(),
                            Scene::Settings => self.settings(),
                        }
                    }</div>
                </main>
            </div>
        }
    }
}

impl Model {
    fn home(&self) -> Html<Context, Self> {
        html! {
            <p>{"Welcome home."}</p>
        }
    }

    fn settings(&self) -> Html<Context, Self> {
        html! {
            <p>{"Settings."}</p>
        }
    }
}

pub fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
