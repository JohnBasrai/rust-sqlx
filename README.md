# rust-sqlx
Example of how to insert and update a record with an enum column using _rust-sqlx/postgresql_. I could not find any examples of how to do the update operation (to an enum), but I found bits and pieces which I pulled together to write this demo.  Hopefully it will save someone else some grief.

***
## Running
Before running this demo against PostgreSQL (other databases supported by sqlx should also work
with changes) we need to start an instance of PostgreSQL.  The quickest way to do that to use an
official Docker container image. For more information, please see [docker-compose.yml](https://turreta.com/blog/2019/09/09/docker-compose-yml-for-mysql/).

```
docker run --rm --network=host --name=postgres -e POSTGRES_PASSWORD=welcome -e POSTGRES_USER=postgres postgres
```

From another shell window, exec into the database container to create the test table.
```
docker exec -i postgres psql --username=postgres < db-init.sql
```
Then build and run the code.
```
 $ export DATABASE_URL=postgres://postgres@localhost:5432/postgres
 $ cargo run
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

That's it.  If you run again you will see more rows added to the table. When the row count is
greater than 10 it will delete 1 row for each row added.
