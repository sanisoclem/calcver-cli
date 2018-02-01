use libcalcver;
use git2::{Repository};
use std::collections::HashMap;
use repo;

pub struct GitRepo {
    path: String,
    last_tag: Option<String>,
    commits_since_last_tag: Vec<String>
}

impl libcalcver::repository::Repository for GitRepo {
    fn get_last_tag(&self) -> Option<&str> {
        match &self.last_tag {
            &Some(ref tag) => Some(tag),
            &None=>None
        }
    }
    fn get_commits_since_last_tag(&self) -> &Vec<String> {
        &self.commits_since_last_tag
    }
}

impl repo::CodeRepository for GitRepo {
    fn commit(&self, _tag: &str) {
        Repository::open(&self.path).unwrap();
        //panic!("not implemented");
    }
    fn from(path: &str) -> GitRepo {
        let r = Repository::open(&path).unwrap();
        let tags = r.tag_names(Some("*")).unwrap();
        
        let tag_map: HashMap<_,_> = tags.iter().filter_map(|t| {
        match t {
            Some(name) => {
                if let Ok(obj) = r.revparse_single(name) {
                    if let Some(tag) = obj.as_tag() {
                        if let Ok(commit) = tag.peel() {
                            Some((commit.id(),String::from(name)))
                        } else {None}
                    } else if let Some(commit) = obj.as_commit() {
                        Some((commit.id(),String::from(name)))
                    } else { None }
                } else { None }
            },
            _ => None
        }
    }).collect();

        let mut revwalk = r.revwalk().unwrap();
        let mut tag:Option<String> = None;
        let mut commits:Vec<String> = vec![];
        revwalk.push_head().unwrap();

        for c in revwalk {
            let commit =r.find_commit(c.unwrap()).unwrap();
            if let Some(tg) = tag_map.get(&commit.id()) {
                tag = Some(tg.to_owned());
                break;
            }
            if let Some(msg) = commit.message() {
                commits.push(msg.to_string());
            }
        }

        GitRepo {
            path: path.to_owned(),
            last_tag: tag,
            commits_since_last_tag: commits
        }
    }
}