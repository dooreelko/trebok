# Processing Ollama Streaming Input Line By Line

## Feature Specification

Ollama will stream dissection results as two-item one-line json arrays.
We should collect the line until the newline and pass it for node creation while continuing collecting the next line.
If there's an empty response with a thinking status like `{"model":"qwen3:4b","created_at":"2025-11-26T14:18:31.506114297Z","response":"","thinking":"Okay","done":false}` print the thought to the stderr with a newline but no carriage return.

## Decisions Taken

1.  **Line-by-line processing:** A buffer is used to accumulate incoming `resp.response` chunks. When a newline character is encountered, the accumulated line is extracted, parsed as a JSON array, and sent through the channel. The remaining content in the buffer is kept for the next chunk.
2.  **Handling "thinking" status:** When `resp.response` is empty, the entire `GenerationResponse` object is serialized back into a `serde_json::Value`. This `serde_json::Value` is then checked for a "thinking" field. If found, its string value is printed to `stderr`. This approach was chosen because the `ollama-rs` library's `GenerationResponse` struct does not directly expose the `thinking` field, and attempts to infer its structure from compiler errors were unsuccessful.

## Rejected Approaches

1.  **Direct access to `thinking` field:** Initially, it was assumed that `GenerationResponse` or `GenerationFinalResponseData` would have a direct `thinking` field. This was rejected after compilation errors indicated its absence.
2.  **Inferring `GenerationFinalResponseData` structure:** Multiple attempts were made to infer the structure of `GenerationFinalResponseData` based on compiler suggestions (e.g., `final_data.context.0.get("thinking")`, `final_data.0.get("thinking")`). These were rejected as they consistently led to compilation errors, indicating that the inferred structures were incorrect or incomplete without direct access to the `ollama-rs` source code.

## Implementation Details

The `dissect_markdown` function in `ollama.rs` was modified. A `buffer` of type `String` was introduced to accumulate partial lines from the Ollama stream. Inside the `while let Some(res) = stream.next().await` loop:
-   If `resp.response` is empty, the `resp` object is converted to a `serde_json::Value`. This `Value` is then checked for a "thinking" field. If present, its string content is printed to `stderr`.
-   Otherwise, `resp.response` is appended to the `buffer`.
-   A `while let Some(newline_pos) = buffer.find('\n')` loop then extracts complete lines from the buffer, parses them as `(String, String)` JSON arrays, and sends them through the `mpsc::channel`.
