# ♾️ `loop`

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
        --no-stat          Do not display statistics at the end of execution
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

date total: 4 ok: 4 ko: 0
```

At the end of execution (or if `ctrl+c` interrupts the loop), a summary of the number of success and failure is outputted.
This summary can be deactivated with `--no-stat` option.

`loop` can easily be used with `time`:

- Generate random_time:
    ```bash
    $ dd if=/dev/urandom of=/dev/null bs=1 count=1024 status=none
    ```
- Loop for 1000 iteration generating random data:
    ```bash
    $ loop --iter 1000 --no-stat dd if=/dev/urandom of=/dev/null bs=1 count=1024 status=none
    ```
- How much time is needed:
    ```bash
    $ time loop --iter 1000 --no-stat dd if=/dev/urandom of=/dev/null bs=1 count=1024 status=none
    loop --iter 1000 --no-stat dd if=/dev/urandom of=/dev/null bs=1 count=1024   0.60s user 1.05s system 95% cpu 1.723 total
    ```

Another example with [`hurl`](https://hurl.dev) to perform tests on HTTP endpoints:

```bash
$ loop --while-ok hurl test.hurl
...
hurl total: 140 ok: 139 ko: 1
```

