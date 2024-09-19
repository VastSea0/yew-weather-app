# Yew Weather App 🦀

This project is a weather application built using Yew, a modern web framework for creating multi-threaded front-end web apps with WebAssembly. It uses Trunk for easy development and building.

## Features

- Fetches and displays weather data
- Built with Yew framework
- Uses WebAssembly for performance
- Trunk for easy development and building

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- Trunk installed (`cargo install trunk`)
- A modern web browser that supports WebAssembly

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/yew-hava-durumu.git
   cd yew-hava-durumu
   ```

2. Install dependencies:
   ```
   cargo build
   ```

## Development

To run the project in development mode:

```
trunk serve
```

This will start a local development server, usually at `http://127.0.0.1:8080`. Trunk will watch for file changes and automatically rebuild your project.

## Building for Production

To build the project for production:

```
trunk build --release
```

This will generate optimized files in the `dist` directory.

## Project Structure

- `src/`: Contains the Rust source code
- `Cargo.toml`: Rust package manifest
- `index.html`: Entry point for the web application
- `Trunk.toml`: Trunk configuration file (if you have one)

## Contributing

Contributions to this project are welcome. Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Yew Framework: https://yew.rs/
- WebAssembly: https://webassembly.org/
- Trunk: https://trunkrs.dev/