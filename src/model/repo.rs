use std::path::{Path, PathBuf};

use git2::{Config, Repository};

use model::Basket;


const REPO_PATH: &str = "repos";

pub struct Repo<'a> {
    repo: Repository,
    basket: &'a Basket,
}

impl<'a> Repo<'a> {
    pub fn open(basket: &'a Basket) -> Self {
        let path = Self::path(basket);
        let repo = Repository::open(path).unwrap();

        Self { repo, basket }
    }

    pub fn create(basket: &'a Basket) -> Self {
        // Create bare repository
        let p = Self::path(basket);
        let repo = Repository::init_bare(&p).unwrap();

        // Add author's data to the config of this repo
        let mut config = Config::open(&p.join("config")).unwrap();
        config.set_str("user.name", basket.owner.username()).unwrap();
        config.set_str("user.email", "dummy@todo.soon").unwrap(); // TODO

        // Create an empty commit
        {
            // This should return the correct signature with the data we just set
            // in the config.
            let sig = repo.signature().unwrap();

            // Create an empty tree object, write it to the ODB, and obtain
            // a handle to the tree.
            let tree_id = repo.treebuilder(None).unwrap().write().unwrap();
            let tree = repo.find_tree(tree_id).unwrap();

            // Commit the tree we just created
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Initial commit",
                &tree,
                // No parents for the first commit
                &[]
            ).unwrap();
        }

        Self { repo, basket }
    }

    pub fn raw(&self) -> &Repository {
        &self.repo
    }

    pub fn debug(&self) {
        // println!("{:?}", self.repo.index().unwrap().path());
    }

    fn path(basket: &Basket) -> PathBuf {
        Path::new(REPO_PATH)
            .join(basket.owner.username())
            .join(basket.name())
    }
}
