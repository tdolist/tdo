mod cli;
mod commands;
mod config;
mod display;
mod store;
mod timeparse;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {e:#}", "error:".red().bold());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Set up tracing based on -v / RUST_LOG.
    let filter = match cli.verbose {
        0 => "warn",
        1 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .with_target(false)
        .compact()
        .init();

    let config = config::load()?;
    let mut collection = store::load()?;

    match cli.command {
        // No subcommand: quick-add if a task string was given, otherwise list.
        None => match cli.task {
            Some(ref task) => {
                if task.len() < config.local.min_task_length {
                    anyhow::bail!(
                        "task must be at least {} characters (got {}). \
                         Use `tdo add` to bypass this limit.",
                        config.local.min_task_length,
                        task.len()
                    );
                }
                commands::todo::add(&mut collection, task, None, None, None, &config)?;
            }
            None => {
                display::print_collection(&collection, false);
            }
        },

        Some(cmd) => match cmd {
            Commands::All => {
                display::print_collection(&collection, true);
            }

            Commands::Add {
                task,
                list,
                due,
                remind,
            } => {
                commands::todo::add(
                    &mut collection,
                    &task,
                    list.as_deref(),
                    due.as_deref(),
                    remind.as_deref(),
                    &config,
                )?;
            }

            Commands::Done { id } => {
                let id_map = display::build_id_map(&collection);
                commands::todo::mark_done(&mut collection, id, &id_map)?;
            }

            Commands::Edit { id } => {
                let id_map = display::build_id_map(&collection);
                commands::todo::edit(&mut collection, id, &id_map)?;
            }

            Commands::Due { id, when } => {
                let id_map = display::build_id_map(&collection);
                commands::todo::set_due(&mut collection, id, &when, &id_map)?;
            }

            Commands::Move { id, list } => {
                let id_map = display::build_id_map(&collection);
                commands::todo::move_todo(&mut collection, id, &list, &id_map)?;
            }

            Commands::Newlist { name } => {
                commands::list::newlist(&mut collection, &name)?;
            }

            Commands::Lists => {
                display::print_list_summary(&collection);
            }

            Commands::Clean { list } => {
                commands::todo::clean(&mut collection, list.as_deref())?;
            }

            Commands::Remove { list } => {
                commands::list::remove(&mut collection, &list)?;
            }

            Commands::Export { path, undone } => {
                commands::export::export(&collection, &path, undone)?;
            }

            Commands::Sync { .. } => {
                eprintln!(
                    "{} Server sync is not yet available (coming in Phase 5).",
                    "note:".cyan().bold()
                );
            }
        },
    }

    Ok(())
}
