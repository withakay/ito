## ADDED Requirements

### Requirement: FileSystem trait for dependency injection

The `ito-common` crate SHALL define a `FileSystem` trait that abstracts filesystem operations, enabling dependency injection for testing without requiring a DI container framework.

#### Scenario: Trait is object-safe
- **WHEN** using `&dyn FileSystem`
- **THEN** compilation succeeds (trait is object-safe)

#### Scenario: Trait supports Send + Sync
- **WHEN** using `FileSystem` in async or multi-threaded contexts
- **THEN** trait bounds include `Send + Sync`

### Requirement: FileSystem trait methods

The `FileSystem` trait SHALL provide methods for common filesystem operations: read, write, exists, create_dir_all, read_dir, remove_file, remove_dir_all.

#### Scenario: Read file contents
- **WHEN** calling `fs.read_to_string(path)`
- **THEN** returns file contents as `io::Result<String>`

#### Scenario: Write file contents
- **WHEN** calling `fs.write(path, contents)`
- **THEN** writes contents to path as `io::Result<()>`

#### Scenario: Check file existence
- **WHEN** calling `fs.exists(path)`
- **THEN** returns `bool` indicating if path exists

#### Scenario: Create directories recursively
- **WHEN** calling `fs.create_dir_all(path)`
- **THEN** creates all parent directories as needed

#### Scenario: List directory contents
- **WHEN** calling `fs.read_dir(path)`
- **THEN** returns iterator of directory entries

### Requirement: StdFs default implementation

The crate SHALL provide a `StdFs` struct implementing `FileSystem` that delegates to `std::fs` operations.

#### Scenario: StdFs is zero-cost
- **WHEN** using `StdFs`
- **THEN** it is a zero-sized type (no runtime overhead)

#### Scenario: StdFs implements Default
- **WHEN** calling `StdFs::default()`
- **THEN** returns a usable StdFs instance

#### Scenario: StdFs delegates to std::fs
- **WHEN** calling `StdFs.read_to_string("/etc/hostname")`
- **THEN** delegates to `std::fs::read_to_string`

### Requirement: Generic functions accept FileSystem

Functions that perform filesystem I/O SHALL accept a generic `F: FileSystem` parameter rather than calling `std::fs` directly.

#### Scenario: Config loading uses FileSystem
- **WHEN** calling `load_config(fs, path)`
- **THEN** reads files through the provided `fs` parameter

#### Scenario: Mock filesystem in tests
- **WHEN** testing config loading with a mock `FileSystem`
- **THEN** no actual filesystem access occurs
