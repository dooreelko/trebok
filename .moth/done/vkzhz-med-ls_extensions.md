# Ls Extensions

## Feature Specification
The `bok node ls` command should:
1. Include the node hash/ID before the blurb in the output
2. Respect the "after" ordering attribute when displaying sibling nodes

## Problem Analysis
The original `ls` command implementation:
1. Only displayed node blurbs without their IDs, making it difficult to reference specific nodes
2. Sorted nodes alphabetically by path, ignoring the "after" attribute in meta.hocon files that specifies ordering relationships between sibling nodes

## Decisions Taken
- **Output format**: Display node ID followed by a space and then the blurb (e.g., "185236344 Starting Node")
- **Ordering algorithm**: Implement topological sort based on "after" attributes
  - Nodes without "after" attribute can be placed first
  - Nodes with "after" attribute must come after the referenced node
  - Handle circular dependencies or missing references gracefully by adding remaining nodes in their current order
- **Node struct extension**: Add optional "after" field to Node struct to store the ordering relationship
- **Meta.hocon parsing**: Read the "after" attribute from meta.hocon files during node traversal

## Decisions Rejected
- Alphabetical sorting: rejected because it doesn't respect the semantic ordering specified by users through "after" attributes
- Sorting only at root level: rejected because child nodes at any level should respect their "after" ordering
- Strict validation of "after" references: rejected in favor of graceful handling (nodes with missing/circular references are still displayed)

## Implementation Details
1. **Node struct update**: Added `after: Option<String>` field to the Node struct in bok/src/commands/node.rs
2. **Output modification**: Updated `print_nodes_recursive()` and `ls()` functions to include node ID in output format
3. **Meta.hocon reading**: Modified `get_nodes_recursive()` to parse the "after" attribute from meta.hocon files using HoconLoader
4. **Topological sorting**: Implemented `sort_by_after_attribute()` function that performs topological sort:
   - Iteratively places nodes whose dependencies (after references) are already placed
   - Continues until all nodes are placed
   - Gracefully handles circular dependencies and missing references by adding remaining nodes when no progress can be made

## Testing
Verified with example graph in ./play directory:
- Node IDs are correctly displayed before blurbs
- Nodes with "after" attributes appear after their referenced nodes
- Tested chained "after" relationships (A → B → C → D) - ordering correctly maintained
- Verified graceful handling of missing references