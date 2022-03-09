# Sauron

CLI watcher that periodically checks if a given endpoint is reachable and sends latency stats to
StatsD. Sauron is only able to watch after http(s) services.

```bash
HTTP service health watcher

USAGE:
    sauron <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add       Add thing
    help      Print this message or the help of the given subcommand(s)
    list      List things
    remove    Remove thing
    run       Run a one-off check
    setup     
    watch     Watch
```

## Example usage

#### List target on Sauron's watchlist: 

```bash
> ./sauron list
Monitoring:
---
Target: https://podcasti.si/health/
Metric: podcasti
---
Target: https://google.com
Metric: google
---
```

#### Remove target from Sauron's watchlist:

```bash
> ./sauron remove https://google.com 
https://google.com removed successfully.
```

#### Add target to Sauron's watchlist:

```bash
> ./sauron add https://example.com example 
https://example.com added successfully.

```

## Requirements

StatsD server is required in order to collect data.