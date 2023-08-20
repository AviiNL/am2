use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::*;

use super::RepositoryResult;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn do_something(&self) -> RepositoryResult<String> {
        // do something async

        // show a list of tables in the database
        let tables = sqlx::query!(
            r#"
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public'
            ORDER BY table_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // serialize the list of tables to a string
        let tables = tables
            .into_iter()
            .filter_map(|table| table.table_name)
            .collect::<Vec<_>>()
            .join(", ");

        Ok(tables)
    }
}

impl UserRepository {
    pub async fn all(&self) -> RepositoryResult<Vec<User>> {
        todo!()
        // let users = sqlx::query_as!(
        //     crate::User,
        //     r#"
        //     SELECT
        //         id AS "id: Uuid",
        //         name,
        //         email,
        //         verified,
        //         password,
        //         roles,
        //         created_at,
        //         updated_at
        //     FROM users
        //     "#
        // )
        // .fetch_all(&self.pool)
        // .await?;

        // Ok(users)
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> RepositoryResult<Option<User>> {
        todo!()
        // let user = sqlx::query_as!(
        //     crate::model::User,
        //     r#"
        //     SELECT
        //         id AS "id: Uuid",
        //         name,
        //         email,
        //         verified,
        //         password,
        //         roles AS "roles: Roles",
        //         created_at,
        //         updated_at
        //     FROM users WHERE id = ?
        //     "#,
        //     id
        // )
        // .fetch_optional(&self.pool)
        // .await?;

        // Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        todo!()
        // let user = sqlx::query_as!(
        //     crate::model::User,
        //     r#"
        //     SELECT
        //         id AS "id: Uuid",
        //         name,
        //         email,
        //         verified,
        //         password,
        //         roles AS "roles: Roles",
        //         created_at,
        //         updated_at
        //     FROM users WHERE email = ?
        //     "#,
        //     email
        // )
        // .fetch_optional(&self.pool)
        // .await?;

        // Ok(user)
    }

    pub async fn exists(&self, email: &str) -> RepositoryResult<bool> {
        todo!()
        // let exists = sqlx::query!(
        //     r#"
        //     SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) AS "exists!: bool"
        //     "#,
        //     email
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(exists.exists)
    }
}

impl UserRepository {
    // create
    pub async fn create(&self, user: &User) -> RepositoryResult<User> {
        todo!()
        // let salt = SaltString::generate(&mut OsRng);
        // let hashed_password = Argon2::default()
        //     .hash_password(user.password.as_bytes(), &salt)
        //     .map_err(|_| "Failed to hash password")?;

        // let hashed_password = hashed_password.to_string();

        // let roles = vec!["ROLE_USER"];

        // let id = Uuid::new_v4();

        // let user = sqlx::query_as!(
        //     User,
        //     r#"
        //     INSERT INTO users (id, name, email, password, roles)
        //     VALUES ($1, $2, $3, $4, $5)
        //     RETURNING
        //         id AS "id: Uuid",
        //         name,
        //         email,
        //         verified,
        //         password,
        //         roles AS "roles: Roles",
        //         created_at,
        //         updated_at
        //     "#,
        //     id,
        //     user.name,
        //     user.email,
        //     hashed_password,
        //     roles
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(user)
    }
}

impl UserRepository {
    // update password
    pub async fn update(&self, id: Uuid, body: &User) -> RepositoryResult<User> {
        todo!()
        // let mut query = QueryBuilder::new(
        //     r#"
        //     UPDATE users
        //     SET "#,
        // );

        // query.push("name = ").push_bind(body.name.clone());
        // query.push(", email = ").push_bind(body.email.clone());

        // if let Some(password) = &body.password {
        //     let salt = SaltString::generate(&mut OsRng);
        //     let hashed_password = Argon2::default()
        //         .hash_password(password.as_bytes(), &salt)
        //         .map_err(|_| "Failed to hash password")?;

        //     let hashed_password = hashed_password.to_string();

        //     query.push(", password = ").push_bind(hashed_password);
        // }

        // if let Some(verified) = &body.verified {
        //     query.push(", verified = ").push_bind(verified);
        // }

        // query.push(" WHERE id = ").push_bind(id);

        // query.push(r#" RETURNING id, name, email, verified, password, roles, created_at, updated_at "#);

        // Ok(query.build_query_as::<User>().fetch_one(&self.pool).await?)
    }
}
