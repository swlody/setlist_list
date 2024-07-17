use axum::{async_trait, Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    prelude::*,
    Error, Result,
};
use minijinja::{path_loader, Environment};
use serde::Serialize;

#[derive(Clone)]
pub struct MiniJinjaView {
    env: Environment<'static>,
}

impl MiniJinjaView {
    fn new() -> Self {
        // TODO statically load templates in production?
        // https://github.com/mitsuhiko/minijinja/blob/main/examples/embedding/src/main.rs

        // TODO Autoreload in development - why does autoreload seem to be working already?
        // Look at https://docs.rs/minijinja-autoreload/latest/minijinja_autoreload/
        let mut env = Environment::new();
        env.set_loader(path_loader("assets/views"));

        Self { env }
    }
}

impl ViewRenderer for MiniJinjaView {
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String> {
        if let Some((key, block)) = key.split_once(':') {
            let tmpl = self
                .env
                .get_template(key)
                .map_err(|e| Error::Anyhow(e.into()))?;

            tmpl.eval_to_state(data)
                .map_err(|e| Error::Anyhow(e.into()))?
                .render_block(block)
                .map_err(|e| Error::Anyhow(e.into()))
        } else {
            let tmpl = self
                .env
                .get_template(key)
                .map_err(|e| Error::Anyhow(e.into()))?;

            tmpl.render(data).map_err(|e| Error::Anyhow(e.into()))
        }
    }
}

pub struct ViewEngineInitializer;
#[async_trait]
impl Initializer for ViewEngineInitializer {
    fn name(&self) -> String {
        "view-engine".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Ok(router.layer(Extension(ViewEngine::from(MiniJinjaView::new()))))
    }
}
