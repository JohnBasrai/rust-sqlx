use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
struct User
{
    pub id:   i64,
    pub name: String,
    pub role: UserRole,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole
{
    Admin,
    User,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>
{
    // Create a connection pool
    println!("----- 1");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/postgres")
        .await?;

    let num = rand::thread_rng().gen_range(0 .. u16::MAX);
    let user_name = format!("Attila The Hun:{}", num);

    // Insert a new user
    println!("----- 2");
    let record = sqlx::query!(
        r#"INSERT INTO users (name,role) values ($1, $2) returning id"#,
        user_name,
        UserRole::Admin as UserRole
    )
    .fetch_one(&pool)
    .await?;

    // Update User to change a value
    println!("----- 3: {:?}", record);
    let new_name = format!("Attila the long dead Hun:{:?}", record.id);
    let _ = sqlx::query_as!(
        User,
        r#"UPDATE users SET name = $2, role = $3 WHERE id=$1;"#,
        record.id, new_name, UserRole::User as UserRole)
        .fetch_all(&pool)
        .await?;

    // Select all Users
    println!("----- 5");
    let result = sqlx::query_as!(User, r#"SELECT id, name, role as "role: _" FROM users"#)
        .fetch_all(&pool)
        .await?;

    let str_result = result
        .iter()
        .map(|r| format!("\n   {:?}", r))
        .collect::<Vec<String>>()
        .join(", ");
    println!("\n== SELECT users with sqlx::query_as!:\n{str_result}");

    // Select query with map() (build the User manually)
    println!("----- 6");
    let select_query = sqlx::query("SELECT id, name, role FROM users");
    let users: Vec<User> = select_query
        .map(|row: PgRow| User {
            id:   row.get("id"),
            name: row.get("name"),
            role: row.get("role"),
        })
        .fetch_all(&pool)
        .await?;

    println!("\n=== SELECT users with query.map..:\n{:?}", users);

    // Select query_as (using derive FromRow)
    println!("----- 7");
    let select_query = sqlx::query_as::<_, User>("SELECT id, name, role FROM users");
    let users: Vec<User> = select_query.fetch_all(&pool).await?;
    println!("\n=== SELECT users with query.map..:\n{:?}", users);

    // Delete the row just added if row > 10
    if record.id > 10
    {
        println!("----- 8");
        sqlx::query!(r#"DELETE FROM users WHERE id = $1;"#, record.id)
            .execute(&pool)
            .await?;
    }
    Ok(())
}
