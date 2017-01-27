# Database monitor

PostgreSQL database monitor tool to search PostgreSQL databases. Provides simple WEB interface to find database on multiple PostgreSQL servers.

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
* `-c` (`--config`) CONFIG: set path to configuration file in JSON format.

## Configuration file

Configuration file should be in JSON format with all fields required. See `config.json` file in project root. Configuration file fields:

* `address`: string, represents local address to bind on;
* `port`: integer, represents port to listen on. Should be in 0-65535 range;
* `interval`: interval between probing databases;
* `servers`: object represents server name and credentials:
  * `name`: string, server host name or address (will be shown in GUI);
  * `role`: string, role to login with;
  * `password`: string: password to get access to the server.
