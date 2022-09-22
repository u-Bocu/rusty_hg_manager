use egui::{text::LayoutJob, TextFormat};
use std::process::Command;
use std::str;
use std::thread;
use std::sync::mpsc;

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

pub fn hg_status(repo_list: &Option<Vec<String>>, tx: mpsc::Sender<String>) {
    let repo_list = repo_list.clone();
    thread::spawn(move || {
        match repo_list {
            Some(repo_list) => {
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


                for repo in repo_list {
                    let text = cmd
                        .arg(os_arg)
                        .arg("cd /D ".to_owned() + &repo + " & hg outgoing -n")
                        .output()
                        .expect("failed to execute command");

                    tx.send(str::from_utf8(&text.stdout).unwrap().to_owned())
                        .unwrap();

                    let text = cmd
                        .arg(os_arg)
                        .arg("cd /D ".to_owned() + &repo + " & hg status")
                        .output()
                        .expect("failed to execute command");

                    tx.send(str::from_utf8(&text.stdout).unwrap().to_owned())
                        .unwrap();
                }
            }
            None => (),
        }
    });
}

pub fn hg_pull(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

pub fn hg_push(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

pub fn hg_switch(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

pub fn hg_purge(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}
