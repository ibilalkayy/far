# 🛠️ fara — Find And Replace App

**fara** stands for **Find And Replace App** — a fast and flexible command-line tool to search and replace text across files and folders.

---

## ✨ Features

* 🔍 **Find** text in files or entire folders.
* ✏️ **Replace** text easily and quickly.
* 🎯 **Target** specific files, directories, or use glob patterns like `**/*.rs`.
* 🔡 **Case-insensitive** search with `--ignore-case`.
* 📝 Optional **regex** support for advanced matching.
* 💾 Backup files with `--backup` (if needed).
* 🚫 **Dry run** option to preview changes.

---

## 🧱 Installation

You can install `fara` using [Cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install fara
```

Alternatively, clone the repository and build it manually:

```bash
git clone https://github.com/ibilalkayy/fara.git
cd fara
cargo build && cargo install --path .
```

---

## 🚀 Usage

### Basic Example

```bash
fara --find "Foo" --replace "Bar" --target "./src/**/*.rs"
```

This will search for `"Foo"` in all `.rs` files under `./src/` and replace it with `"Bar"`.

---

### 🔧 Options

| Option                   | Description                                               |
| ------------------------ | --------------------------------------------------------- |
| `-f`, `--find <TEXT>`    | The text to search for                                    |
| `-r`, `--replace <TEXT>` | The new text to replace the matched text                  |
| `-t`, `--target <PATH>`  | The file or folder to search in (supports glob patterns)  |
| `-e`, `--regex <REGEX>`  | Use a regular expression for advanced matching            |
| `--ignore-case`          | Ignore case when matching text                            |
| `--confirm`              | Ask for confirmation before replacing                     |
| `--dry-run`              | Show the changes without modifying the file (coming soon) |
| `--output <FILE>`        | Write the changed text to a different output file         |
| `--backup <PATTERN>`     | Create a backup of matching files                         |
| `-h`, `--help`           | Show help information                                     |
| `-V`, `--version`        | Show version information                                  |

---

## 📂 Example Commands

Replace `"Hello"` with `"Hi"` in all `.txt` files in a folder:

```bash
fara --find "Hello" --replace "Hi" --target "./notes/**/*.txt" --confirm
```

Use regex to replace digits with a dash:

```bash
fara --regex "\d+" --replace "-" --target "./data/*.csv"  --confirm
```

Case-insensitive replace:

```bash
fara --find "welcome" --replace "Hello" --target "main.rs" --ignore-case
```

---

## 🤝 Contributing

We welcome contributions!
If you find a bug, want to request a feature, or contribute code, feel free to [open an issue](https://github.com/ibilalkayy/fara/issues) or submit a pull request.

---

## 📄 License

Licensed under the [Apache-2.0 License](LICENSE).
