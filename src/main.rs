
extern crate getopts;
use getopts::Options;
use std::env;
use std::process::Command;

// construct options
fn options() -> Options {
    let mut opts = Options::new();
    opts.optopt("d", "dir", "Take the pages from the given directory. (default: public)", "<dir>");
    opts.optopt("m", "message", "Use the given <msg> as the commit message. (default: Update documentation)", "<msg>");
    opts.optopt("b", "branch", "Name of the branch to write to. (default: code-pages]", "<branch>");
    opts.optflag("p", "push", "Push the branch to origin/<branch> after committing. (default: false)");
    opts.optflag("h", "help", "Print this help message.");
    opts
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] <dir>", program);
    println!("{}", opts.usage(&brief));
}

fn check_repo() -> bool {
    Command::new("git")
            .arg("rev-parse")
            .status()
            .expect("Unknown Git error")
            .success()
}

fn try_rebase(remote: &str, branch: &str) -> bool {
    let output =  Command::new("git")
            .args(&["rev-list", "--max-count=1",
                 &format!("{}/{}", remote, branch)[..]])
            .output().unwrap();

    if !output.status.success() {
        let msg = String::from_utf8(output.stderr).unwrap();
        println!("stdout:{}",msg);
        return false;
    }

    let rev = String::from_utf8(output.stdout).unwrap();
    println!("rev:{}", rev);

    Command::new("git")
            .args(&["update-ref", 
                &format!("refs/heads/{}",branch)[..], &rev[..]])
            .status()
            .unwrap()
            .success()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let opts = options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => { panic!(f.to_string())}
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let page_dir = matches.opt_str("d").unwrap_or(String::from("public"));
    let commit_msg = matches.opt_str("m").unwrap_or(String::from("Update documentation"));
    let branch = matches.opt_str("b").unwrap_or(String::from("code-pages"));
    let push = matches.opt_present("p");

    if !check_repo() {
        println!("check repo failed.");
        return
    }

    if !try_rebase("origin", &branch[..]) {
        println!("rebase failed.");
        return 
    }



    println!("page_dir:{}, commit_msg:{}, branch:{}, push:{}.", page_dir, commit_msg, branch, push);
    
}
