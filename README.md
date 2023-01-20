# seaorm-mysql-array

---

this is an example program to store array data in mysql using [Foreign Key Realtionship](https://dev.mysql.com/doc/refman/5.6/en/create-table-foreign-keys.html) between entity(`Employee`) and array data table(`Projects`).

# Setup

1. create `emp_db`: `create database emp_db;`

2. run seaorm migrations. This will drop-create tables.

```sh
DATABASE_URL="mysql://root:password@localhost:3306/emp_db" sea-orm-cli migrate refresh
```

3. compile program & run. `cargo run`

# Explainer

TODO
