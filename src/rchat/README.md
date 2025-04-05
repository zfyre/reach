# Working with Diesel

1. Setup diesel through diesel CLI:
   ```bash
    diesel setup
   ```
2. Generate a 'rchat' migration:
   ```bash
    diesel migration generate rchat
   ```
2. Run the migrations:
   ```bash
    diesel migration run
   ```
3. Redo the migrations:
   ```bash
    diesel migration redo
   ```

## A few Diesel and Postgress commands

### Starting the Postgres server
```bash
psql -U postgres -d server_name(Ex. reach) # Connect to the Postgres server as the postgres user
``` 

### Postgress CLI commands
```bash
\c rchat # Connect to the rchat database

\dt # List all tables in the current database
\dt schema_name.* # List all tables in a specific schema

\di # List all indexes in the current database
\di schema_name.* # List all indexes in a specific schema

\dls # List all databases
\x # Toggle expanded display for query results
\l # List all databases
\l+ # List all databases with additional information
\dn # List all schemas in the current database
\dt+ # List all tables with additional information

SELECT * FROM rchat.history; # Select all rows from the history table
SELECT * FROM rchat.history WHERE id = 1; # Select a specific row from the history table

SET client_encoding = 'UTF8'; # Set the client encoding to UTF8
```

