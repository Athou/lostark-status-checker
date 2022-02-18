# Lost Ark status checker

Checks the status of a Lost Ark server so you don't need to refresh the status page manually.

If the server is up, sends an operating-system notification, otherwise checks again after 30s.

## Usage

    lostark-status-checker.exe [OPTIONS]

    OPTIONS:
        -h, --help                         Print help information
        -i, --interval <INTERVAL>          interval between checks if server is not up [default: 30]
        -s, --server-name <SERVER_NAME>    Name of the server [default: Trixion]
        -V, --version                      Print version information
