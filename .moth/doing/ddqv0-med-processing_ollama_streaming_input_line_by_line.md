## Feature Specification: Processing Ollama Streaming Input Line By Line
Ollama will stream dissection results as two-item one-line json arrays.

First, we will use plain reqwest for ollama. Here's an example call that we need to implement:
```bash
curl -X POST http://localhost:11434/api/generate -d  "$(jq --arg doc "$(cat ../IDEA.md)" --null-input '{"model": "qwen3:4b", "think": false, "stream": true, "prompt": ("You are a helpful assistant that dissects markdown content into its smallest atomic semantic units. For each unit, provide a short blurb (up to 50 characters) in form of "subject verb characteristic" and the full content of the unit. For example for text 'Here's the idea. I'm not thinking in a linear manner and (hypothesis) I think this hinders my attempts to write my book. The idea is to create a recursive mind-map-like graph system, where each node  represents and idea or a concept and can have its own sub-graph of concepts that will elaborate the parent concept further.' the blurb could be 'non-linear thinking requires specialized tooling'. For each unit respond with a one-line JSON array that contains two strings: ["blurb", "dissected_unit_content"] followed by a new line. Do not include any other text in your response, just one two-item array per line." + $doc)}')"
```

Then we should read streaming responses line by line and treat each line as a two-item array specified above.

if there's an empty response with a thinking satus like `{"model":"qwen3:4b","created_at":"2025-11-26T14:18:31.506114297Z","response":"","thinking":"Okay","done":false}` print the thought to the stderr with 

### Decisions Taken:

1.  **HTTP Client**: `reqwest` was chosen for making asynchronous HTTP requests to the Ollama API.
2.  **Asynchronous Execution**: Due to the synchronous nature of the `LlmProvider::dissect_markdown` trait method and the asynchronous streaming nature of the Ollama API, a `tokio` runtime was created within the `dissect_markdown` function to block on the asynchronous HTTP call and stream processing. This allows the function to conform to the synchronous trait while handling asynchronous operations internally.
3.  **Streaming Response Handling**: Instead of using `tokio_stream::wrappers::LinesStream` (which expects an `AsyncBufRead` and caused type mismatch issues), a custom line processing logic was implemented. This involves:
    *   Reading the `reqwest` response as a stream of `bytes::Bytes` chunks.
    *   Maintaining a `String` buffer to accumulate partial lines across chunks.
    *   Iteratively searching for newline characters (`
`) in the buffer.
    *   Processing complete lines (JSON parsing) and removing them from the buffer.
    *   Retaining any remaining partial content in the buffer for the next chunk.
    *   Processing any final content in the buffer after the stream ends.
4.  **Ollama API Request Structure**: The request body was constructed to match the Ollama `/api/chat` endpoint, including `model`, `stream: true`, and `messages` with `system` and `user` prompts.
5.  **Ollama API Response Parsing**: Each line from the streaming response is parsed as a `GenerateResponse` struct.
    *   If a `thinking` message is present, it is printed to `stderr`.
    *   If a `response` content is present, it is further parsed as a JSON array `["blurb", "dissected_unit_content"]`.
6.  **Model Name Retrieval**: The `LlmSettings.model` field was confirmed to be a `String` (not `Option<String>`). The `model_name` for the request is directly obtained using `self.settings.model.as_str()`.
7.  **Return Type**: The processed `(blurb, content)` tuples are collected into a `Vec<Result<(String, String)>>` and then returned as a `Box<dyn Iterator<Item = Result<(String, String)>> + 'a>` to match the `LlmProvider` trait signature.

### Decisions Rejected:

1.  **Modifying `LlmProvider` Trait**: Changing the `LlmProvider` trait to return an asynchronous stream was considered but rejected as it would require explicit user permission and was outside the immediate scope of implementing the feature within the existing trait constraints.
2.  **Blocking on Async Calls without `tokio`**: Directly blocking on `async` calls without a proper `tokio` runtime was rejected due to potential deadlocks and performance issues.
3.  **Collecting all results into a `Vec` before processing**: While the final output is an iterator over a `Vec`, the internal processing of the stream is done line-by-line to adhere to the "streaming input line by line" requirement as much as possible within the synchronous trait constraint.

### Implementation Details (Abstract):

The `dissect_markdown` function in `OllamaProvider` now performs the following steps:
1.  Initializes an `reqwest::Client` and constructs the Ollama API endpoint URL.
2.  Prepares the `ChatRequest` payload, including the system prompt, user prompt (with the provided markdown content), and the model name from `LlmSettings`.
3.  Spawns a `tokio` runtime to execute an asynchronous block.
4.  Within the asynchronous block, it sends a POST request to the Ollama API.
5.  It then processes the streaming response body:
    *   A `String` buffer is used to accumulate incoming bytes and extract complete lines.
    *   Each extracted line is deserialized into a `GenerateResponse` struct.
    *   "Thinking" messages are printed to `stderr`.
    *   Response content (expected to be a JSON array `["blurb", "content"]`) is parsed, and the extracted `(blurb, content)` pairs are added to a `Vec`.
6.  After the stream concludes, any remaining content in the buffer is processed.
7.  Finally, the function returns a `Box<dyn Iterator>` over the collected `Vec` of `Result<(String, String)>`.