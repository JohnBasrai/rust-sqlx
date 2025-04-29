# rust-sqlx

Async PostgreSQL database access in Rust using SQLx and Tokio.  This project demonstrates
how to insert and update PostgreSQL enum columns using Rust enums and `sqlx`.

At the time of writing, clear examples for updating enum fields were hard to find. This
demo pulls together the correct patterns in a working example. Hopefully it saves others
some time and frustration.

***

## Running

Before running this demo with PostgreSQL (the primary target database), you'll need to
start a local PostgreSQL instance.

The quickest way is to use the official Docker container image:

```bash
docker run --rm --network=host --name=postgres \
  -e POSTGRES_PASSWORD=welcome \
  -e POSTGRES_USER=postgres \
  -p 5432:5432 \
  postgres
```

Then, from another shell window, exec into the database container and initialize the schema:

```bash
docker exec -i postgres psql --username=postgres < db-init.sql
```

Back in the first shell window, set the `DATABASE_URL` environment variable and run the application:

```bash
export DATABASE_URL=postgres://postgres@localhost:5432/postgres
cargo run
```

You’ll see records being added. Once the table exceeds 10 rows, the program will begin
deleting one row for each new insert.

### Sample Output

```text
----- 1
----- 2
----- 3: Record { id: 1 }
----- 5

== SELECT users with sqlx::query_as!:

   User { id: 1, name: "Attila the long dead Hun:1", role: User }
----- 6

=== SELECT users with query.map..:
[User { id: 1, name: "Attila the long dead Hun:1", role: User }]
----- 7

=== SELECT users with query.map..:
[User { id: 1, name: "Attila the long dead Hun:1", role: User }]
```

If you run again you will see more rows added to the table. When the row count is greater
than 10 it will delete 1 row for each row added.

