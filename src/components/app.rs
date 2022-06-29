use crate::components::KeygroupCreator;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    html! {
    <>
        <section class="hero is-primary">
            <div class="hero-body has-text-centered">
                <p class="title">{"MPC Valet"}</p>
                <pre>{"▣ ▣ ▣ ▣\n▣ ▣ ▣ ▣\n▣ ▣ ▣ ▣\n▣ ▣ ▣ ▣"}
                </pre>
            </div>
        </section>
        <KeygroupCreator />
        <footer class="footer">
            <div class="content has-text-centered">
                <p>
                    <strong>{"MPC Valet"}</strong> {" v"}{VERSION}{". "}
                    {"Made by "}<a href="https://plule.github.io">{"plule"}</a>{" with "}<a href="https://yew.rs">{"yew."}</a>
                </p>
                <p>
                    <a href="https://github.com/plule/mpc_valet">{"Source code."}</a>
                </p>
            </div>
        </footer>
    </>
    }
}
