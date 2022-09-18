# How to Run

- Make sure cargo is installed
- Keep an instance of postgresql running
- Make a .env file consisting of

```env
SERVER_ADDR=127.0.0.1:8080
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/optic?schema=optic"
JWT_ACCESS_TOKEN_SECRET_0=xxx
JWT_REFRESH_TOKEN_SECRET_0=yyy
JWT_ACCESS_TOKEN_SECRET_1=xxx
JWT_REFRESH_TOKEN_SECRET_1=yyy
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
CLIENT_URL=http://localhost:3000
```

- Run the following commands

## For First Time

````shell
$ cargo prisma migrate dev
```

```shell
$ cargo prisma generate
$ cargo server
````
