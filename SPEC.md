# BOK CLI Specification

This document provides a detailed specification for the `bok` Command Line Interface (CLI). It is based on the project's source code, internal documentation, and design ideas. The specification is intended to be sufficient to recreate the CLI from scratch without referencing the original source code.

## 1. Overview

`bok` is a CLI tool designed to assist authors in writing books using a non-linear, graph-based approach. The core idea is to capture thoughts and concepts as a recursive graph of nodes, which can then be organized, edited, and compiled into a linear book format.

The system treats each idea or concept as a "node." Each node can contain content and be linked to other nodes, forming a hierarchical or relational structure. The CLI provides tools to manage these nodes, interact with Large Language Models (LLMs) for content generation and editing, and visualize the overall structure of the book.

## 2. Core Concepts

### 2.1. Node

A **Node** is the atomic unit of content in the `bok` system.

-   **Representation**: Each node is represented by a directory on the filesystem.
-   **Directory Name**: The directory name consists of the node's unique ID and a short, human-readable title (blurb), separated by a space (e.g., `123456789 Some Title`).
-   **Unique ID**: A 32-bit hash generated using the **Murmur3** algorithm from the node's initial blurb.
-   **Content**: The main text of the node is stored in a `text.qmd` (Quarto Markdown) file within its directory.
-   **Metadata**: Node metadata is stored in a `meta.hocon` file within its directory. This HOCON file contains:
    -   `title`: The node's blurb.
    -   `after`: An optional string field containing the ID of a sibling node that should precede this one, used for ordering.

### 2.2. Book Configuration

The project is defined by a `bok.hocon` file in the root directory. This file contains global metadata for the book and configuration for the CLI.

-   `title`: The title of the book.
-   `author`: The author of the book.
-   `starting_node`: The ID of the root node from which hierarchical operations like `node ls` should begin.
-   `llm`: A configuration block for the Language Model provider (see below).

### 2.3. LLM Providers

The CLI uses an abstraction for interacting with Large Language Models. This allows for a pluggable architecture supporting different LLM backends.

-   **Provider Trait**: A core `LlmProvider` trait defines the contract for LLM interactions, primarily the `dissect_markdown` function.
-   **Ollama Provider**: The primary implementation uses a local Ollama instance. It communicates with the Ollama `/api/generate` endpoint via asynchronous HTTP requests using the `reqwest` library.
-   **Dummy Provider**: A fallback provider for testing and development that splits markdown content by double newlines.
-   **Extensibility**: The design allows for future providers like Anthropic or OpenAI.

## 3. System Architecture & Technology

-   **Language**: **Rust** (2024 Edition).
-   **Asynchronous Runtime**: **Tokio** is used to enable asynchronous operations, particularly for handling streaming responses from LLMs.
-   **CLI Framework**: **`clap`** (v4) is used for parsing command-line arguments, defining subcommands, and managing shell completions.
-   **Configuration Management**:
    -   Configuration is read from `bok.hocon` files.
    -   The **`hocon`** crate is used for parsing these files.
    -   A default configuration is hardcoded, which is overridden by settings in the `bok.hocon` file.
-   **LLM Interaction**:
    -   Asynchronous HTTP requests are made with **`reqwest`**.
    -   Streaming responses from the LLM are processed line-by-line using `tokio::io::BufReader`.
    -   An `AsyncIterator` trait is defined to abstract over the streaming response, providing a clean interface for consumers.
-   **Filesystem Interaction**:
    -   The **`glob`** crate is used for finding node directories based on hash patterns.
    -   Standard library `std::fs` is used for file and directory creation/deletion.
-   **Serialization**: **`serde`** and **`serde_json`** are used for serializing data structures into JSON (for `vis d3`) and for deserializing LLM responses.

## 4. CLI Command Reference

The CLI is structured around a main command `bok` followed by several subcommands.

---

### `bok init`

-   **Purpose**: Initializes a new book project in the current directory.
-   **Arguments**:
    -   `[blurb]`: An optional string that serves as the title for the root node. Defaults to "Starting Node".
-   **Behavior**:
    1.  Creates a root node using the provided blurb.
    2.  Creates a `bok.hocon` file in the current directory.
    3.  Populates `bok.hocon` with default book metadata (`title`, `author`) and sets the `starting_node` to the ID of the newly created root node.

---

### `bok node`

Manages the nodes within the book.

#### `bok node add <blurb>`

-   **Purpose**: Adds a new child node.
-   **Arguments**:
    -   `blurb`: A required string that becomes the node's title.
-   **Options**:
    -   `--under <node-id>`: The ID of an existing node to create the new node under. If not provided, the new node is created at the root level.
-   **Behavior**:
    1.  Generates a new node ID using Murmur3 on the blurb.
    2.  Creates a new directory named `<id> <blurb>` under the parent directory (or root).
    3.  Creates a `text.qmd` file inside the new directory containing the blurb as content.
    4.  Creates a `meta.hocon` file with the `title`.

#### `bok node rm <node-id>`

-   **Purpose**: Removes a node and all its children.
-   **Arguments**:
    -   `node-id`: The ID of the node to remove.
-   **Behavior**:
    1.  Searches the entire directory tree for a directory starting with the given ID.
    2.  Recursively deletes the directory and all its contents.

#### `bok node ls`

-   **Purpose**: Lists the node hierarchy in a tree-like structure.
-   **Behavior**:
    1.  Reads `bok.hocon` to find the `starting_node`.
    2.  Recursively traverses the filesystem from the current directory to build a tree of all nodes.
    3.  Sorts sibling nodes based on the `after` attribute in their `meta.hocon` files using a topological sort.
    4.  Prints the hierarchy to the console, starting from the configured `starting_node`, with indentation to show parent-child relationships. The output format for each line is `<id> <blurb>`.

