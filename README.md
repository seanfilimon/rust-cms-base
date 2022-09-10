# How to Run

- Make sure cargo is installed
- Keep an instance of postgresql running
- Make a .env file consisting of

```env
SERVER_ADDR=127.0.0.1:8080
PG.USER=postgres
PG.PASSWORD=postgres
PG.DBNAME=optic
PG.HOST=localhost
PG.PORT=5432
PG.POOL.MAX_SIZE=10
```

- Run the following commands

```shell
$ chmod u+x ./schema.sh
$ ./schema.sh
$ cargo run
```
