Ziply makes specifying directories inside a ZIP simple.

# Installation

```sh
cargo install ziply
```

## Quick Start

1. Define your ZIP files with a `ziply.yaml`.
3. Run `ziply run`.

## Configuration

Here is an example of a ziply.yaml file:

```yaml
packs:
  test:
    # Specifies the output path for the generated ZIP file.
    filename: path/to/output/test.zip

    # Lists the entries that will be included in the ZIP archive.
    entries:

      # Defines the directory within the archive where files will be stored.
      - dest_dir: .

        # Specifies the files to include in the specified destination directory.
        # Files can be specified in two formats: 'source' or dictionary '{src: "", dest: ""}'.
        files:
          - ./src/main.rs # Use the original name of the source file if no destination is specified.
          # Renames the source file when adding it to the archive.
          - src: ./src/main.rs
            dest: renamed_main.rs
```
