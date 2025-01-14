# Note:
I'm not continuing with this project since there are a thousand web server already written in Rust. However, I'm glad I learned a lot during its development.

# Stirn
**Stirn** is a web server **prototype** written in Rust, designed to host and manage multiple websites seamlessly. I am aiming to add enhancements like real-time data sharing and WebAssembly (Wasm) integration.

## Vision
Stirn aims to create a robust environment where multiple websites can interact and share resources in real-time. Imagine a scenario where two interconnected sites automatically synchronize user sessions and data, providing a seamless multi-site experience. Stirn acts as the backbone for these interactions, while a complementary project, **Stirner**, will handle frontend-level communication and data exchange between sites.

## Stirner: The Frontend Counterpart (Upcoming)
Stirner is envisioned as a project that complements Stirn by managing the frontend interactions between hosted websites. It will enable features such as:
- Real-time data synchronization between sites (e.g., a user action on Site A reflecting on Site B).
- Enhanced interactivity by leveraging Stirn’s backend capabilities.
- WebAssembly (Wasm) integration for dynamic and performance-critical frontend tasks.

Together, Stirn and Stirner will create a cohesive system where backend efficiency and frontend interactivity work hand in hand.

## Features
- **Multi-site Support:** Serve multiple websites simultaneously.
- **Configuration:** Sites are managable via `stirners.json` configuration file.
- **High Performance:** Built with Rust for speed and reliability.
- **WebAssembly (Wasm) Integration (Upcoming):** Add Wasm modules to enhance website functionalities.
- **Real-time Communication (Upcoming):** Enable websites to instantly share data with each other. Edit and synchronize information in real-time.

## Development Roadmap
### Phase 1: Core Web Server Enhancements (We are here)
1. Improve request handling to support more HTTP methods.
2. Implement basic routing capabilities.
3. Add support for static file serving with caching options.
4. Enhance logging and error handling for better debugging.
5. Optimize the server for multi-threaded environments. (use tokio?)
6. Introduce session management to track user sessions across multiple sites.
7. Develop a modular design for easier addition of future features like WebAssembly and WebSocket support.

### Phase 2: WebAssembly Integration
1. Research and choose a Rust-to-Wasm toolchain.
2. Refactor the server to support loading Wasm modules per site.
3. Implement a sample Wasm module and integrate it into one site.
4. Document the process for adding and managing Wasm modules.

### Phase 3: Real-time Communication
1. Add WebSocket support to the server.
2. Create a communication protocol for real-time data sharing.
3. Implement a demo use case (e.g., shared user sessions or collaborative tools).
4. Optimize performance for handling multiple connections.

### Phase 4: API and Configuration Enhancements
1. Extend the `stirners.json` format to support Wasm and real-time communication settings.
2. Develop an admin interface for managing configurations dynamically.

### Phase 5: Testing and Deployment
1. Write extensive unit and integration tests.
2. Test performance under load.
3. Package the server for easy deployment.
4. Release a stable version.


## Contributing
Contributions are welcome! Please submit issues or pull requests on the [GitHub repository](https://github.com/xomvio/stirn).
