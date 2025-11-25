# Trebok

This project is a set of tools for management of recursive mind-map ideas with the eventual goal of generation of books and other more ordered media.

## Shell Completion

Generate code completions using `bok completion`.

To enable shell completion for `bok`, follow the instructions for your shell:

### Bash

To enable completion for Bash, add the following to your `~/.bashrc` or `~/.bash_profile`:

```bash
source bok.bash
```

Alternatively, you can move the completion file to your Bash completions directory:

```bash
sudo mv bok.bash /etc/bash_completion.d/
```

Or, to source the completion script directly without saving it to a file:

```bash
source <(bok completion bash)
```

### Zsh

To enable completion for Zsh, add the following to your `~/.zshrc`:

```zsh
fpath=(<path to completions directory> $fpath)
autoload -Uz compinit
compinit
```

Alternatively, you can move the completion file to one of your `fpath` directories.

