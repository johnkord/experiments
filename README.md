# experiments

This repository contains experimental projects and design documents.

## Design Documentation

The [docs/](docs/) folder contains design documents for various experimental projects:

- **[RustOS Design](docs/design/rust-os-design.md)**: Comprehensive design document for a hypervisor-native operating system written entirely in Rust
- **[Kernel/User Interaction Alternatives](docs/design/kernel-user-interaction-alternatives.md)**: Exploration of alternatives to traditional syscalls, including RustOS's innovative capability channel approach

## Projects

### RustOS
A modern, hypervisor-native operating system written entirely in Rust, designed to leverage Rust's memory safety guarantees and zero-cost abstractions. Key innovations include:

- **Capability Channels**: Revolutionary alternative to traditional syscalls
- **Hypervisor-First Design**: Optimized for virtualized environments
- **Async-First Architecture**: Built around Rust's async/await ecosystem
- **Type-Safe System Interfaces**: Compile-time verification of system interactions

See the [design documentation](docs/design/) for detailed specifications and architectural decisions.