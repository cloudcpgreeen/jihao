use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{Error, OutboxEntry, Profile, Status, User, UserAddress};
use super::UserRepo;

pub struct PgUserRepo {
    pub pool: PgPool,
}

impl PgUserRepo {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    /// Run migrations embedded at compile time.
    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }

    /// Block on the current tokio runtime. Safe because this is always called from within one.
    fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(f))
    }
}

impl UserRepo for PgUserRepo {
    fn create_user(&self, principal_code: &str, nickname: &str) -> Result<User, Error> {
        if nickname.is_empty() {
            return Err(Error::InvalidInput("nickname must not be empty".into()));
        }
        let id = format!("usr_{}", Uuid::new_v4());
        let now = Utc::now();

        self.block_on(async {
            sqlx::query(
                "INSERT INTO users (id, principal_code, nickname, status, created_at, updated_at) VALUES ($1, $2, $3, 'active', $4, $4)",
            )
            .bind(&id)
            .bind(principal_code)
            .bind(nickname)
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::InvalidInput(e.to_string()))?;

            Ok(User {
                id,
                principal_code: principal_code.into(),
                nickname: nickname.into(),
                avatar: String::new(),
                email: String::new(),
                verified_name: None,
                status: Status::Active,
                created_at: now,
                updated_at: now,
            })
        })
    }

    fn get_user(&self, id: &str) -> Result<Option<User>, Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: String,
            principal_code: String,
            nickname: String,
            avatar: String,
            email: String,
            status: String,
            verified_name: Option<String>,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
        }

        self.block_on(async {
            let row: Option<Row> = sqlx::query_as("SELECT id, principal_code, nickname, avatar, email, status, created_at, updated_at FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| Error::InvalidInput(e.to_string()))?;

            Ok(row.map(|r| User {
                id: r.id,
                principal_code: r.principal_code,
                nickname: r.nickname,
                avatar: r.avatar,
                email: r.email,
                verified_name: r.verified_name,
                status: match r.status.as_str() {
                    "inactive" => Status::Inactive,
                    "suspended" => Status::Suspended,
                    _ => Status::Active,
                },
                created_at: r.created_at,
                updated_at: r.updated_at,
            }))
        })
    }

    fn get_profile(&self, id: &str) -> Result<Option<Profile>, Error> {
        self.block_on(async {
            let user = self.get_user(id)?;
            Ok(user.as_ref().map(Profile::from))
        })
    }

    fn update_profile(&self, id: &str, nickname: &str, avatar: &str) -> Result<Profile, Error> {
        if nickname.is_empty() {
            return Err(Error::InvalidInput("nickname must not be empty".into()));
        }

        self.block_on(async {
            let result = sqlx::query(
                "UPDATE users SET nickname = $1, avatar = $2, updated_at = $3 WHERE id = $4",
            )
            .bind(nickname)
            .bind(avatar)
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::InvalidInput(e.to_string()))?;

            if result.rows_affected() == 0 {
                return Err(Error::NotFound);
            }

            let user = self.get_user(id)?.ok_or(Error::NotFound)?;
            Ok(Profile::from(&user))
        })
    }

    fn update_status(&self, id: &str, new_status: Status) -> Result<User, Error> {
        let status_str = match new_status {
            Status::Active => "active",
            Status::Inactive => "inactive",
            Status::Suspended => "suspended",
        };

        self.block_on(async {
            let result = sqlx::query(
                "UPDATE users SET status = $1, updated_at = $2 WHERE id = $3",
            )
            .bind(status_str)
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::InvalidInput(e.to_string()))?;

            if result.rows_affected() == 0 {
                return Err(Error::NotFound);
            }

            self.get_user(id)?.ok_or(Error::NotFound)
        })
    }

    fn mark_verified(&self, id: &str, verified_name: &str) -> Result<(), Error> {
        self.block_on(async {
            let result = sqlx::query(
                "UPDATE users SET verified_name = $1, updated_at = $2 WHERE id = $3",
            )
            .bind(verified_name)
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::InvalidInput(e.to_string()))?;

            if result.rows_affected() == 0 {
                return Err(Error::NotFound);
            }
            Ok(())
        })
    }

    fn save_address(
        &self,
        _user_id: &str,
        _province: &str,
        _city: &str,
        _district: &str,
        _detail: &str,
        _is_default: bool,
    ) -> Result<UserAddress, Error> {
        // ponytail: address table not in PG yet, add when user-facing schema lands
        Err(Error::InvalidInput("pg address not implemented".into()))
    }

    fn get_addresses(&self, _user_id: &str) -> Result<Vec<UserAddress>, Error> {
        Ok(Vec::new())
    }

    fn set_default_address(&self, _user_id: &str, _address_id: &str) -> Result<(), Error> {
        Err(Error::InvalidInput("pg address not implemented".into()))
    }

    fn delete_address(&self, _user_id: &str, _address_id: &str) -> Result<(), Error> {
        Err(Error::InvalidInput("pg address not implemented".into()))
    }

    fn fetch_unpublished_events(&self) -> Result<Vec<OutboxEntry>, Error> {
        // ponytail: outbox still in-memory. Returns empty — outbox isn't in PG yet.
        Ok(Vec::new())
    }

    fn mark_published(&self, _id: &str) -> Result<(), Error> {
        // ponytail: outbox still in-memory, nothing to mark.
        Ok(())
    }
}
