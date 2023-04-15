## Build

Docker is installed in rootless mode.

If you are not in Debian, you can build the app to this platform running:

```bash
docker pull rust
make build_for_debian
```

## Run

The file `contacts.csv` must exist in the path from where the program is executed.

```bash
make run
```

## Resources

Tutorial:

<https://docs.rs/csv/latest/csv/tutorial/>
