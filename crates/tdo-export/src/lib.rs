//! The export module for Tdo server.
#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]

#[macro_use]
extern crate prettytable;

use colored::*;
use prettytable::Table;
use prettytable::format;
use tdo_core::error::*;
use std::{slice, io, ptr};
use std::collections::HashMap;

#[repr(C)]
struct winsize {
    ws_row: libc::c_ushort, /* rows, in characters */
    ws_col: libc::c_ushort, /* columns, in characters */
    ws_xpixel: libc::c_ushort, /* horizontal size, pixels */
    ws_ypixel: libc::c_ushort, /* vertical size, pixels */
}

/// Generates a well formated String of all undone Todos
pub fn gen_tasks_mail(tdo: &tdo_core::tdo::Tdo) -> Option<String> {
    let mut listed = String::new();
    for list in tdo.to_owned().lists.into_iter() {
        let undone = list.list_undone();
        if undone.len() > 0 {
            listed.push_str("\n------------------------------------------------------------\n\t");
            listed.push_str(&list.name);
            listed.push_str("\n------------------------------------------------------------\n");
            for entry in undone {
                if entry.done {
                    listed.push_str(&format!("- {:?}\n", entry.name));
                }
            }
            listed.push_str("\n\n");
        }
    }
    match listed.len() {
        0 => None,
        _ => Some(listed),
    }
}

/// Generates a markdown String to export the lists.
pub fn gen_tasks_md(tdo: &tdo_core::tdo::Tdo, list_done: bool) -> Option<String> {
    let mut markdown = String::from("# Your tasks\n\n");
    let name = get_full_name();
    if name.is_ok() {
        markdown.push_str(&format!("Here are the tasks for {}\n\n", &name.unwrap()));
    }
    let mut intern = String::new();
    for list in tdo.to_owned().lists.iter() {
        let tasks: Vec<tdo_core::todo::Todo>;
        if list_done {
            tasks = list.list.to_owned();
        } else {
            tasks = list.list_undone();
        }
        if tasks.len() > 0 {
            intern.push_str(&format!("\n### {}\n", &list.name));
            for entry in tasks {
                if entry.done {
                    intern.push_str(&format!("- [x] {}\n", &entry.name));
                } else {
                    intern.push_str(&format!("- [ ] {}\n", &entry.name));
                }
            }
        } else if list_done {
            intern.push_str(&format!("\n### {}\n", &list.name));
        }
    }
    match intern.len() {
        0 => None,
        _ => {
            markdown.push_str(&intern);
            Some(markdown)
        }
    }
}

/// Returns the formated output for the terminal printout.
pub fn render_terminal_output(tdo: &tdo_core::tdo::Tdo, all: bool) {
    let width = match get_winsize() {
        Ok((x, _)) => {
            if x <= 9 + tdo.get_highest_id().to_string().len() {
                println!("{} Terminalsize is too small.", "error:".red().bold());
                std::process::exit(1);
            }
            x
        }
        Err(_) => {
            println!("{} Terminalsize could not be fetched.",
                     "error:".red().bold());
            std::process::exit(1);
        }
    };
    let mut table = Table::new();
    for list in tdo.lists.to_owned().iter() {
        let tasks: Vec<tdo_core::todo::Todo>;
        if all {
            tasks = list.list.to_owned();
        } else {
            tasks = list.list_undone();
        }
        table.add_row(row![bc->"###", "", b->&list.name]);
        if tasks.len() > 0 {
            for entry in tasks {
                let reformated = reformat_task(&entry.name,
                                               width - 9 - tdo.get_highest_id().to_string().len());
                if entry.done {
                    table.add_row(row![c->"[x]", r->entry.id, reformated]);
                } else {
                    table.add_row(row![c->"[ ]", r->entry.id, reformated]);
                }
            }
        }
        table.add_row(row![""]);
    }
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.printstd();
}

