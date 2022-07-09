# Setup PostgreSQL

## First Setup in Linux

```bash
# install postgresql
sudo apt install postgresql

# /usr/lib/postgresql/14/
# cat /etc/postgresql-common/createcluster.conf
cat /etc/postgresql/14/main/postgresql.conf 

# connect to db as postgres user
sudo -u postgres psql postgres

# change password
\password postgres

# exit
\q
```

## Create Configuration

```bash
DB_NAME=mdb_test
DB_USER=test_user
DB_PWD=test
DB_HOST=127.0.0.1
DB_PORT=6543
```

```bash
# change port
nano /etc/postgresql/14/main/postgresql.conf 

# restart
sudo systemctl restart postgresql

# psql -h localhost -U postgres -p 5432
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

## Check Table

```bash
psql -h 127.0.0.1 -p 6543 -U test_user mdb_test 
\dt     ||      \dt+
```
