/// Integration tests for PgUserRepo.
/// Requires DATABASE_URL env var pointing to a running postgres.
/// Skipped silently if not set.
use std::sync::Once;

use user_svc::domain::Status;
use user_svc::repository::{PgUserRepo, UserRepo};

static MIGRATE: Once = Once::new();

fn db_url() -> Option<String> {
    std::env::var("DATABASE_URL").ok()
}

fn setup() -> Option<PgUserRepo> {
    let url = db_url()?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let repo = rt.block_on(async {
        let repo = PgUserRepo::new(&url).await.expect("connect");
        MIGRATE.call_once(|| {
            rt.block_on(async { repo.migrate().await.expect("migrate") });
        });
        // Clean up test data
        let _ = sqlx::query("DELETE FROM users WHERE id LIKE 'usr_%'")
            .execute(&repo.pool)
            .await;
        repo
    });
    Some(repo)
}

#[test]
fn pg_create_and_get_user() {
    let repo = match setup() {
        Some(r) => r,
        None => {
            eprintln!("skipping: DATABASE_URL not set");
            return;
        }
    };

    let user = repo.create_user("pcode_bob", "bob").expect("create");
    assert_eq!(user.nickname, "bob");
    assert!(user.id.starts_with("usr_"));
    assert_eq!(user.status, Status::Active);

    let found = repo.get_user(&user.id).expect("get").expect("exists");
    assert_eq!(found.nickname, "bob");
    assert_eq!(found.id, user.id);
}

#[test]
fn pg_get_nonexistent_returns_none() {
    let repo = match setup() {
        Some(r) => r,
        None => return,
    };

    let result = repo.get_user("usr_nonexistent").expect("get");
    assert!(result.is_none());
}

#[test]
fn pg_update_profile() {
    let repo = match setup() {
        Some(r) => r,
        None => return,
    };

    let user = repo.create_user("pcode_carol", "carol").expect("create");
    let profile = repo
        .update_profile(&user.id, "carol2", "https://img.test/avatar.png")
        .expect("update");

    assert_eq!(profile.nickname, "carol2");
    assert_eq!(profile.avatar, "https://img.test/avatar.png");

    // Verify persisted
    let updated = repo.get_user(&user.id).unwrap().unwrap();
    assert_eq!(updated.nickname, "carol2");
}

#[test]
fn pg_update_status() {
    let repo = match setup() {
        Some(r) => r,
        None => return,
    };

    let user = repo.create_user("pcode_dave", "dave").expect("create");
    let updated = repo.update_status(&user.id, Status::Suspended).expect("update");
    assert_eq!(updated.status, Status::Suspended);

    let found = repo.get_user(&user.id).unwrap().unwrap();
    assert_eq!(found.status, Status::Suspended);
}

#[test]
fn pg_create_user_empty_nickname_fails() {
    let repo = match setup() {
        Some(r) => r,
        None => return,
    };

    let result = repo.create_user("pcode_empty", "");
    assert!(result.is_err());
}
