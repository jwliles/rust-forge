# Forge Rust Migration and TUI Enhancement Context

## Project Goals
- **Performance**: Improve execution speed and memory efficiency with Rust's zero-cost abstractions
- **Safety**: Leverage Rust's memory safety guarantees for more robust file operations
- **Discoverability**: Add an interactive TUI while maintaining full CLI functionality
- **Cross-Platform**: Better handle path differences across operating systems

## Design Decisions
- **Core/UI Separation**: Split functionality into core library and UI implementations
- **CLI-First Development**: Ensure all features remain accessible through CLI commands
- **TUI as Enhancement**: Implement TUI as an optional feature via Cargo feature flags
- **Forge Metaphor**: Use consistent terminology (forge, heat, cool) throughout interfaces

## Project Structure
```
forge/
├── crates/
│   ├── forge-core/       # Core library with all functionality
│   ├── forge-cli/        # CLI implementation
│   └── forge-tui/        # TUI implementation (optional)
├── Cargo.toml            # Workspace definition
└── README.md             # Project documentation
```

## Core Concepts

### File Status Workflow
- **Untracked**: Regular files not managed by Forge
- **Heated/Staged**: Files prepared for forging (staging area)
- **Forged/Tracked**: Files managed by Forge with active symlinks

### TUI Layout
- **File System Navigator**: Browse filesystem to find configuration files
- **Forge Inventory**: View all currently tracked files with symlink status
- **Heating Station**: See files staged for forging
- **File Details**: Preview file contents and metadata
- **Command Log**: View underlying commands being executed

### Key Interactions
- **Space**: Stage/unstage files (toggle heating)
- **F**: Forge selected or staged files
- **Tab**: Move between panels
- **/**: Search across files with Skim integration
- **C**: Cool (unforge) tracked files

## Technical Considerations
- **Rust Ownership Model**: Handle shared state between UI components
- **Async File Operations**: Prevent UI blocking during filesystem operations
- **Feature Flags**: Allow compiling with or without TUI support
- **Cross-Platform Path Handling**: Abstract platform-specific behavior

## Implementation Strategy

### 1. Core Library Migration (Phase 1)
- Port existing Go functionality to Rust library
- Create clean API for all operations
- Implement thorough testing
  
### 2. CLI Implementation (Phase 2)
- Create CLI using clap with familiar commands
- Maintain backward compatibility
- Improve error reporting

### 3. TUI Development (Phase 3)
- Implement TUI using ratatui and crossterm
- Add Skim integration for fuzzy finding
- Ensure all core functionality is accessible

## Future Considerations
- Profile system for managing different groups of configurations
- Virtual environments for safe experimentation
- Git integration for version control
- Remote synchronization capabilities
- Enhanced visualization tools

## Development Priorities
1. Core functionality with memory safety and performance improvements
2. CLI with feature parity to current version
3. Basic TUI functionality
4. Advanced TUI features (diff view, batch operations)
5. Performance optimizations

This context helps ensure development remains focused and aligned with the tool's purpose while meeting real user needs effectively.
