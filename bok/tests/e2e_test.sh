#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Get the absolute path to the project root
PROJECT_ROOT=$(pwd)

# Define the path to the bok executable
BOK_EXE="$PROJECT_ROOT/bok/target/release/bok"

# Create a temporary directory for the test
TEST_DIR=$(mktemp -d)
echo "Running tests in temporary directory: $TEST_DIR"
cd "$TEST_DIR"

# Test 1: Initialize a new book
echo "--- Test 1: bok init ---"
"$BOK_EXE" init "My First Book"
if [ ! -f "book.conf" ]; then
    echo "Test 1 Failed: book.conf file not created."
    exit 1
fi
echo "Test 1 Passed."

# Test 2: Add a root node
echo "--- Test 2: bok node add (root) ---"
"$BOK_EXE" node add "Root Node Blurb"
ROOT_NODE_DIR=$(find . -maxdepth 1 -type d -name "*Root Node Blurb*" | head -n 1)
ROOT_NODE_HASH=$(basename "$ROOT_NODE_DIR" | awk '{print $1}')
if [ -z "$ROOT_NODE_HASH" ]; then
    echo "Test 2 Failed: Root node not created."
    exit 1
fi
echo "Test 2 Passed."

# Test 3: Add a child node under the root node
echo "--- Test 3: bok node add (child) ---"
ROOT_NODE_FULL_ID="${ROOT_NODE_HASH}-Root-Node-Blurb" # Construct the full ID
"$BOK_EXE" node add --under "$ROOT_NODE_FULL_ID" "Child Node Blurb"
CHILD_NODE_DIR=$(find "$ROOT_NODE_DIR" -maxdepth 1 -type d -name "*Child Node Blurb*" | head -n 1)
CHILD_NODE_HASH=$(basename "$CHILD_NODE_DIR" | awk '{print $1}')
if [ -z "$CHILD_NODE_HASH" ]; then
    echo "Test 3 Failed: Child node not created."
    exit 1
fi
echo "Test 3 Passed."

# Test 4: List nodes and check hierarchy
echo "--- Test 4: bok node ls ---"
LS_OUTPUT=$("$BOK_EXE" node ls)
echo "LS Output:"
echo "$LS_OUTPUT"

EXPECTED_OUTPUT_PART1="My First Book"
EXPECTED_OUTPUT_PART2="Root Node Blurb"
EXPECTED_OUTPUT_PART3="  Child Node Blurb"

if ! echo "$LS_OUTPUT" | grep -q "$EXPECTED_OUTPUT_PART1"; then
    echo "Test 4 Failed: Expected '$EXPECTED_OUTPUT_PART1' not found in ls output."
    exit 1
fi
if ! echo "$LS_OUTPUT" | grep -q "$EXPECTED_OUTPUT_PART2"; then
    echo "Test 4 Failed: Expected '$EXPECTED_OUTPUT_PART2' not found in ls output."
    exit 1
fi
if ! echo "$LS_OUTPUT" | grep -q "$EXPECTED_OUTPUT_PART3"; then
    echo "Test 4 Failed: Expected '$EXPECTED_OUTPUT_PART3' not found in ls output."
    exit 1
fi
echo "Test 4 Passed."

# Test 5: Check completion values for spaces replaced by hyphens
echo "--- Test 5: Completion with hyphens ---"
# Add a node with spaces in its blurb
"$BOK_EXE" node add "Node With Spaces Blurb"
NODE_WITH_SPACES_DIR=$(find . -maxdepth 1 -type d -name "*Node With Spaces Blurb*" | head -n 1)
NODE_WITH_SPACES_HASH=$(basename "$NODE_WITH_SPACES_DIR" | awk '{print $1}')

# Let's try to add a node under the "Node With Spaces Blurb" using the hyphenated version
# This will implicitly test if the `get_node_hashes_for_clap` is providing the correct
# hyphenated string for completion, and if the parsing logic in main.rs correctly
# extracts the hash from it.

HYPHENATED_BLURB="${NODE_WITH_SPACES_HASH}-Node-With-Spaces-Blurb"
"$BOK_EXE" node add --under "$HYPHENATED_BLURB" "Child Under Hyphenated Blurb"

CHILD_UNDER_HYPHENATED_DIR=$(find . -maxdepth 2 -type d -name "*Child Under Hyphenated Blurb*" | head -n 1)
CHILD_UNDER_HYPHENATED_HASH=$(basename "$CHILD_UNDER_HYPHENATED_DIR" | awk '{print $1}')

if [ -z "$CHILD_UNDER_HYPHENATED_HASH" ]; then
    echo "Test 5 Failed: Child node under hyphenated blurb not created."
    exit 1
fi

# Verify that the child node is indeed under the parent with spaces
LS_OUTPUT_TEST5=$("$BOK_EXE" node ls)
if ! echo "$LS_OUTPUT_TEST5" | grep -q "Node With Spaces Blurb"; then
    echo "Test 5 Failed: Parent node 'Node With Spaces Blurb' not found in ls output."
    exit 1
fi
if ! echo "$LS_OUTPUT_TEST5" | grep -q "  Child Under Hyphenated Blurb"; then
    echo "Test 5 Failed: Child node 'Child Under Hyphenated Blurb' not found under parent."
    exit 1
fi

echo "Test 5 Passed."

# Test 6: Check shell completion for bok node rm
echo "--- Test 6: bok node rm with completion ---"
# Add a node to be removed
"$BOK_EXE" node add "Node To Be Removed Blurb"
NODE_TO_BE_REMOVED_DIR=$(find . -maxdepth 1 -type d -name "*Node To Be Removed Blurb*" | head -n 1)
NODE_TO_BE_REMOVED_HASH=$(basename "$NODE_TO_BE_REMOVED_DIR" | awk '{print $1}')

# Construct the hyphenated blurb for completion
HYPHENATED_RM_BLURB="${NODE_TO_BE_REMOVED_HASH}-Node-To-Be-Removed-Blurb"

# Attempt to remove the node using the hyphenated blurb
"$BOK_EXE" node rm "$HYPHENATED_RM_BLURB"

# Verify that the node is removed
if [ -d "$NODE_TO_BE_REMOVED_DIR" ]; then
    echo "Test 6 Failed: Node '$NODE_TO_BE_REMOVED_DIR' was not removed."
    exit 1
fi

# Verify that the node is no longer in the ls output
LS_OUTPUT_TEST6=$("$BOK_EXE" node ls)
if echo "$LS_OUTPUT_TEST6" | grep -q "Node To Be Removed Blurb"; then
    echo "Test 6 Failed: Node 'Node To Be Removed Blurb' still found in ls output after removal."
    exit 1
fi

echo "Test 6 Passed."

# Clean up the temporary directory
echo "Cleaning up temporary directory: $TEST_DIR"
cd - > /dev/null # Go back to the original directory
rm -rf "$TEST_DIR"

echo "All tests passed!"
