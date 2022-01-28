<div align="center">

# vsh
A blazingly fast shell made in Rust 🦀

</div>

This is a shell made for the sophisticated power user. Far from finished yet but the main motive is to create a hyperextensible shell with all the features of previous generations added with tools geared towards improving workflows.

## Installation

#### 1. Install via vsh-vm

vsh-vm is the version manager for vsh. You can install it with:
```
wget https://github.com/Vaimer9/vsh-vm/releases/latest/download/vsh-vm
vsh-vm --help
```
You will have to run it as root because it installs vsh into the `/bin` directory.
Dont worry it won't remove your entire kernel, you can check the source code [here](https://github.com/xmantle/vsh-vm)

#### 2. Manual Install

You can choose the versions in [releases page](https://github.com/xmantle/vsh/releases)

## Customization
Once you run vsh for the first time it automatically creates `.vshrc.toml`.
All the customization possible is written there.
Even then here is the text
```toml
[prompt]

double = false
color = [115, 147, 179]
text_color = [33, 33, 33]
promptchar = "λ"
style = "classic"

[misc]
alias = [
	["", ""]
]
```


### Example Config file
This is the config file personally used by me:
```toml
[prompt]
style = "Modern"
color = [33, 33, 33]
text_color = [115, 147, 179]
double = false
```
As you can see customizing is not that hard and doesn't require too much code. I will be adding more and more into the level of customization possible!

## Roadmap

- [x] Proper Prompt
- [x] Run commands
- [x] Exit with Ctrl+C & Ctrl+D via Rustyline
- [x] Good looking prompt
- [x] Multiple Commands
- [x] Command History
- [x] Prompt Customization
- [x] Install Script
- [ ] `ctrl` + `z` functionality (i.e Being able to run processes in the background)
- [ ] Piping
- [ ] Command Completion
- [ ] `vsh` Scripting language :eyes:
- [ ] Custom `ls` command
- [ ] Intergration with `git`, `node` and `cargo`
- [x] Customization via `.vshrc`
- [ ] Plugin Support (Yikes!)

See [projects](https://github.com/xmantle/vsh/projects/1) for more
## License

Licensed under a Mozilla Public License.

For more information take a look at the license [here](./LICENSE).


