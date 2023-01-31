# semver-rs

The awesome [pg-semver][] extension but implemented in Rust.
It supports all the operations [pg-semver][] does.

Supported PostgreSQL Versions:
* 14
* 15

## Installation

### Building from source

```bash
~$ git clone https://github.com/fabmation-gmbh/pg_semver-rs
~$ cd pg-semver-rs
~$ cargo pgx install pg15 --release
```

## Similar Projects

* [pg-semver][] – The _original_ extension which is written in C.


## License

For the license see [LICENSE](./LICENSE).

NOTE: The documentation is based on the [pg-semver][] documentation since they share the same features with the same syntax. Their documentation is licensed under the _PostgreSQL License_.

<!-- LINKS -->
[pg-semver]: https://github.com/theory/pg-semver

