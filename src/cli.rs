//! command-line interface for running various tasks and commands
//! related to the application. It allows developers to interact with the
//! application via the command line.
//!
//! # Example
//!
//! ```rust,ignore
//! use myapp::app::App;
//! use loco_rs::cli;
//! use migration::Migrator;
//!
//! #[tokio::main]
//! async fn main() {
//!     cli::main::<App, Migrator>().await
//! }
//! ```
use clap::{Parser, Subcommand};

use crate::{
    app::{AppContext, Hooks},
    boot::{create_app, create_context, list_endpoints, start, ServeParams, StartMode},
    environment::{resolve_from_env, Environment, DEFAULT_ENVIRONMENT},
    logger,
};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Playground {
    /// Specify the environment
    #[arg(short, long, global = true, help = &format!("Specify the environment [default: {}]", DEFAULT_ENVIRONMENT))]
    environment: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Specify the environment
    #[arg(short, long, global = true, help = &format!("Specify the environment [default: {}]", DEFAULT_ENVIRONMENT))]
    environment: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an app
    Start {
        /// start worker
        #[arg(short, long, action)]
        worker: bool,
        /// start same-process server and worker
        #[arg(short, long, action)]
        server_and_worker: bool,
        /// server bind address
        #[arg(short, long, action)]
        binding: Option<String>,
        /// server port address
        #[arg(short, long, action)]
        port: Option<i32>,
    },
    /// Describe all application endpoints
    Routes {},
    #[cfg(feature = "with-db")]
    /// Display the app version
    Version {},
}

#[cfg(feature = "with-db")]
/// run playgroup code
///
/// # Errors
///
/// When could not create app context
pub async fn playground<H: Hooks>() -> crate::Result<AppContext> {
    let cli = Playground::parse();
    let environment: Environment = cli.environment.unwrap_or_else(resolve_from_env).into();

    let app_context = create_context::<H>(&environment).await?;
    Ok(app_context)
}

/// # Main CLI Function
///
/// The `main` function is the entry point for the command-line interface (CLI)
/// of the application. It parses command-line arguments, interprets the
/// specified commands, and performs corresponding actions. This function is
/// generic over `H` and `M`, where `H` represents the application hooks and `M`
/// represents the migrator trait for handling database migrations.
///
/// # Errors
///
/// Returns an any error indicating success or failure during the CLI execution.
///
/// # Example
///
/// ```rust,ignore
/// use myapp::app::App;
/// use loco_rs::cli;
/// use migration::Migrator;
///
/// #[tokio::main]
/// async fn main()  {
///     cli::main::<App, Migrator>().await
/// }
/// ```
#[cfg(feature = "with-db")]
pub async fn main<H: Hooks>() -> eyre::Result<()> {
    let cli: Cli = Cli::parse();
    let environment: Environment = cli.environment.unwrap_or_else(resolve_from_env).into();

    let config = environment.load()?;

    if !H::init_logger(&config, &environment)? {
        logger::init::<H>(&config.logger);
    }

    let task_span = create_root_span(&environment);
    let _guard = task_span.enter();

    match cli.command {
        Commands::Start {
            worker,
            server_and_worker,
            binding,
            port,
        } => {
            let start_mode = if worker {
                StartMode::WorkerOnly
            } else if server_and_worker {
                StartMode::ServerAndWorker
            } else {
                StartMode::ServerOnly
            };

            let boot_result = create_app::<H>(start_mode, &environment).await?;
            let serve_params = ServeParams {
                port: port.map_or(boot_result.app_context.config.server.port, |p| p),
                binding: binding
                    .unwrap_or_else(|| boot_result.app_context.config.server.binding.to_string()),
            };
            start::<H>(boot_result, serve_params).await?;
        }
        Commands::Routes {} => {
            let app_context = create_context::<H>(&environment).await?;
            show_list_endpoints::<H>(&app_context);
        }
        Commands::Version {} => {
            println!("{}", H::app_version(),);
        }
    }
    Ok(())
}

#[cfg(not(feature = "with-db"))]
pub async fn main<H: Hooks>() -> eyre::Result<()> {
    let cli = Cli::parse();
    let environment: Environment = cli.environment.unwrap_or_else(resolve_from_env).into();

    let config = environment.load()?;

    if !H::init_logger(&config, &environment)? {
        logger::init::<H>(&config.logger);
    }

    let task_span = create_root_span(&environment);
    let _guard = task_span.enter();

    match cli.command {
        Commands::Start {
            worker,
            server_and_worker,
            binding,
            port,
        } => {
            let start_mode = if worker {
                StartMode::WorkerOnly
            } else if server_and_worker {
                StartMode::ServerAndWorker
            } else {
                StartMode::ServerOnly
            };

            let boot_result = create_app::<H>(start_mode, &environment).await?;
            let serve_params = ServeParams {
                port: port.map_or(boot_result.app_context.config.server.port, |p| p),
                binding: binding.map_or(
                    boot_result.app_context.config.server.binding.to_string(),
                    |b| b,
                ),
            };
            start::<H>(boot_result, serve_params).await?;
        }
        Commands::Routes {} => {
            let app_context = create_context::<H>(&environment).await?;
            show_list_endpoints::<H>(&app_context)
        }
        Commands::Task { name, params } => {
            let vars = task::Vars::from_cli_args(params);
            let app_context = create_context::<H>(&environment).await?;
            run_task::<H>(&app_context, name.as_ref(), &vars).await?;
        }
        Commands::Generate { component } => {
            gen::generate::<H>(component.into(), &config)?;
        }
        Commands::Version {} => {
            println!("{}", H::app_version(),);
        }
    }
    Ok(())
}

fn show_list_endpoints<H: Hooks>(ctx: &AppContext) {
    let mut routes = list_endpoints::<H>(ctx);
    routes.sort_by(|a, b| a.uri.cmp(&b.uri));
    for router in routes {
        println!("{router}");
    }
}

fn create_root_span(environment: &Environment) -> tracing::Span {
    tracing::span!(tracing::Level::DEBUG, "app", environment = %environment)
}
