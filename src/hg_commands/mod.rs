use egui::{text::LayoutJob, TextFormat};
use std::process::Command;

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
        .arg("cd ".to_owned() + repo + " & hg branch")
        .output()
        .expect("failed to execute command");

    println!("{:?}", text);

    let text = cmd
        .arg(os_arg)
        .arg("hg branch")
        .output()
        .expect("failed to execute command");

    println!("{:?}", text);

    format!("{:?}", text)
}

pub fn hg_status(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {
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
                cmd.current_dir(repo);

                let text = cmd
                    .arg(os_arg)
                    .arg("hg outgoing -n -q")
                    .output()
                    .expect("failed to execute command");

                job.append(&format!("{:?}", text), 0f32, TextFormat::default());

                let text = cmd
                    .arg(os_arg)
                    .arg("hg status")
                    .output()
                    .expect("failed to execute command");

                job.append(&format!("{:?}", text), 0f32, TextFormat::default());
            }
        }
        None => (),
    }
}

pub fn hg_pull(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

pub fn hg_push(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

pub fn hg_switch(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}

pub fn hg_purge(repo_list: &Option<Vec<String>>, job: &mut LayoutJob) {}
