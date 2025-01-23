# Rust NetworkManager Project

This project demonstrates how to use `zbus` for D-Bus communication in Rust,
specifically for interacting with NetworkManager. It provides a set of examples
and utility functions to manage network connections, devices, and settings
through NetworkManager's D-Bus API.

## Introduction

NetworkManager is a daemon for simplifying networking configuration on Linux
systems. `zbus` is a Rust crate that offers an idiomatic way of interacting with
D-Bus services. By leveraging `zbus`, this project aims to provide Rust
developers with an easy-to-use interface for managing network configurations
through NetworkManager.

## Prerequisites

Before you can use this project, ensure you have the following installed:
- Rust and Cargo (latest stable version recommended)
- NetworkManager on your Linux distribution

## Installation

To use this project, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/kevinvoell/network_manager.git
   ```

2. Change into the project directory:
   ```sh
   cd network_manager
   ```

3. Build the project:
   ```sh
   cargo build
   ```

## Usage

This section provides basic examples of how to use the project to interact with
NetworkManager through zbus.

## Contributing

We welcome contributions! Please open an issue or submit a pull request for any
improvements, bug fixes, or feature additions. Follow the Rust Code of Conduct
in all interactions within the project.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
