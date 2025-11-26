== BOK ==

Here's the idea. I'm not thinking in a linear manner and (hypothesis) I think this hinders my attempts to write my book. The idea is to collect the somewhat random graph of thoughts and generate a linear book from that using relationships between the nodes of the graph.

For that we can create a recursive mind-map-like graph system, where each node  represents and idea or a concept and can have its own sub-graph of concepts that will elaborate the parent concept further.

Each node will have some markdown text (qmd) associated with it and will have 
a metadata file (as a hocon document) describing its relationships (parent, non-hierarchical relations) and 
own attributes(ordering).

Each node will have a unique id (Murmur3).
Each node will store all its data in a directory. The name of the directory will be its
unique id, followed by space and a short title.

The tool should:
- allow adding and removing nodes
- visualize chosen level of nodes (1+) (generate json that can be consumed by D3, generate mermaid or dot diagram)
- generate quarto book based on the graph. only the chapters - index, cover page, 
styles etc will be provided by the user
- use local (through ollama) or remote LLM for 
	- stylistic consistency, line and copy editing
	- check chapter completness
	- detection of duplicates/similar
- analyse existing qmd files and suggest and split them into individual nodes
- eventually allow for creation of a non-linear book, where one can start reading at 
an arbitrary point of interest and continue either by going deeper or by going to a
related ideas


== Implementation ==
- Main cli, called bok written in Rust
- vscode extension written in typescript that interacts with the cli

=== CLI functions ===

- bok init # creates book.conf in hocon format containing book metadata and starting node
- bok node add <short blurb>
- bok vis d3
- bok vis mermaid
- bok generate quarto
- bok lineedit <node>
- bok copyedit <node>
- bok check # run checks
- bok import <qmd file>