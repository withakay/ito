# Installation

## Homebrew (macOS)

```bash
brew tap withakay/ito
brew install ito-cli
ito --version
```

Note: the Homebrew formula name is `ito-cli` (it installs the `ito` binary).

## Prebuilt Binary (macOS/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/withakay/ito/main/scripts/install.sh | sh
ito --version
```

## Build From Source

```bash
make rust-install
ito --version
```
