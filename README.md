# Bakman — Bakcup CLI Tool

Bakman is a lightweight command-line backup utility designed to streamline the process
of backing up files and directories. Bakman allows users to quickly create reliable
backups of their important data, whether it's individual files, entire directories,
or even custom folder structures. Bakman is perfect for both personal and professional
use, offering flexibility and efficiency in managing backups directly from the terminal.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Features

- File and directory backup.
- Backup versioning based on date.
- Restore functionality.

Will be added:

- Backups compression.
- Encryption.
- Incremental backupss.

## Installation

To install `bakman`, just use `cargo`:

```bash
cargo install bakman --git https://github.com/kulichkoff/bakman
```

## Usage

You have an ability to see usage of the tool just using `help` command.

```bash
bakman help
bakmen help backup
```

To backup a file or a whole directory just use command below:

```bash
bakman backup path/to/file
```

...or you can specify output directory:

```bash
bakman backup path/to/file -o /var/backups
```

Replace `<profile_name>` with the name of the profile you want to switch to.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
