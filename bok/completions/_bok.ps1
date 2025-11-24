
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'bok' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'bok'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'bok' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initializes a new book')
            [CompletionResult]::new('node', 'node', [CompletionResultType]::ParameterValue, 'Adds, removes, or lists nodes')
            [CompletionResult]::new('vis', 'vis', [CompletionResultType]::ParameterValue, 'Visualizes the book')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generates the book in a specific format')
            [CompletionResult]::new('lineedit', 'lineedit', [CompletionResultType]::ParameterValue, 'Line edits a node')
            [CompletionResult]::new('copyedit', 'copyedit', [CompletionResultType]::ParameterValue, 'Copy edits a node')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Runs checks on the book')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Imports a qmd file')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;init' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;node' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Adds a new node')
            [CompletionResult]::new('rm', 'rm', [CompletionResultType]::ParameterValue, 'Removes a node')
            [CompletionResult]::new('ls', 'ls', [CompletionResultType]::ParameterValue, 'Lists the node hierarchy')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;node;add' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;node;rm' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;node;ls' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;node;help' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Adds a new node')
            [CompletionResult]::new('rm', 'rm', [CompletionResultType]::ParameterValue, 'Removes a node')
            [CompletionResult]::new('ls', 'ls', [CompletionResultType]::ParameterValue, 'Lists the node hierarchy')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;node;help;add' {
            break
        }
        'bok;node;help;rm' {
            break
        }
        'bok;node;help;ls' {
            break
        }
        'bok;node;help;help' {
            break
        }
        'bok;vis' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('d3', 'd3', [CompletionResultType]::ParameterValue, 'Generates a d3 json file')
            [CompletionResult]::new('mermaid', 'mermaid', [CompletionResultType]::ParameterValue, 'Generates a mermaid diagram')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;vis;d3' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;vis;mermaid' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;vis;help' {
            [CompletionResult]::new('d3', 'd3', [CompletionResultType]::ParameterValue, 'Generates a d3 json file')
            [CompletionResult]::new('mermaid', 'mermaid', [CompletionResultType]::ParameterValue, 'Generates a mermaid diagram')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;vis;help;d3' {
            break
        }
        'bok;vis;help;mermaid' {
            break
        }
        'bok;vis;help;help' {
            break
        }
        'bok;generate' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('quarto', 'quarto', [CompletionResultType]::ParameterValue, 'Generates a quarto book')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;generate;quarto' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;generate;help' {
            [CompletionResult]::new('quarto', 'quarto', [CompletionResultType]::ParameterValue, 'Generates a quarto book')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;generate;help;quarto' {
            break
        }
        'bok;generate;help;help' {
            break
        }
        'bok;lineedit' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;copyedit' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;check' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;import' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;completion' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'bok;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initializes a new book')
            [CompletionResult]::new('node', 'node', [CompletionResultType]::ParameterValue, 'Adds, removes, or lists nodes')
            [CompletionResult]::new('vis', 'vis', [CompletionResultType]::ParameterValue, 'Visualizes the book')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generates the book in a specific format')
            [CompletionResult]::new('lineedit', 'lineedit', [CompletionResultType]::ParameterValue, 'Line edits a node')
            [CompletionResult]::new('copyedit', 'copyedit', [CompletionResultType]::ParameterValue, 'Copy edits a node')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Runs checks on the book')
            [CompletionResult]::new('import', 'import', [CompletionResultType]::ParameterValue, 'Imports a qmd file')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'bok;help;init' {
            break
        }
        'bok;help;node' {
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Adds a new node')
            [CompletionResult]::new('rm', 'rm', [CompletionResultType]::ParameterValue, 'Removes a node')
            [CompletionResult]::new('ls', 'ls', [CompletionResultType]::ParameterValue, 'Lists the node hierarchy')
            break
        }
        'bok;help;node;add' {
            break
        }
        'bok;help;node;rm' {
            break
        }
        'bok;help;node;ls' {
            break
        }
        'bok;help;vis' {
            [CompletionResult]::new('d3', 'd3', [CompletionResultType]::ParameterValue, 'Generates a d3 json file')
            [CompletionResult]::new('mermaid', 'mermaid', [CompletionResultType]::ParameterValue, 'Generates a mermaid diagram')
            break
        }
        'bok;help;vis;d3' {
            break
        }
        'bok;help;vis;mermaid' {
            break
        }
        'bok;help;generate' {
            [CompletionResult]::new('quarto', 'quarto', [CompletionResultType]::ParameterValue, 'Generates a quarto book')
            break
        }
        'bok;help;generate;quarto' {
            break
        }
        'bok;help;lineedit' {
            break
        }
        'bok;help;copyedit' {
            break
        }
        'bok;help;check' {
            break
        }
        'bok;help;import' {
            break
        }
        'bok;help;completion' {
            break
        }
        'bok;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
