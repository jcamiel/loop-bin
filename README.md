# Loop

Run a command in loop, nothing else.

```
USAGE:
    loop [OPTIONS] <CMD>...

ARGS:
    <CMD>...    Command to execute

OPTIONS:
    -d, --delay <delay>    Delay between iteration in milliseconds
    -h, --help             Print help information
    -i, --iter <iter>      Number of iteration
        --stats            Display statistics at the end of execution
        --while-ko         Loop while exit code is failure
        --while-ok         Loop while exit code is success
```

## Installation

```
$ cargo install loop-bin
```


## Samples

```bash
$ loop --iter 4 --delay 1000 date
Tue Aug  9 06:59:58 CEST 2022
Tue Aug  9 06:59:59 CEST 2022
Tue Aug  9 07:00:00 CEST 2022
Tue Aug  9 07:00:01 CEST 2022
```

With [`hurl`](https://hurl.dev):

```bash
$ loop --while-ok --stats hurl test.hurl
...
hurl total: 140 ok: 139 ko: 1
```