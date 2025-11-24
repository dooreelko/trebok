# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_bok_global_optspecs
	string join \n h/help V/version
end

function __fish_bok_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_bok_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_bok_using_subcommand
	set -l cmd (__fish_bok_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c bok -n "__fish_bok_needs_command" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_needs_command" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_needs_command" -f -a "init" -d 'Initializes a new book'
complete -c bok -n "__fish_bok_needs_command" -f -a "node" -d 'Adds, removes, or lists nodes'
complete -c bok -n "__fish_bok_needs_command" -f -a "vis" -d 'Visualizes the book'
complete -c bok -n "__fish_bok_needs_command" -f -a "generate" -d 'Generates the book in a specific format'
complete -c bok -n "__fish_bok_needs_command" -f -a "lineedit" -d 'Line edits a node'
complete -c bok -n "__fish_bok_needs_command" -f -a "copyedit" -d 'Copy edits a node'
complete -c bok -n "__fish_bok_needs_command" -f -a "check" -d 'Runs checks on the book'
complete -c bok -n "__fish_bok_needs_command" -f -a "import" -d 'Imports a qmd file'
complete -c bok -n "__fish_bok_needs_command" -f -a "completion" -d 'Generate shell completions'
complete -c bok -n "__fish_bok_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand init" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand init" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand node; and not __fish_seen_subcommand_from add rm ls help" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand node; and not __fish_seen_subcommand_from add rm ls help" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand node; and not __fish_seen_subcommand_from add rm ls help" -f -a "add" -d 'Adds a new node'
complete -c bok -n "__fish_bok_using_subcommand node; and not __fish_seen_subcommand_from add rm ls help" -f -a "rm" -d 'Removes a node'
complete -c bok -n "__fish_bok_using_subcommand node; and not __fish_seen_subcommand_from add rm ls help" -f -a "ls" -d 'Lists the node hierarchy'
complete -c bok -n "__fish_bok_using_subcommand node; and not __fish_seen_subcommand_from add rm ls help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from add" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from rm" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from rm" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from ls" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from ls" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from help" -f -a "add" -d 'Adds a new node'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from help" -f -a "rm" -d 'Removes a node'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from help" -f -a "ls" -d 'Lists the node hierarchy'
complete -c bok -n "__fish_bok_using_subcommand node; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand vis; and not __fish_seen_subcommand_from d3 mermaid help" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand vis; and not __fish_seen_subcommand_from d3 mermaid help" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand vis; and not __fish_seen_subcommand_from d3 mermaid help" -f -a "d3" -d 'Generates a d3 json file'
complete -c bok -n "__fish_bok_using_subcommand vis; and not __fish_seen_subcommand_from d3 mermaid help" -f -a "mermaid" -d 'Generates a mermaid diagram'
complete -c bok -n "__fish_bok_using_subcommand vis; and not __fish_seen_subcommand_from d3 mermaid help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from d3" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from d3" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from mermaid" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from mermaid" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from help" -f -a "d3" -d 'Generates a d3 json file'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from help" -f -a "mermaid" -d 'Generates a mermaid diagram'
complete -c bok -n "__fish_bok_using_subcommand vis; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand generate; and not __fish_seen_subcommand_from quarto help" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand generate; and not __fish_seen_subcommand_from quarto help" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand generate; and not __fish_seen_subcommand_from quarto help" -f -a "quarto" -d 'Generates a quarto book'
complete -c bok -n "__fish_bok_using_subcommand generate; and not __fish_seen_subcommand_from quarto help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand generate; and __fish_seen_subcommand_from quarto" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand generate; and __fish_seen_subcommand_from quarto" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand generate; and __fish_seen_subcommand_from help" -f -a "quarto" -d 'Generates a quarto book'
complete -c bok -n "__fish_bok_using_subcommand generate; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand lineedit" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand lineedit" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand copyedit" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand copyedit" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand check" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand check" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand import" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand import" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand completion" -s h -l help -d 'Print help'
complete -c bok -n "__fish_bok_using_subcommand completion" -s V -l version -d 'Print version'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "init" -d 'Initializes a new book'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "node" -d 'Adds, removes, or lists nodes'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "vis" -d 'Visualizes the book'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "generate" -d 'Generates the book in a specific format'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "lineedit" -d 'Line edits a node'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "copyedit" -d 'Copy edits a node'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "check" -d 'Runs checks on the book'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "import" -d 'Imports a qmd file'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "completion" -d 'Generate shell completions'
complete -c bok -n "__fish_bok_using_subcommand help; and not __fish_seen_subcommand_from init node vis generate lineedit copyedit check import completion help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c bok -n "__fish_bok_using_subcommand help; and __fish_seen_subcommand_from node" -f -a "add" -d 'Adds a new node'
complete -c bok -n "__fish_bok_using_subcommand help; and __fish_seen_subcommand_from node" -f -a "rm" -d 'Removes a node'
complete -c bok -n "__fish_bok_using_subcommand help; and __fish_seen_subcommand_from node" -f -a "ls" -d 'Lists the node hierarchy'
complete -c bok -n "__fish_bok_using_subcommand help; and __fish_seen_subcommand_from vis" -f -a "d3" -d 'Generates a d3 json file'
complete -c bok -n "__fish_bok_using_subcommand help; and __fish_seen_subcommand_from vis" -f -a "mermaid" -d 'Generates a mermaid diagram'
complete -c bok -n "__fish_bok_using_subcommand help; and __fish_seen_subcommand_from generate" -f -a "quarto" -d 'Generates a quarto book'
