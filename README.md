# Trebok

This project is a set of tools for management of recursive mind-map ideas with the eventual goal of generation of books and other more ordered media.

## Shell Completion

To enable shell completion for `bok`, follow the instructions for your shell:

### Bash

To enable completion for Bash, add the following to your `~/.bashrc` or `~/.bash_profile`:

```bash
source /home/doo/projects/trebok/bok/completions/bok.bash
```

Alternatively, you can move the completion file to your Bash completions directory:

```bash
sudo mv /home/doo/projects/trebok/bok/completions/bok.bash /etc/bash_completion.d/
```

Or, to source the completion script directly without saving it to a file:

```bash
source <(/home/doo/projects/trebok/bok/target/release/bok completion bash)
```

### Zsh

To enable completion for Zsh, add the following to your `~/.zshrc`:

```zsh
fpath=(/home/doo/projects/trebok/bok/completions $fpath)
autoload -Uz compinit
compinit
```

Alternatively, you can move the completion file to one of your `fpath` directories.

### Fish

To enable completion for Fish, move the completion file to `~/.config/fish/completions/`:

```fish
mkdir -p ~/.config/fish/completions
cp /home/doo/projects/trebok/bok/completions/bok.fish ~/.config/fish/completions/
```

### PowerShell

To enable completion for PowerShell, add the following to your PowerShell profile (you can find its path by running `$PROFILE`):

```powershell
. /home/doo/projects/trebok/bok/completions/_bok.ps1
```

### Elvish

To enable completion for Elvish, add the following to your `~/.elvish/rc.elv`:

```elvish
edit:completion:add-path /home/doo/projects/trebok/bok/completions
```