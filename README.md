<div align="center">

# vsh

A blazingly fast shell made in Rust 🦀

</div>

This is a shell made for the sophisticated power user. Far from finished yet but the main motive is to create a hyperextensible shell with all the features of previous generations added with tools geared towards improving workflows.

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
3. Make sure you have `build-essentials`
4. Go into the repository and run `make install`

vsh will be installed in `/bin` directory, you can change this via changing line #11 in `Makefile` on your machine

## Customization

Once you run vsh for the first time it automatically creates `.vshrc.toml`.
All the customization possible is written there.
Even then here is the text

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

This is the config file personally used by me:

```toml
[prompt]
theme="&[#7393B3]`[`{{current_dir}}`] `"
```

As you can see customizing is not that hard and doesn't require too much code. I will be adding more and more into the level of customization possible! See [this documentation for more infos on themes](https://github.com/Vaimer9/vsh/blob/main/docs/THEME.md)

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
