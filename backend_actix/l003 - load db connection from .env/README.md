# create postgresql new configuration

```text
DB_NAME=mdb_test
DB_USER=test_user
DB_PWD=test
DB_HOST=127.0.0.1
DB_PORT=6543
```

```bash
# connect to db
psql -h 127.0.0.1 -U postgres -p 6543

# create user
DROP USER IF EXISTS test_user;
CREATE USER test_user WITH ENCRYPTED PASSWORD 'test';

# check if user creation succeeded
\du

# create database
create database mdb_test;

# assign user to db
GRANT ALL PRIVILEGES ON DATABASE mdb_test to test_user;
```
