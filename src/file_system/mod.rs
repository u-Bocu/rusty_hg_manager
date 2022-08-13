use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    ops::Deref,
};
use walkdir::WalkDir;

pub fn find_repo_list(project_folder: String) -> Result<Option<Vec<String>>, Error> {
    let mut repo_list: Vec<String> = Vec::new();

    for entry in WalkDir::new(project_folder)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name == ".hg" {
            repo_list.push(entry.path().to_string_lossy().deref().to_string());
        }

        if f_name == "projects.conf" {
            repo_list.clear();

            let file_path: String = entry.path().to_string_lossy().deref().to_string();

            let input = File::open(file_path)?;
            let buffered = BufReader::new(input);

            for line in buffered.lines() {
                let line_content: String = line.unwrap();

                if !line_content.starts_with("#") {
                    let repo_path = entry
                        .path()
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_string_lossy()
                        .deref()
                        .to_string()
                        + "\\"
                        + &line_content;

                    repo_list.push(repo_path);
                }
            }
            break;
        }
    }

    println!("{:?}", repo_list);

    if repo_list.is_empty() {
        Ok(None)
    } else {
        Ok(Some(repo_list))
    }
}
