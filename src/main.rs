use lazy_regex::regex_captures;
use std::process::Command;

fn main() {
    let args = std::env::args();
    let args: Vec<_> = args.collect();
    let filename = &args[1];
    let line_num: u64 = args[2].parse().unwrap();
    let repo = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .expect("failed to execute process")
        .stdout;
    let repo = String::from_utf8(repo).unwrap();
    let repo = repo.trim();
    let repo = repo.split_once(".git").unwrap().0;
    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("failed to execute process")
        .stdout;
    let branch = String::from_utf8(branch).unwrap();
    let branch = branch.trim();
    let RepoData { org, repo } = RepoData::from_url(repo);

    let link = format!("https://github.com/{org}/{repo}/blob/{branch}/{filename}#L{line_num}");
    println!("{link}")
}

struct RepoData<'a> {
    org: &'a str,
    repo: &'a str,
}

impl<'a> RepoData<'a> {
    fn from_url(repo_url: &'a str) -> Self {
        let (_whole, org, repo) =
            regex_captures!("https://github.com/([^/]+)/([^/]+)", repo_url).unwrap();
        Self { org, repo }
    }
}
