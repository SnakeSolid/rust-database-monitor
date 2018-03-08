# Database monitor

PostgreSQL database monitor is a tool to search PostgreSQL databases. Provides simple WEB interface to find database on multiple PostgreSQL servers.

PostgreSQL database monitor every 10 minutes walk all servers from configuration and executes query:

```sql
SELECT
    d.datname,
    d.datcollate,
    r.rolname
FROM pg_database AS d
    INNER JOIN pg_roles AS r ON ( r.oid = d.datdba )
WHERE
    rolcreaterole = FALSE AND
    rolcanlogin = TRUE
```

Query results are stored in worker in memory. Update interval can be defined with `--interval` option or `interval` configuration parameter.

## Usage

To start run:

```bash
./database-monitor [OPTIONS]
```

Optional arguments:

* `-h` (`--help`): show short help and exit;
* `-b` (`--bind`) BIND: address to bind on, default value localhost;
* `-p` (`--port`) PORT: port to listen on, default value 8080;
* `-i` (`--interval`) INTERVAL: probe databases interval in seconds, default value 10 minutes;
* `-c` (`--config`) CONFIG: set path to configuration file in JSON format. Parameter is required.

## Configuration file

Configuration file should be in JSON format with all fields required. See `config.json` file in project root. Configuration file fields:

* `address`: string, represents local address to bind on;
* `port`: integer, represents port to listen on. Should be in 0-65535 range;
* `interval`: interval between probing databases;
* `metadata`: should be defined to start meta-data collector:
  * `host`: string, meta-data server host name or address;
  * `port`: integer, meta-data server port (default value 5432);
  * `database`: string, meta-data database;
  * `role`: string, role to login on meta-data server;
  * `password`: string, password to login on meta-data server;
  * `query`: string: query to get meta-data information (see meta-data query section).
* `servers`: object represents server name and credentials:
  * `name`: string, server host name or address (will be shown in GUI);
  * `port`: integer, server port (default value 5432);
  * `description`: string, server description (can be null);
  * `role`: string, role to login with;
  * `password`: string: password to get access to the server.

## Meta-data query

The `metadata.query` used to retrieve commit, project name and branch name for every database. The query must return three fields:

* first - bigint, commit number;
* second - string, branch name;
* third - string, project name.

Meta-data query must contain two parameters:

* $1 - string, server name;
* $2 - string, database name;

Meta-data query example:

```sql
SELECT
  m.commit::BIGINT AS commit,
  m.branch AS branch,
  m.project AS project
FROM monitoring.monitor AS m
WHERE m.server_name = $1
  AND m.database_name = $2
ORDER BY m.id DESC
LIMIT 1
```
