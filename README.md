# Reccak

Simplified SHA3 Hash function written in Rust.

# Usage

To calculate hash of some input you can either pass it as an argument

```bash
$ cargo run --release hash -- <some-input>
```


### Using rayon
To reverse hashes(defined in `src/reverse_hash.rs`):
```
cargo run --release --bin reverse-hash-rayon
```

# Using custom thread pool

```
cargo run --release --bin reverse-hash-custom
```

It will use N workers, where N is number of CPUs. To override this value, use `WORKERS` environment variable, e.g with two workers:
```
WORKERS=2 cargo run --release --bin reverse-hash-custom
```
