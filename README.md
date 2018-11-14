# Rust Http Server

## Run
```
cargo build --release && target/release/rustoserve

# After accessing through browser
Request took: 0.041129 ms
```

In debug mode, the log print `Request took: 0.177581 ms`.

## Comparation
Compared to PHP counterpart (see `phpsrc/index.php`), Rust performance is in par.

```
$ cd phpsrc/ && php -S localhost:3001

# After accessing through browser
[Wed Nov 14 09:01:55 2018] Time: 0.18095970153809 ms
[Wed Nov 14 09:01:55 2018] ::1:59210 [200]: /
```
