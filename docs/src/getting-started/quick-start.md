---
date_created: 2025-10-03T02-46-38
date_updated: 2026-05-30
title: quick-start
---
# Quick Start

Create a managed folder:

```bash
mkdir -p ~/dotfiles
forge init --name dotfiles --dir ~/dotfiles
```

Stage and link files:

```bash
forge stage ~/.vimrc ~/.bashrc
forge link
forge list
```

Stage directories:

```bash
forge stage --recursive ~/.config/nvim
forge stage --depth 2 ~/.config/git
```

Undo staging or linking:

```bash
forge unstage ~/.vimrc
forge unlink --yes ~/.bashrc
```

Create a portable pack:

```bash
forge start packing my_dotfiles
forge pack --scope my_dotfiles ~/.vimrc ~/.bashrc
forge pack --scope my_dotfiles --recursive ~/.config/nvim
forge check --scope my_dotfiles
forge seal --scope my_dotfiles
```

Preview and install a pack:

```bash
forge explain .forge/archives/my_dotfiles-2026-05-30.zip
forge install .forge/archives/my_dotfiles-2026-05-30.zip --dry-run
forge install .forge/archives/my_dotfiles-2026-05-30.zip --target ~/restored-config
```
