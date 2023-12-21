# easy-neovim-pop-shell-nav

Quickly navigate pop-shell windows and Neovim splits with the same keybindings.

This utility was heavily inspired by 

* [easy-i3-neovim-nav](https://github.com/tom-anders/easy-i3-neovim-nav)

* [i3-vim-focus](https://github.com/jwilm/i3-vim-focus)

## Installation

```
cargo install --git https://github.com/erauer/easy-neovim-pop-shell-nav.git
```

## Usage

### Neovim configuration

Add this to your `init.lua`:

```
local servername = vim.api.nvim_get_vvar("servername")
vim.opt.title = true
vim.opt.titlestring = string.format("nvim %s -- [%s] ", vim.fn.getcwd(),
```

> **_Note:_** `easy-neovim-pop-shell-nav` uses the window's titlestring in order to extract the server name
used for communicating with Neovim. The default regex assumes that the servername is contained in
square brackets at the very end of your `titlestring`. 

### Pop-shell keybindings

Here are example keybindings for `Pop Shell`:

| Shortcut      | Command                           |
|---------------|-----------------------------------|
| `Super` + `h` | `easy-neovim-pop-shell-nav left`  |
| `Super` + `j` | `easy-neovim-pop-shell-nav down`  |
| `Super` + `k` | `easy-neovim-pop-shell-nav up`    |
| `Super` + `l` | `easy-neovim-pop-shell-nav right` |
| `Super` + `z` | `easy-neovim-pop-shell-nav open`  |

### Hyprland key bindings

```

bind = $mainMod, left, exec, easy-neovim-pop-shell-nav left --backend hyprland
bind = $mainMod, right, exec, easy-neovim-pop-shell-nav right --backend hyprland 
bind = $mainMod, up, exec, easy-neovim-pop-shell-nav up --backend hyprland 
bind = $mainMod, down, exec, easy-neovim-pop-shell-nav down --backend hyprland 
```

### Wayland usage

In order to use with Wayland, the [Window Calls Extended](https://extensions.gnome.org/extension/4974/window-calls-extended/) extension will need to be installed.

## Status

Let's be honest, at this point it `seems to work on my machine.`  I'm new to rust and by no means is the code production ready or of good quality. That being said, I'm open to issues and PRs and will continue to maintain this code as long as it is of use to me.


