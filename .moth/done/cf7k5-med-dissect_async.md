make fn dissect_markdown async emitting each node as it arrives

## Implementation Details

The `dissect_markdown` function was refactored to be asynchronous and return a `Pin<Box<dyn Stream<Item = Result<(String, String)>> + Send + 'a>>`.

1.  **`bok/Cargo.toml`**: Added `async-trait = "0.1"` dependency.
2.  **`bok/src/llm.rs`**:
    *   Added `use async_trait::async_trait;`, `use futures::stream::Stream;`, and `use std::pin::Pin;`.
    *   Modified the `LlmProvider` trait to use `#[async_trait]` and updated the `dissect_markdown` signature to `async fn dissect_markdown(...) -> Result<Pin<Box<dyn Stream<Item = Result<(String, String)>> + Send + 'a>>>`.
    *   Updated the `DummyLlmProvider` implementation to match the new async signature, converting its iterator into a stream using `futures::stream::iter` and `Box::pin`.
3.  **`bok/src/llm_providers/ollama.rs`**:
    *   Added `use async_trait::async_trait;`, `use futures::stream::{self, Stream, StreamExt};`, and `use std::pin::Pin;`.
    *   Removed `use anyhow::anyhow;` and `use std::io::{Write, stderr};` as they were no longer needed.
    *   Modified the `impl LlmProvider for OllamaProvider` block to use `#[async_trait]`.
    *   Refactored the `dissect_markdown` function to be `async` and return a `Pin<Box<dyn Stream>>`.
    *   The internal logic for making the `reqwest` call and processing the byte stream was wrapped in a `stream::unfold` block.
    *   The buffering logic was updated to correctly handle incomplete lines and avoid Rust's borrowing errors by processing complete lines and updating the buffer with the remaining incomplete part.
    *   Removed a debugging `eprintln!` statement.
4.  **`bok/src/commands/import.rs`**:
    *   Added `use futures::StreamExt;`.
    *   Made the `run` function `async`.
    *   `await`ed the `llm_provider.dissect_markdown()` call.
    *   Changed the iteration over `parts_iterator` to `while let Some(part_result) = parts_stream.next().await`.
5.  **`bok/src/main.rs`**:
    *   Added `#[tokio::main]` above the `main` function.
    *   Changed `fn main()` to `async fn main()`.
    *   `await`ed the `commands::import::run` call.