/// Creates a new issue for the given repository and returns succes or failure.
pub fn github_issue(tdo: &mut tdo_core::tdo::Tdo,
                    repo: &str,
                    issue_text: &str,
                    body: Option<&str>)
                    -> TdoResult<tdo_core::todo::GitHub> {
    let mut issue = HashMap::new();
    issue.insert("title", issue_text);
    if body.is_some() {
        issue.insert("body", body.unwrap());
    }
    let api_token = match tdo.get_gh_token() {
        Some(token) => token,
        None => {
            tdo.set_gh_token(None);
            tdo.get_gh_token().unwrap()
        }
    };
    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("https://api.github.com/repos/{}/issues", repo))
        .bearer_auth(api_token)
        .json(&issue)
        .send();

    match res {
        Ok(content) => {
            match content.status() {
                reqwest::StatusCode::CREATED => {
                    let response: tdo_core::todo::GHIssueResponse = content.json().unwrap();
                    Ok(tdo_core::todo::GitHub::new(repo, response.number))
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    Err(ErrorKind::GithubError(github_error::ErrorKind::BadCredentials).into())
                }
                _ => Err(ErrorKind::GithubError(github_error::ErrorKind::UnknownError).into()),
            }
        }
        Err(_) => Err(ErrorKind::GithubError(github_error::ErrorKind::UnknownError).into()),
    }
}

/// Updates the status of a github issue todo
pub fn update_github_issue(old_todo: &tdo_core::todo::Todo, api_token: &str) -> TdoResult<tdo_core::todo::Todo> {
    let github = old_todo.clone().github.unwrap();
    let mut todo = old_todo.clone();
    let repo = github.repo.as_str();
    let number = github.issue_number;
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("https://api.github.com/repos/{}/issues/{}", repo, number))
        .bearer_auth(api_token)
        .send();
    match res {
        Ok(content) => {
            match content.status() {
                reqwest::StatusCode::OK => {
                    let response: tdo_core::todo::GHIssueResponse = content.json().unwrap();
                    if todo.name != response.title {
                        todo.edit(&response.title);
                    }
                    match response.state.as_str() {
                        "closed" => todo.set_done(),
                        "open" => todo.set_undone(),
                        _ => return Err(ErrorKind::GithubError(github_error::ErrorKind::UnknownError).into()),
                    }
                }
                reqwest::StatusCode::NOT_FOUND => {
                    return Err(ErrorKind::GithubError(github_error::ErrorKind::DoesNotExist).into())
                }
                _ => {
                    return Err(ErrorKind::GithubError(github_error::ErrorKind::UnknownError).into())
                }
            }
        }
        Err(_) => return Err(ErrorKind::GithubError(github_error::ErrorKind::UnknownError).into()),
    }

    Ok(todo)
}


#[allow(unsafe_code)]
fn get_full_name() -> io::Result<String> {
    unsafe {
        let uid = libc::geteuid();
        let user = ptr::read(libc::getpwuid(uid));
        let res = String::from_utf8_unchecked(slice::from_raw_parts(user.pw_gecos as *const u8,
                                                                    libc::strlen(user.pw_gecos))
            .to_vec());
        let results: Vec<&str> = res.split(",").collect();
        let name = results[0].to_string();
        if name == "" {
            Err(io::Error::last_os_error())
        } else {

            Ok(name)
        }
    }
}

#[allow(unsafe_code)]
fn get_winsize() -> io::Result<(usize, usize)> {
    let w = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &w) };

    match r {
        0 => Ok((w.ws_col as usize, w.ws_row as usize)),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "No valid data.")),
    }
}

fn reformat_task(task_str: &str, size: usize) -> String {
    let mut task: String = String::new();
    let mut temp_vec: Vec<&str> = task_str.split_whitespace().collect();
    while temp_vec.len() > 0 {
        let mut temp_str = String::new();
        if temp_vec[0].len() >= size {
            let mut entr_str = temp_vec.remove(0).to_string();
            let mut tmp_str: String;
            while entr_str.len() >= size {
                tmp_str = entr_str.split_off(size);
                task.push_str(format!("{}\n", entr_str).as_str());
                entr_str = tmp_str;
            }
            if temp_vec.len() == 0 {
                task.push_str(entr_str.as_str());
                return task;
            }
            temp_str.push_str(format!("{} ", entr_str).as_str());
        }
        while temp_str.len() + temp_vec[0].len() < size {
            temp_str.push_str(temp_vec.remove(0));
            temp_str.push_str(" ");
            if temp_vec.len() == 0 {
                task.push_str(temp_str.as_str());
                return task;
            }
        }
        task.push_str(format!("{}\n", temp_str).as_str());
    }
    task
}
