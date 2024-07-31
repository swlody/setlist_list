use std::{path::PathBuf, sync::Arc};

use axum::{async_trait, Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    prelude::*,
    Error, Result,
};
use minijinja::{path_loader, Environment};
#[cfg(debug_assertions)]
use minijinja_autoreload::AutoReloader;
use serde::Serialize;

#[derive(Clone)]
pub struct MiniJinjaView {
    #[cfg(debug_assertions)]
    reloader: Arc<AutoReloader>,
    #[cfg(not(debug_assertions))]
    env: Arc<Environment<'static>>,
}

impl MiniJinjaView {
    #[cfg(debug_assertions)]
    fn new() -> Self {
        let reloader = AutoReloader::new(move |notifier| {
            let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/views");
            let mut env = Environment::new();
            env.set_loader(path_loader(&template_path));
            notifier.watch_path(&template_path, true);
            notifier.set_fast_reload(true);
            Ok(env)
        });

        Self {
            reloader: Arc::new(reloader),
        }
    }

    #[cfg(not(debug_assertions))]
    fn new() -> Self {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/views");
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));
        Self { env: Arc::new(env) }
    }
}

fn render<S: Serialize>(env: &Environment<'static>, key: &str, data: S) -> Result<String> {
    if let Some((key, block)) = key.split_once(':') {
        let tmpl = env.get_template(key).map_err(|e| Error::Anyhow(e.into()))?;

        tmpl.eval_to_state(data)
            .map_err(|e| Error::Anyhow(e.into()))?
            .render_block(block)
            .map_err(|e| Error::Anyhow(e.into()))
    } else {
        let tmpl = env.get_template(key).map_err(|e| Error::Anyhow(e.into()))?;

        tmpl.render(data).map_err(|e| Error::Anyhow(e.into()))
    }
}

impl ViewRenderer for MiniJinjaView {
    #[cfg(debug_assertions)]
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String> {
        let env = self
            .reloader
            .acquire_env()
            .map_err(|e| Error::Anyhow(e.into()))?;

        render(&env, key, data)
    }

    #[cfg(not(debug_assertions))]
    fn render<S: Serialize>(&self, key: &str, data: S) -> Result<String> {
        render(&self.env, key, data)
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
