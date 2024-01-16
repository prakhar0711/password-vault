# Password Vault in Rust


## Overview

This Rust-based Password Vault is a secure and user-friendly command-line application designed to manage and store passwords with a focus on data integrity and persistence. The application leverages Rust's file handling capabilities, JSON serialization, and the Serde library for structured data management.

## Features

1. **Add Entry:**
   - Securely add new entries to the password vault, providing service, username, and password information.

2. **List Entries:**
   - View a list of all stored entries in the password vault, facilitating easy access to saved credentials.

3. **Search Entry:**
   - Search for specific entries by service name, enabling quick retrieval of relevant credentials.

4. **File I/O and Error Handling:**
   - Robust file operations ensure the secure storage of passwords, with effective error handling for a reliable user experience.

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/prakhar0711/password-vault.git
   ```

2. Navigate to the project directory:
   ```bash
   cd password-vault
   ```

3. Build and run the application:
   ```bash
   cargo run
   ```

### Simplify

Navigate to :

   ```bash
   "password-vault/target/release" and run "password-vault.exe" file.
   ```
## Dependencies

- [Serde](https://github.com/serde-rs/serde): Serialization/deserialization library for Rust.
- [Serde JSON](https://github.com/serde-rs/json): JSON support for Serde.

## Contributions

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or create a pull request.


