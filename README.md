<div align="center">

# vsh

A blazingly fast shell made in Rust 🦀

</div>

This is a shell made for the sophisticated power user. Far from finished yet, but the main motive is to create a hyperextensible shell with all the features of previous generations added with tools geared towards improving your workflow.

## Installation

#### 1. Install via wget

Install via running the following commands

```
sudo wget https://github.com/vaimer9/vsh/releases/latest/download/vsh -q -P /bin
sudo chmod +x /bin/vsh
```

#### 2. Manual Install

1. Clone the Repository
2. Make sure you have `rustup` installed
3. Make sure you have `build-essential` installed
4. Go into the cloned repository and run `make install`

vsh will be installed in `/bin` directory by default, you can change this via changing line #11 in `Makefile`

## Customization

On first run, vsh will automatically create `.vshrc.toml`.
Any customization is done there.
Here is the default config:

```toml
[prompt]
theme="&[#7393B3]`[`{{current_dir}}`] `"

[misc]
alias = [
	["", ""]
]

[effects]
underlined = false
bold = true
dimmed = false
suggestion_color = "red"

truecolors = false
true_suggestion_color = [255, 0, 0]
```

### Example Config file

This is my personal config, as an example:

```toml
[prompt]
theme="&[#7393B3]`[`{{current_dir}}`] `"
```

As you can see customizing isn't that hard and doesn't require too much code. I will be improving the level of customization possible! See [this documentation for more info on themes](https://github.com/Vaimer9/vsh/blob/main/docs/THEME.md)

## Roadmap

- [x] Proper Prompt
- [x] Run commands
- [x] Exit with Ctrl+C & Ctrl+D via Rustyline
- [x] Good looking prompt
- [x] Multiple Commands
- [x] Command History
- [x] Prompt Customization
- [ ] `ctrl` + `z` functionality (i.e Being able to run processes in the background)
- [ ] Piping
- [x] Command Completion
- [ ] `vsh` Scripting language :eyes:
- [ ] Custom `ls` command
- [ ] Intergration with `git`, `node` and `cargo`
- [x] Customization via `.vshrc`
- [ ] Plugin Support (Yikes!)

See [projects](https://github.com/xmantle/vsh/projects/1) for more

## License

Licensed under a Mozilla Public License.

For more information take a look at the license [here](./LICENSE).
