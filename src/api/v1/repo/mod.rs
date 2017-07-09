use std::path::{Path, PathBuf};

use rocket::State;
use git2;
use base64;

use model::{AuthUser, Basket};
use db::Db;
use super::ApiResponse;


// ============================================================================
// repo/tree
// ============================================================================

#[derive(Serialize, Deserialize, FromForm)]
pub struct TreeReq {
    username: String,
    basket: String,
    id: String,
    prefix: String,
}

#[derive(Serialize, Deserialize)]
pub struct TreeEntry {
    id: String,
    filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct TreeResponse {
    entries: Vec<TreeEntry>,
}

#[get("/repo/tree?<msg>")]
pub fn tree(
    msg: TreeReq,
    db: State<Db>,
    auth_user: Option<AuthUser>
) -> ApiResponse<TreeResponse> {
    let basket = match Basket::load(&msg.basket, &msg.username, auth_user.as_ref(), &db) {
        Some(b) => b,
        None => return ApiResponse::NotFound,
    };

    let repo = basket.open_repo();
    let tree_id = match git2::Oid::from_str(&msg.id) {
        Ok(id) => id,
        Err(_) => return ApiResponse::BadRequest {
            msg: "invalid id".into(),
        },
    };

    let tree = match repo.raw().find_tree(tree_id) {
        Ok(c) => c,
        Err(_) => return ApiResponse::InternalServerError,
    };

    let entries = tree.iter().map(|e| {
        TreeEntry {
            id: e.id().to_string(),
            filename: e.name().unwrap().into(),
            // kind: e.kind()
        }
    }).collect();

    ApiResponse::Ok(TreeResponse { entries })
}


// ============================================================================
// repo/commit
// ============================================================================

#[derive(Serialize, Deserialize, FromForm)]
pub struct CommitReq {
    username: String,
    basket: String,
    reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitResponse {
    id: String,
    tree_id: String,
    message: Option<String>,
}

#[get("/repo/commit?<msg>")]
pub fn commit(
    msg: CommitReq,
    db: State<Db>,
    auth_user: Option<AuthUser>
) -> ApiResponse<CommitResponse> {
    let basket = match Basket::load(&msg.basket, &msg.username, auth_user.as_ref(), &db) {
        Some(b) => b,
        None => return ApiResponse::NotFound,
    };

    let repo = basket.open_repo();
    let commit_id = match repo.raw().refname_to_id(&msg.reference) {
        Ok(id) => id,
        Err(_) => return ApiResponse::BadRequest {
            msg: "invalid reference name".into(),
        },
    };

    let commit = match repo.raw().find_commit(commit_id) {
        Ok(c) => c,
        Err(_) => return ApiResponse::BadRequest {
            msg: "reference does not point to a commit".into(),
        },
    };

    ApiResponse::Ok(CommitResponse {
        id: commit.id().to_string(),
        tree_id: commit.tree_id().to_string(),
        message: commit.message().map(|s| s.into()),
    })
}


// ============================================================================
// repo/commit
// ============================================================================

#[derive(Serialize, Deserialize, FromForm)]
pub struct BlobReq {
    username: String,
    basket: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlobResponse {
    data: String,
}

#[get("/repo/blob?<msg>")]
pub fn blob(
    msg: BlobReq,
    db: State<Db>,
    auth_user: Option<AuthUser>
) -> ApiResponse<BlobResponse> {
    let basket = match Basket::load(&msg.basket, &msg.username, auth_user.as_ref(), &db) {
        Some(b) => b,
        None => return ApiResponse::NotFound,
    };

    let repo = basket.open_repo();
    let blob_id = match git2::Oid::from_str(&msg.id) {
        Ok(id) => id,
        Err(_) => return ApiResponse::BadRequest {
            msg: "invalid reference name".into(),
        },
    };

    let blob = match repo.raw().find_blob(blob_id) {
        Ok(b) => b,
        Err(_) => return ApiResponse::BadRequest {
            msg: "oid does not reference a blob".into(),
        },
    };

    ApiResponse::Ok(BlobResponse {
        data: base64::encode(blob.content()),
    })
}


// ============================================================================
// repo/commit
// ============================================================================

#[derive(Serialize, Deserialize, FromForm)]
pub struct TreeEntryReq {
    username: String,
    basket: String,
    commit_ref: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TreeEntryResponse {
    Tree {
        entries: Vec<TreeEntry>,
    },
    Blob {
        content: String,
    }
}

#[get("/repo/tree_entry?<msg>")]
pub fn tree_entry(
    msg: TreeEntryReq,
    db: State<Db>,
    auth_user: Option<AuthUser>
) -> ApiResponse<TreeEntryResponse> {
    let basket = match Basket::load(&msg.basket, &msg.username, auth_user.as_ref(), &db) {
        Some(b) => b,
        None => return ApiResponse::NotFound,
    };

    let repo = basket.open_repo();
    let commit_id = match repo.raw().refname_to_id(&msg.commit_ref) {
        Ok(id) => id,
        Err(_) => return ApiResponse::BadRequest {
            msg: "invalid reference name".into(),
        },
    };

    let commit = match repo.raw().find_commit(commit_id) {
        Ok(c) => c,
        Err(_) => return ApiResponse::BadRequest {
            msg: "reference does not point to a commit".into(),
        },
    };

    let tree = commit.tree().unwrap();
    let entry = if msg.path.is_empty() {
        tree.into_object()
    } else {
        match tree.get_path(Path::new(&msg.path[1..])) {
            Ok(entry) => entry.to_object(repo.raw()).unwrap(),
            Err(_) => return ApiResponse::BadRequest {
                msg: "path not found".into(),
            },
        }
    };

    let resp = match entry.kind().unwrap() {
        git2::ObjectType::Tree => TreeEntryResponse::Tree {
            entries: entry.as_tree().unwrap().iter().map(|e| {
                TreeEntry {
                    id: e.id().to_string(),
                    filename: e.name().unwrap().into(),
                    // kind: e.kind()
                }
            }).collect(),
        },
        git2::ObjectType::Blob => TreeEntryResponse::Blob {
            content: base64::encode(entry.as_blob().unwrap().content()) ,
        },
        _ => return ApiResponse::InternalServerError,
    };

    ApiResponse::Ok(resp)
}
