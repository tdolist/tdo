use clap::{App, Arg, AppSettings, SubCommand, Shell};

pub fn cli() -> App<'static, 'static> {
    App::new("tdo")
        .version(crate_version!())
        .author("Felix Wittwer <dev@felixwittwer.de>, Felix Döring <development@felixdoering.com>")
        .about("A todo list tool for the terminal")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("task")
            .help("Task you want to add")
            .takes_value(true))
        .arg(Arg::with_name("list")
            .help("List you want to add the task to")
            .takes_value(true))
        .subcommand(SubCommand::with_name("all").about("Lists all tasks."))
        .subcommand(SubCommand::with_name("add")
            .about("Add a task to a certain list or the default list.")
            .arg(Arg::with_name("task")
                .help("Task you want to add")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("list")
                .help("List you want to add the task to")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("edit")
            .about("Edit a task description.")
            .arg(Arg::with_name("id")
                .help("ID of the task you want to edit")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("done")
            .about("Mark a task as 'done'.")
            .arg(Arg::with_name("id")
                .help("ID of the task you want to mark ad done")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("move")
            .about("Move a todo to another list.")
            .arg(Arg::with_name("id")
                .help("ID of the task you want to move")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("listname")
                .help("Name of the new list the todo should be moved to")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("github")
            .about("Generate a new todo and a github issue")
            .setting(AppSettings::ArgsNegateSubcommands)
            .setting(AppSettings::SubcommandsNegateReqs)
            .arg(Arg::with_name("repository")
                .help("<username>/<repo> you want to create the issue")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("title")
                .help("The issue title")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("body")
                .help("Content of the issue")
                .takes_value(true))
            .subcommand(SubCommand::with_name("set")
                .about("Set the GitHub Access Token")
                .arg(Arg::with_name("token")
                    .help("GitHub Access Token")
                    .takes_value(true)))
            .subcommand(SubCommand::with_name("update")
                .about("Update all created Issues")))
        .subcommand(SubCommand::with_name("newlist")
            .about("Create a new todo list.")
            .arg(Arg::with_name("listname")
                .help("Name of the new list")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("lists")
            .about("Display done/undone statistics of all lists."))
        .subcommand(SubCommand::with_name("clean")
            .about("Removes all tasks that have been marked as done.")
            .arg(Arg::with_name("listname")
                .help("Clears only the given list")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("remove")
            .about("Remove a todo list and its contents.")
            .arg(Arg::with_name("listname")
                .help("Name of the list to be deleted")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("export")
            .about("Export your todos as markdown.")
            .arg(Arg::with_name("destination")
                .help("Location to export the markdown")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("undone").help("Export only undone tasks (default exports all)")))
        .subcommand(SubCommand::with_name("reset")
            .about("DANGER ZONE. Deletes all your todos and todo lists."))
        .subcommand(SubCommand::with_name("completions")
            .about("Generate completion scripts for your shell.")
            .after_help(COMPLETION_HELP)
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::with_name("shell").possible_values(&Shell::variants())))

}


//
static COMPLETION_HELP: &'static str =
r"NOTICE:
    One can generate a completion script for `tdo` that is
    compatible with a given shell. The script is output on `stdout`
    allowing one to re-direct the output to the file of their
    choosing. Where you place the file will depend on which shell, and
    which operating system you are using. Your particular
    configuration may also determine where these scripts need to be
    placed.
    Here are some common set ups for the three supported shells under
    Unix and similar operating systems (such as GNU/Linux).
    BASH:
    Completion files are commonly stored in `/etc/bash_completion.d/`
    Run the command:
    `tdo completions bash > /etc/bash_completion.d/tdo.bash-completion`
    This installs the completion script. You may have to log out and
    log back in to your shell session for the changes to take affect.
    BASH (macOS/Homebrew):
    Homebrew stores bash completion files within the Homebrew directory.
    With the `bash-completion` brew formula installed, run the command:
    `tdo completions bash > $(brew --prefix)/etc/bash_completion.d/tdo.bash-completion`
    FISH:
    Fish completion files are commonly stored in
    `$HOME/.config/fish/completions`
    Run the command:
    `tdo completions fish > ~/.config/fish/completions/tdo.fish`
    This installs the completion script. You may have to log out and
    log back in to your shell session for the changes to take affect.
    ZSH:
    ZSH completions are commonly stored in any directory listed in
    your `$fpath` variable. To use these completions, you must either
    add the generated script to one of those directories, or add your
    own to this list.
    Adding a custom directory is often the safest best if you're
    unsure of which directory to use. First create the directory, for
    this example we'll create a hidden directory inside our `$HOME`
    directory
    `mkdir ~/.zfunc`
    Then add the following lines to your `.zshrc` just before
    `compinit`
    `fpath+=~/.zfunc`
    Now you can install the completions script using the following
    command
    `tdo completions zsh > ~/.zfunc/_tdo`
    You must then either log out and log back in, or simply run
    `exec zsh`
    For the new completions to take affect.
    CUSTOM LOCATIONS:
    Alternatively, you could save these files to the place of your
    choosing, such as a custom directory inside your $HOME. Doing so
    will require you to add the proper directives, such as `source`ing
    inside your login script. Consult your shells documentation for
    how to add such directives.
    POWERSHELL:
    The powershell completion scripts require PowerShell v5.0+ (which
    comes Windows 10, but can be downloaded separately for windows 7
    or 8.1).
    First, check if a profile has already been set
    `PS C:\> Test-Path $profile`
    If the above command returns `False` run the following
    `PS C:\> New-Item -path $profile -type file -force`
    Now open the file provided by `$profile` (if you used the
    `New-Item` command it will be
    `%USERPROFILE%\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`
    Next, we either save the completions file into our profile, or
    into a separate file and source it inside our profile. To save the
    completions into our profile simply use
    `PS C:\> tdo completions powershell >> %USERPROFILE%\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`";
