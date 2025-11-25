we need a node lister component that:
- will recursively find all nodes
- provide the node collection for the hierarchical representation of `bok node ls`
- provide the node collection as a flat list of ids and blurbs for shell completion

## Implementation Details:

### Node Lister Component

The node lister component was implemented to fulfill two primary requirements: providing a hierarchical view of nodes for `bok node ls` and a flat list of node identifiers for shell completion.

1.  **Hierarchical Representation:**
    *   A `Node` struct was introduced to encapsulate a node's ID, blurb, and its children, forming a tree-like structure.
    *   A recursive function (`get_nodes_recursive`) was developed to traverse the file system, identify nodes based on their directory structure and `meta.hocon` files, and construct this hierarchical `Node` structure.
    *   The `ls()` command was refactored to utilize this `get_nodes_recursive` function to build the node tree and then a separate recursive function (`print_nodes_recursive`) to format and print the hierarchy to the console, respecting a starting node defined in `book.conf`.

2.  **Flat List for Shell Completion:**
    *   A function (`get_all_nodes_flat`) was implemented to recursively collect all node IDs and their blurbs into a flat list of `(ID, Blurb)` tuples. This serves as the source for shell completion candidates.

### Shell Completion for `--under` argument

The integration with `clap_complete` for shell completion of the `--under` argument in `bok node add` presented a challenge regarding values containing spaces.

*   **Problem:** Initially, the goal was to display "hash - blurb" as a single selectable completion candidate. However, `clap_complete`'s mechanism for generating bash completion scripts, which relies on `compgen -W`, inherently splits values containing spaces into multiple tokens. Attempts to quote the values or use `ValueHint::CommandString` did not alter this behavior.
*   **Decision:** Due to the limitations of `clap_complete` and bash's `compgen -W` command, for shell completion, replace spaces with hyphens and when an id is passed to commands and contains hyphens, take only the part before the first hyphen.
*   **Parsing:** The `main.rs` command parsing logic for `NodeAction::Add` was updated to extract only the hash from the (now hash-only) completion value, ensuring correct functionality.

This approach provides functional shell completion while respecting the technical limitations and project guidelines.