# Feature Specification

## Overview
Currently the Node struct is tightly coupled with command logic. This change separates the Node data structure and its associated methods from the command implementations, promoting better modularity and maintainability.

## Requirements

### Node Metadata Management
When an existing node is (re)constructed, its attributes are loaded from the corresponding meta.hocon file. Changes to node attributes are saved back to the meta.hocon file.

### Node Manager
Add a separate node manager that is responsible for:
- Node finding (by id/hash)
- Construction of the node graph
- Using that graph for listing
- Generation of a "page" given a set of nodes or a criteria ("nodes under this node", "these X nodes", etc.)

## Implementation Details

### Module Structure
Created a new `node` module at `bok/src/node/` with two submodules:
- `node.rs` - Contains the Node data structure with metadata loading/saving capabilities
- `node_manager.rs` - Contains the NodeManager with all node operations and graph construction logic

### Meta Struct (`bok/src/node/node.rs`)
Created a dedicated `Meta` struct to represent the metadata stored in meta.hocon:
- Fields: `title` (String) and `after` (Option<String>)
- Implements Serialize and Deserialize for potential future use
- The `after` field is skipped during serialization if None

### Meta.yaml File Structure
The metadata file uses YAML format with an explicit `meta` section for better organization:
```yaml
meta:
  title: Node Title
  after: "123456"  # Optional
```

**Switch from HOCON to YAML:**
- YAML has excellent Rust support (serde_yaml)
- Proper serialization/deserialization instead of manual string building
- No good HOCON writing library available in Rust
- Human-readable and widely supported format

This provides:
- Clear namespacing for metadata fields
- Better extensibility for future additions (e.g., tags, status, etc.)
- Consistent structure across all node metadata files
- Type-safe serialization with serde

### Node Struct (`bok/src/node/node.rs`)
- Moved the Node struct from `commands/node.rs` to a dedicated module
- **Changed structure:** Node now contains a `meta: Meta` field instead of individual `blurb` and `after` fields
- Added accessor methods `blurb()` and `after()` for backwards compatibility
- `from_meta()` method loads the entire meta.yaml structure into the Meta struct
  - Returns `Result<Node, String>` to handle parsing errors gracefully
  - Uses serde_yaml to deserialize the entire file structure
  - Extracts `title` and `after` fields from the `meta` section in YAML
  - No longer requires blurb parameter (reads it from meta.yaml)
- `save_meta()` method persists the Meta struct to meta.yaml file
  - Uses serde_yaml::to_string() for proper YAML serialization
  - Automatically formats with correct YAML indentation
  - No manual string building required
- Made Node cloneable for use in different contexts

### bok.yaml Configuration File
The global configuration file has also been migrated from HOCON to YAML:
```yaml
llm:
  provider: ollama
  model: qwen3:8b
  location: http://localhost
  port: 11434
title: My New Book
author: Unknown Author
starting_node: "123456789"
```

**Configuration Loading:**
- `config.rs` now uses serde_yaml for deserialization
- Default values provided via Default trait on LlmSettings
- Clean struct-based configuration instead of HOCON parsing
- Type-safe with proper error handling via anyhow::Error

### NodeManager (`bok/src/node/node_manager.rs`)
Implemented as a collection of static methods providing:

**Node Finding:**
- `find_node_path()` - Finds a node directory by hash/id
- `find_node_path_with_pattern()` - Finds a node using a custom glob pattern
- `get_node_content()` - Retrieves the content of a node's text.qmd file

**Graph Construction:**
- `get_nodes_recursive()` - Builds the full node tree from filesystem, loading metadata for each node
- `sort_by_after_attribute()` - Performs topological sort based on the `after` attribute

**Node Operations:**
- `create_node()` - Creates a new node with directory structure, text.qmd, and meta.hocon
- `remove_node()` - Removes a node and its directory
- `get_all_nodes_flat()` - Returns a flat list of all nodes for CLI completions

