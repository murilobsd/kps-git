use std::{env, path::Path};

use git2::{
    Cred, Direction, IndexAddOption, RemoteCallbacks, Repository, Signature,
};

fn print_push_ref_updates(
    refname: &str,
    failmsg: Option<&str>,
) -> Result<(), git2::Error> {
    match failmsg {
        None => println!("[updated]:  {}", refname),
        Some(msg) => println!("[error]:    {} ({})", refname, msg),
    };
    Ok(())
}

fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!(
                 "{}/.ssh/id_rsa",
                env::var("HOME").unwrap()
            )),
            None,
        )
    });
    callbacks.push_update_reference(print_push_ref_updates);

    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    let mut remote_conn =
        remote.connect_auth(Direction::Push, Some(callbacks), None)?;
    let remote = remote_conn.remote();
    remote.push(&["refs/heads/master:refs/heads/master"], None)
}

fn first_commit(repo: &Repository) -> Result<(), git2::Error> {
    // Commit
    let mut index = repo.index().expect("cannot get the Index file");
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .unwrap();
    let oid = index.write_tree().unwrap();
    let signature = Signature::now("kps", "kps@m0x.ru").unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let message = "kps init";
    repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;

    Ok(())
}

fn init_repo<P: AsRef<Path>>(path: P) -> Repository {
    // Init
    let repo = match Repository::init(&path) {
        Ok(repo) => repo,
        Err(_) => {
            let repo = match Repository::open(path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {}", e),
            };
            repo
        }
    };
    repo
}

fn main() {
    let repo_root = std::env::args().nth(1).expect("expect repo dir");
    let url = std::env::args().nth(2).expect("expect repo url");

    // init repo
    let repo = init_repo(repo_root);
    // commit
    first_commit(&repo).unwrap();
    // push
    push(&repo, &url).unwrap();
}
