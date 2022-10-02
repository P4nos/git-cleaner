use std::fs;
use std::io::{self, Write};
use std::process::{exit, Command, ExitStatus};
use std::str;

fn get_git_branches() -> String {
    let output = match Command::new("git").arg("branch").output() {
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
        Ok(output) => output,
    };

    if output.status.code() > Some(0) {
        io::stderr().write_all(&output.stderr).unwrap();
        exit(1);
    }
    return String::from_utf8(output.stdout).unwrap();
}
fn delete_git_branch(branch_name: &str) {
    match Command::new("git")
        .arg("branch")
        .arg("-D")
        .arg(branch_name)
        .output()
    {
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
        Ok(output) => output,
    };
}

fn write_content_to_file(filename: &str, content: &str) {
    fs::write(filename, content).unwrap();
}

fn read_file(filename: &str) -> String {
    let content = fs::read(filename).unwrap();
    return String::from_utf8(content).unwrap();
}

fn open_file_in_editor(filename: &str) -> ExitStatus {
    return Command::new("vi")
        .arg(filename)
        .status()
        .expect("could not open file");
}

fn main() {
    let output_str = get_git_branches();
    // convert command output to string
    let branches: Vec<&str> = output_str.split("\n").collect();

    //create temp file with the content of the above string on /tmp/.git_cleaner
    let filename = "/tmp/.git-cleaner";
    write_content_to_file(filename, &output_str);

    // open file in default editor
    open_file_in_editor(&filename);

    // read file contents after editing
    let file_content = read_file(&filename);

    let branches_to_keep: Vec<&str> = file_content.split("\n").collect();
    for b in branches {
        // don't delete current branch
        if b.contains("*") {
            continue;
        }
        if !branches_to_keep.contains(&b) {
            delete_git_branch(b.trim());
        }
    }
    // remove tmp
    match Command::new("rm").arg(filename).output() {
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
        Ok(output) => output,
    };
}
