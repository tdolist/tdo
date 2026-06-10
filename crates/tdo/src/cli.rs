//! CLI argument definitions using clap v4 derive.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "tdo",
    about = "A todo list tool for the terminal",
    version,
    // Positional task arg and subcommands are mutually exclusive.
    args_conflicts_with_subcommands = true,
)]
pub struct Cli {
    /// Quick-add a task to the default list (must be ≥ min_task_length chars).
    pub task: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Increase log verbosity (-v debug, -vv trace).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all tasks including completed ones.
    All,

    /// Add a task to a list.
    Add {
        /// Task description.
        task: String,
        /// Target list (uses default list if omitted).
        #[arg(short, long)]
        list: Option<String>,
        /// Due date/time (e.g. +1d, 2025-06-15, tomorrow).
        #[arg(long)]
        due: Option<String>,
        /// Reminder time (e.g. +2h, 2025-06-15T08:00:00Z).
        #[arg(long)]
        remind: Option<String>,
    },

    /// Mark a task as done.
    Done {
        /// Display index of the task.
        id: u32,
    },

    /// Edit the title of a task.
    Edit {
        /// Display index of the task.
        id: u32,
    },

    /// Set or update the due date/time of a task.
    Due {
        /// Display index of the task.
        id: u32,
        /// New due date/time (e.g. +1d, 2025-06-15, tomorrow). Pass "clear" to remove.
        when: String,
    },

    /// Move a task to a different list.
    Move {
        /// Display index of the task.
        id: u32,
        /// Name of the target list.
        list: String,
    },

    /// Create a new todo list.
    Newlist {
        /// Name for the new list.
        name: String,
    },

    /// Show done/undone statistics for all lists.
    Lists,

    /// Remove all completed tasks.
    Clean {
        /// Only clean the named list (cleans all lists if omitted).
        list: Option<String>,
    },

    /// Delete a list and all its todos.
    Remove {
        /// Name of the list to delete.
        list: String,
    },

    /// Export todos as Markdown.
    Export {
        /// Output file path.
        path: PathBuf,
        /// Export only undone tasks.
        #[arg(short, long)]
        undone: bool,
    },

    /// Sync with a tdo server (Phase 5 — coming soon).
    #[command(hide = true)]
    Sync {
        #[command(subcommand)]
        command: Option<SyncCommands>,
    },
}

#[derive(Subcommand)]
pub enum SyncCommands {
    /// Authenticate against a server and save the API key.
    Login,
    /// Remove saved server credentials.
    Logout,
    /// Synchronise local todos with the server.
    Push,
    /// Show differences between local and server state.
    Status,
}
