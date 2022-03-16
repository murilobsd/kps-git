#![allow(dead_code)]
use std::{fs::File, io::Write};

use git2::{
    Commit, Direction, IndexAddOption, ObjectType, Repository, Signature,
};
use tempdir::TempDir;

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    remote.connect(Direction::Push)?;
    remote.push(&["refs/heads/main:refs/heads/main"], None)
}

fn main() {
    // cria um diretório temporário
    let tmp_dir = TempDir::new("kps").unwrap();
    println!("Temp dir: {:?}", tmp_dir.path());
    // isso converte o tmp_dir para path não removendo a pasta
    // quando é dropado o arquivo ou o programa terminar
    let tmp_path = tmp_dir.into_path();

    // criamos um README.md e adicionamos conteúdo
    let path = tmp_path.join("README.md");
    let mut file = File::create(path).unwrap();
    let content = "# My Readme\n\n##Simple test";
    file.write_all(content.as_bytes()).unwrap();

    let repo = match Repository::init(tmp_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let mut index = repo.index().expect("cannot get the Index file");
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .unwrap();
    // index.write().unwrap();
    let oid = index.write_tree().unwrap();
    let signature = Signature::now("kps", "mr@kps.com.br").unwrap();
    // let parent_commit = find_last_commit(&repo).unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let message = "kps init";
    repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])
        .unwrap(); // parents

    // removemos o diretório com todos os arquivos
    // fs::remove_dir_all(tmp_path).expect("remove temp dir");
    // drop(file);
    // tmp_dir.close().unwrap();
}
