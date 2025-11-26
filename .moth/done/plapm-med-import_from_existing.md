# Importing and dissecting

`bok import` should take a path to a markdown file (md or qmd), read its contents, use an external LLM to split it into individual smallest possible items (see IDEA.md in the root for the purpose) without breaking semantic "boundaries" - e.g. not nececcarily individual sentences, but likely paragraphs, lists (one parent for the list and each list item as a child node), code snippets, mermaid diagrams, etc and import them all as children of a node specified with `--under` flag. Important that each imported node will have a metadata attribute "after" specifying a preceding node as was in the original document.

upon dissection we should validate that exactly the same document can be reconstructed from individual nodes to ensure no data is lost.

the LLM service should be provided by an abstraction module that can work with different providers (local or remote ollama, anthropic, openai) optionally configured (location, model etc) in book.conf (local ollama and qwen3:14b being default).

## Implementation Details

-   **`bok import` command:**
    -   Added `import` subcommand to `bok/src/main.rs` with `file` and optional `under` arguments.
    -   Modified `bok/src/commands/import.rs` to read the specified markdown file.
    -   Integrated LLM abstraction for dissecting content.
    -   Implemented node creation for each dissected part, linking them with the `after` metadata.
    -   Added validation to reconstruct the document from created nodes and compare it with the original.
-   **LLM Abstraction:**
    -   Modified `LlmProvider` trait in `bok/src/llm.rs` to return `Result<Vec<(String, String)>>` for `dissect_markdown`.
    -   Updated `DummyLlmProvider` in `bok/src/llm.rs` to return `(blurb, content)` tuples.
    -   Updated `OllamaProvider` in `bok/src/llm_providers/ollama.rs` to return `(blurb, content)` tuples and adjusted the system prompt accordingly.
    -   Added `get_llm_provider` function to `bok/src/llm.rs` to return the appropriate LLM provider based on configuration.
    -   Integrated `anyhow` for error handling.
    -   Implemented `OllamaProvider` to interact with a local Ollama service using `reqwest` and the chat endpoint.
    -   Added `tokio` and `reqwest` in `bok/Cargo.toml` for asynchronous HTTP requests.
    -   Moved Ollama-specific logic to `bok/src/llm_providers/ollama.rs` for better organization.
    -   Created `bok/src/llm_providers/mod.rs` to declare the `ollama` module.
    -   **Attempted to integrate `ollama-rs` crate and the `generate` endpoint, but encountered persistent issues with stream consumption in a blocking context. Decided to revert to the working `reqwest` implementation for stability.**
-   **Configuration:**
    -   Created `bok/bok.conf` as a placeholder for LLM configuration.
    -   Added `config` and `anyhow` crates to `bok/Cargo.toml`.
    -   Created `bok/src/config.rs` to handle loading and managing application configuration, including LLM settings.
    -   Updated `bok/src/config.rs` to use the `hocon` crate directly for parsing `bok.conf` as per user's instruction.
    -   Changed default Ollama model to `qwen3:8b` in `bok/bok.conf` and `bok/src/config.rs`.
-   **Node Creation and Metadata:**
    -   Modified `bok/src/commands/node.rs` to accept `blurb` and `content` as separate arguments in `create_node`.
    -   Updated `bok/src/commands/node.rs` (`add` function) and `bok/src/commands/init.rs` to pass appropriate arguments to `create_node`.
    -   Added `get_node_content` to `bok/src/commands/node.rs` to retrieve the content of a node's `text.qmd` file for validation.
    -   Updated `bok/src/commands/init.rs` to use the `hocon` crate for validating the `book.conf` content before writing it.
    -   Confirmed that node's meta file is already named `meta.hocon` in `bok/src/commands/node.rs`.
-   **Validation:**
    -   Implemented logic in `bok/src/commands/import.rs` to reconstruct the document from created nodes and compare it with the original input.
-   **Integration Test:**
    -   Created `bok/tests/integration_import_test.rs` to test the `import` command.
    -   Renamed the test to `test_import_command_with_ollama_provider` to reflect its actual functionality.
    -   Added `assert_cmd` and `predicates` as dev-dependencies to `bok/Cargo.toml`.
    -   The test successfully verifies the import process, node creation, and content validation using the `OllamaProvider`.

**Next Steps:**
-   Implement Anthropic and OpenAI providers.
-   Refine the dissection logic to handle different markdown elements (paragraphs, lists, code snippets, etc.) more accurately.
-   Improve error handling and user feedback.