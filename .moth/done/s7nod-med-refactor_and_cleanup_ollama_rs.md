## Feature Specification

Refactor and cleanup `bok/src/llm_providers/ollama.rs` to improve code clarity and remove pollution:

1. Use BufReader for cleaner HTTP stream reading line by line
2. Organize imports following Rust conventions
3. Remove unused struct fields
4. Define and implement AsyncIterator trait for type-safe async iteration
5. Refactor LlmProvider and OllamaProvider to use AsyncIterator instead of returning raw streams

### AsyncIterator Design

```rust
trait AsyncIterator {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}
```

Both LlmProvider and OllamaProvider implement this trait, providing a uniform interface for async iteration.

### Ollama Streaming Protocol

The Ollama server responds with a stream of `GenerateResponse` JSON objects, one per line. Each response contains:
- `response`: Optional string chunk of the LLM output
- `done`: Boolean flag (removed as unused in our implementation)

The implementation accumulates `response` chunks in a buffer until a complete line (terminated by `\n` or `\r`) is received. Each complete line is a JSON array `["blurb", "content"]` which is parsed and returned as `(String, String)`.

## Implementation Details

### Import Organization
- Organized imports in standard Rust order: std library, external crates, then local crate imports
- Removed unused `serde_json::Value` import
- Grouped imports logically with blank lines separating sections

### Struct Cleanup
- Removed unused `done` field from `GenerateResponse` struct as it wasn't used in the streaming logic

### OllamaStream::next() Implementation
The async iterator accumulates response chunks from the Ollama API stream:
1. Reads GenerateResponse JSON objects line by line using BufReader::read_line()
2. Accumulates response text chunks in an internal buffer
3. When a chunk contains a newline character, extracts the complete line from the buffer
4. Parses the complete line as a JSON array `[String, String]`
5. Returns the tuple and continues accumulating for the next item

The implementation properly handles:
- Borrow checker constraints by creating owned String before draining the buffer
- EOF detection (returns None when stream ends)
- JSON parsing errors with descriptive error messages
- Both `\n` and `\r` as line terminators

