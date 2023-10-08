# rust-sqlx
Example of rust-sqlx/postgres usage with database enum types. There are not many examples of how to
use rust-sqlx with PostgreSQL to insert/update PostgreSQL ENUM columns, so I wrote one myself.

***
## Running
Before running this code codes against PostgreSQL (other database supported by sqlx should also work
with changes) we need to start an instance of PostgreSQL.  The quickest way to do that to use an
offical Docker container image. For more information, please see [docker-compose.yml](https://turreta.com/blog/2019/09/09/docker-compose-yml-for-mysql/).

```
docker run --rm --network=host --name=postgres -e POSTGRES_PASSWORD=welcome -e POSTGRES_USER=postgres postgres
```

From another shell window, exec into the container to create the test table
```
docker exec -it postgres psql --username=postgres
postgres# create type user_role as enum ('admin', 'user');
CREATE TYPE
postgres# CREATE TABLE IF NOT EXISTS users (
        id        bigserial PRIMARY KEY,
        name      text      NOT NULL UNIQUE,
        role      user_role NOT NULL
    );
CREATE TABLE
postgres# \q
```

```
 $ export DATABASE_URL=postgres://postgres@localhost:5432/postgres
 $ env DATABASE_URL=postgres://postgres@localhost:5432/postgres cargo run
warning: unused manifest key: package.Authors
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/rust-sqlx`
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

That's it. If run again you will more rows added to the table. When the row count is greater than 10 it will start delete 1 row for each row added.
