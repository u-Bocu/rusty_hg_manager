use console::Term;
use egui::text::LayoutJob;
use std::process::Command;
use std::str;
use std::thread;

pub fn hg_branch(repo: &String) -> String {
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    };

    let os_arg = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };

    let text = cmd
        .arg(os_arg)
        .arg("cd /D ".to_owned() + repo + " & hg branch")
        .output()
        .expect("failed to execute command");

    println!("{:?}", text);

    String::from_utf8(text.stdout).unwrap()
}

pub fn hg_status(repo_list: &Option<Vec<String>>) {
    let repo_list = repo_list.clone();
    thread::spawn(move || match repo_list {
        Some(repo_list) => {
            let os_arg = if cfg!(target_os = "windows") {
                "/C"
            } else {
                "-c"
            };

            let term = Term::stdout();

            for repo in repo_list {
                let mut cmd = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                } else {
                    Command::new("sh")
                };

                let output = cmd
                    .arg(os_arg)
                    .arg("cd /D ".to_owned() + &repo + " & hg outgoing")
                    .output()
                    .expect("failed to execute command");

                // Send output to console
                term.write_line(
                    str::from_utf8(output.stdout.as_slice()).expect("Failed to convert line."),
                )
                .expect("Failed to write line.");

                let output = cmd
                    .arg(os_arg)
                    .arg("cd /D ".to_owned() + &repo + " & hg status")
                    .output()
                    .expect("failed to execute command.");

                // Send output to console
                term.write_line(
                    str::from_utf8(output.stdout.as_slice()).expect("Failed to convert line."),
                )
                .expect("Failed to write line.");
            }
        }
        None => (),
    });
}

#[allow(dead_code, unused_variables)]
pub fn hg_pull(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

#[allow(dead_code, unused_variables)]
pub fn hg_push(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

#[allow(dead_code, unused_variables)]
pub fn hg_switch(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

#[allow(dead_code, unused_variables)]
pub fn hg_purge(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}
