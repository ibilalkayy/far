# far — Find And Replace

**far** stands for **Find And Replace** — a fast, flexible command-line tool to search and replace text across files and folders.

## Features

- 🔍 Find text in files or folders
- ✏️ Replace text quickly and easily
- 🎯 Target specific files, directories, or glob patterns
- ⚙️ Optional dry-run support (coming soon)
- 🧠 Smart casing support (e.g. `Foo` → `Bar`, `FOO` → `BAR`)
- 💡 Inspired by Sublime Text’s find & replace

## Installation

```bash
git clone https://github.com/ibilalkayy/far.git
cd far
cargo build
````

## Usage

```bash
far --find "Foo" --replace "Bar" --tartget "./src/**/*.rs"
```

## 📄 License

Licensed under the [Apache-2.0 License](LICENSE).

## 🙌 Contributing

Pull requests are welcome! If you have suggestions for improvements or features, feel free to open an issue.