---

### `bok import`

-   **Purpose**: Imports content from a markdown file, splitting it into multiple nodes using an LLM.
-   **Arguments**:
    -   `file`: The path to the markdown (`.md` or `.qmd`) file to import.
-   **Options**:
    -   `--under <node-id>`: The ID of a parent node under which the new nodes will be created.
-   **Behavior**:
    1.  Reads the content of the specified file.
    2.  Sends the content to the configured LLM provider via the `dissect_markdown` function.
    3.  The LLM streams back pairs of `["blurb", "content"]` for each dissected semantic unit.
    4.  For each pair received, a new node is created under the specified parent.
    5.  The nodes are linked sequentially using the `after` metadata attribute to preserve the original document order.
    6.  After import, it reconstructs the document from the newly created nodes and compares it to the original file content to validate that no data was lost.

---

### `bok vis`

Generates visualizations of the node graph.

#### `bok vis d3`

-   **Purpose**: Generates a JSON output of all root-level nodes, suitable for consumption by D3.js.
-   **Behavior**:
    1.  Scans the root directory for node directories.
    2.  For each node, it reads the `title` from `meta.hocon`.
    3.  Outputs a JSON array of objects, where each object has an `id` (the directory name) and `title`.

#### `bok vis mermaid`

-   **Purpose**: Generates a Mermaid diagram of the node graph.
-   **Status**: Not yet implemented.

---

### `bok generate`

Generates book outputs from the node graph.

#### `bok generate quarto`

-   **Purpose**: Generates a Quarto book.
-   **Status**: Not yet implemented.

---

### `bok check`, `bok lineedit`, `bok copyedit`

-   **Purpose**: These commands are placeholders for future functionality related to content validation and LLM-assisted editing.
-   **Status**: Not yet implemented.

---

### `bok completion`

-   **Purpose**: Generates shell completion scripts.
-   **Arguments**:
    -   `shell`: The target shell (e.g., `bash`, `zsh`, `fish`).
-   **Behavior**:
    -   Uses `clap_complete` to print a completion script for the specified shell to standard output.

## 5. Configuration File Schema (`bok.hocon`)

The application uses a combination of a hardcoded default configuration and the `bok.hocon` file.

```hocon
# Default values are shown
llm {
    provider = "ollama"
    model = "qwen3:8b"
    location = "http://localhost"
    port = 11434
}

# These are set by 'bok init'
title = "My New Book"
author = "Unknown Author"
starting_node = "123456789" // Example hash
```

## 6. LLM Dissection Protocol (Ollama)

The `import` command relies on a specific protocol for interacting with the Ollama API.

1.  **Request**: A `POST` request is sent to `/api/generate`. The body is a JSON object containing:
    -   `model`: The model name from the configuration.
    -   `stream`: `true`.
    -   `prompt`: A detailed system prompt instructing the model to act as a markdown dissector, followed by the user's markdown content. The prompt specifically asks the model to respond with a one-line JSON array `["blurb", "dissected_unit_content"]` for each semantic unit.

2.  **Response**: The API responds with a stream of server-sent events.
    -   Each event is a JSON object, typically containing a `response` field with a chunk of the generated text.
    -   The `OllamaProvider` reads this stream line-by-line.

3.  **Processing**:
    -   The client buffers the content from the `response` fields.
    -   When the buffer contains a newline character, it extracts the complete line.
    -   This line is parsed as a JSON array of two strings: `[blurb, content]`.
    -   This pair is yielded to the `import` command logic, which then creates a node.
    -   This process repeats until the stream is closed.

## 7. Shell Completion

-   **Generation**: Handled by `clap` and the `bok completion` command.
-   **Dynamic Values**: For arguments that accept a node ID (e.g., `node add --under`), completions are dynamically generated by scanning the filesystem for all existing nodes.
-   **Value Formatting**: To handle blurbs with spaces, the completion value is formatted as `<id>-<blurb-with-hyphens>`. When the command is executed, the CLI logic parses this string to extract only the `<id>` part before using it.

## 8. Testing Strategy

The project employs a combination of integration and end-to-end tests to ensure the CLI's correctness and stability.

-   **Integration Tests**: The primary testing method involves Rust-based integration tests located in the `tests/` directory.
    -   **Framework**: Tests are written as standard Rust tests that execute the compiled `bok` binary as a subprocess.
    -   **Tooling**:
        -   **`assert_cmd`**: Used to run the CLI command and make assertions on its exit code, `stdout`, and `stderr`.
        -   **`predicates`**: Used in conjunction with `assert_cmd` to create expressive assertions on the output text.
        -   **`tempfile`**: Used to create temporary directories, providing an isolated filesystem environment for each test run, which is crucial for commands that create or modify files (`init`, `node add`, `import`).
-   **End-to-End Tests**:
    -   A shell script, `e2e_test.sh`, exists for broader, script-based end-to-end testing scenarios for each command and each combination of arguments.
    -   Import end-to-end test is by default performed against the `DummyProvider` to avoid the need for a live LLM instance. There's a flag to enable testing against a live Ollama instance if desired.
-   **Conditional Tests**:
    -   A Cargo feature flag, `ollama_tests`, is used to gate tests that require a live, running Ollama instance.
    -   This allows developers to run the core suite of tests without needing to set up the Ollama dependency, while still enabling full end-to-end validation in environments where it is available (e.g., `cargo test --features ollama_tests`).