**Listing:**
- `list_nodes()` - Lists nodes hierarchically starting from the configured starting node
- `print_nodes_recursive()` - Helper for hierarchical display with indentation

### Command Module Updates
Updated `bok/src/commands/node.rs` to be a thin wrapper around NodeManager:
- Removed all direct node manipulation logic
- Delegates to NodeManager methods
- Maintains the same public API for backwards compatibility

### Integration Updates
Updated all modules that were using the old node command functions:
- `bok/src/commands/init.rs` - Uses `NodeManager::create_node()`
- `bok/src/commands/import.rs` - Uses `NodeManager::create_node()` and `NodeManager::get_node_content()`
- `bok/src/commands/vis.rs` - Uses `NodeManager::get_nodes_recursive()` for visualization, uses shared Node struct with a local VisNode for serialization
- `bok/src/main.rs` - Added node module declaration

## Decisions Made

### Meta Struct for Metadata
**Decision:** Create a dedicated `Meta` struct to encapsulate all metadata fields from meta.hocon.
**Rationale:**
- Provides type safety and clear separation of concerns
- Makes it easy to add new metadata fields in the future without changing Node's structure
- Enables full deserialization of meta.hocon in one operation
- Accessor methods on Node maintain backwards compatibility while internal structure is cleaner

### Switch from HOCON to YAML
**Decision:** Migrate all configuration and metadata files from HOCON to YAML format.
**Rationale:**
- **Writing Support:** No robust HOCON writing library available in Rust (we were manually building HOCON strings)
- **Serde Integration:** YAML has excellent serde support with serde_yaml crate
- **Type Safety:** Proper serialization/deserialization eliminates manual string building errors
- **Industry Standard:** YAML is more widely supported and familiar to developers
- **Readability:** YAML is clean and human-readable like HOCON
- **Tooling:** Better editor support and validation tools for YAML

### Explicit Meta Section in YAML
**Decision:** Use an explicit `meta:` section in meta.yaml files instead of placing fields at the root level.
**Rationale:**
- Provides clear namespacing for metadata fields
- Better organization and readability of the YAML file
- Allows for future expansion with other top-level sections (e.g., `content`, `links`, `tags`)
- More maintainable as the metadata structure grows
- Follows best practices for structured configuration

### Module Organization
**Decision:** Create a separate top-level `node` module rather than nesting it under `commands`.
**Rationale:** This emphasizes that Node is a core data structure, not a command implementation.

### NodeManager as Static Methods
**Decision:** Implement NodeManager with static methods rather than as an instance with state.
**Rationale:** The manager doesn't need to maintain state between operations - all state is persisted in the filesystem.

### Metadata Loading Strategy
**Decision:** Load metadata from meta.hocon on-demand during graph construction rather than caching.
**Rationale:** Keeps the implementation simple and ensures we always have fresh data from disk.

### Backwards Compatibility in vis.rs
**Decision:** Keep a local `VisNode` struct for serialization rather than making Node serializable.
**Rationale:** The serialization format is specific to the visualization output and shouldn't constrain the core Node struct.

## Rejected Approaches

### Individual Metadata Fields on Node
**Rejected:** Keeping `blurb` and `after` as direct fields on Node struct.
**Reason:**
- Less maintainable as more metadata fields are added
- Mixes the Node's identity (id, children) with its metadata
- Harder to extend metadata without modifying Node struct
- The Meta struct provides better encapsulation

### Caching Node Graph
**Rejected:** Maintaining an in-memory cache of the node graph.
**Reason:** Adds complexity without clear benefit for a CLI tool where graph construction is fast enough.

### Making Node Serializable
**Rejected:** Adding Serialize/Deserialize derives to the core Node struct.
**Reason:** Serialization concerns are specific to certain use cases (like visualization) and shouldn't be part of the core data structure.

### Instance-based NodeManager
**Rejected:** Creating a NodeManager instance that holds the node graph.
**Reason:** No need for state management - all operations are independent and work directly with the filesystem.