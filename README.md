## Build

Docker is installed in rootless mode.

If you are not in Debian, you can build the app to this platform running:

```bash
docker pull rust
make build_for_debian
```

## Configuration

Environment variables:

```bash
# Absolute path to export the contact in HTML format.
export CONTACT_HTML_PATH_NAME=/home/foo/html
```

## Run

The file `contacts.csv` must exist in the path from where the program is executed.

### Search term

```bash
make search_term
```

### Generate html

```bash
make export
```

## Resources

Tutorial:

<https://docs.rs/csv/latest/csv/tutorial/>
