
use builtin;
use str;

set edit:completion:arg-completer[bok] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'bok'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'bok'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand init 'Initializes a new book'
            cand node 'Adds, removes, or lists nodes'
            cand vis 'Visualizes the book'
            cand generate 'Generates the book in a specific format'
            cand lineedit 'Line edits a node'
            cand copyedit 'Copy edits a node'
            cand check 'Runs checks on the book'
            cand import 'Imports a qmd file'
            cand completion 'Generate shell completions'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;init'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;node'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand add 'Adds a new node'
            cand rm 'Removes a node'
            cand ls 'Lists the node hierarchy'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;node;add'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;node;rm'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;node;ls'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;node;help'= {
            cand add 'Adds a new node'
            cand rm 'Removes a node'
            cand ls 'Lists the node hierarchy'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;node;help;add'= {
        }
        &'bok;node;help;rm'= {
        }
        &'bok;node;help;ls'= {
        }
        &'bok;node;help;help'= {
        }
        &'bok;vis'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand d3 'Generates a d3 json file'
            cand mermaid 'Generates a mermaid diagram'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;vis;d3'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;vis;mermaid'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;vis;help'= {
            cand d3 'Generates a d3 json file'
            cand mermaid 'Generates a mermaid diagram'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;vis;help;d3'= {
        }
        &'bok;vis;help;mermaid'= {
        }
        &'bok;vis;help;help'= {
        }
        &'bok;generate'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand quarto 'Generates a quarto book'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;generate;quarto'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;generate;help'= {
            cand quarto 'Generates a quarto book'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;generate;help;quarto'= {
        }
        &'bok;generate;help;help'= {
        }
        &'bok;lineedit'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;copyedit'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;check'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;import'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;completion'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
        &'bok;help'= {
            cand init 'Initializes a new book'
            cand node 'Adds, removes, or lists nodes'
            cand vis 'Visualizes the book'
            cand generate 'Generates the book in a specific format'
            cand lineedit 'Line edits a node'
            cand copyedit 'Copy edits a node'
            cand check 'Runs checks on the book'
            cand import 'Imports a qmd file'
            cand completion 'Generate shell completions'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'bok;help;init'= {
        }
        &'bok;help;node'= {
            cand add 'Adds a new node'
            cand rm 'Removes a node'
            cand ls 'Lists the node hierarchy'
        }
        &'bok;help;node;add'= {
        }
        &'bok;help;node;rm'= {
        }
        &'bok;help;node;ls'= {
        }
        &'bok;help;vis'= {
            cand d3 'Generates a d3 json file'
            cand mermaid 'Generates a mermaid diagram'
        }
        &'bok;help;vis;d3'= {
        }
        &'bok;help;vis;mermaid'= {
        }
        &'bok;help;generate'= {
            cand quarto 'Generates a quarto book'
        }
        &'bok;help;generate;quarto'= {
        }
        &'bok;help;lineedit'= {
        }
        &'bok;help;copyedit'= {
        }
        &'bok;help;check'= {
        }
        &'bok;help;import'= {
        }
        &'bok;help;completion'= {
        }
        &'bok;help;help'= {
        }
    ]
    $completions[$command]
}
