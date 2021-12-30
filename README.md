<div align="center">

# vsh
A blazingly fast shell made in Rust 🦀

</div>

This is a shell made for the sophisticated power user. Far from finished yet but the main motive is to create a hyperextensible shell with all the features of previous generations added with tools geared towards improving workflows.

## Installation

Copy and paste the following command and choose the appropriate installation method for you. You can remove the `install.py` file afterwards
```sh
wget https://raw.githubusercontent.com/Vaimer9/vsh/main/install.py
python3 install.py
```


## Customization
Once you run vsh for the first time it automatically creates `.vshrc.toml`.
All the customization possible is written there.
Even then here is the text
```toml
# This is the config file for vsh. For now you can only edit the Prompt styling here

# Prompt
# These are the default values
# [prompt]

# Whether the prompt is single-lined or double lined
# double = false

# Prompt Background
# Highest value can be 255, anything above that will result in an error.
# If there are mroe than 3 elements that will also result in an error
# color = [115, 147, 179]

# Prompt Text Background
# Same as Prompt Background just that its for Text
# text_color = [33, 33, 33]

# Prompt Character
# The reason I included this is because it gives some cusomization to Classic Prompt users
# The character used at the start of prompt
# Doesn't have to be a char, can also be a String
# promptchar = "λ"

# Style
# Two option: Modern, Classic
# Modern requires you to have nerd fonts and you can change Background color
# Classic has the regular fonts.
# style = "classic"


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
- [ ] Piping
- [ ] Command Completion
- [ ] `vsh` Scripting language :eyes:
- [ ] Custom `ls` command
- [ ] Intergration with `git`, `node` and `cargo`
- [ ] Customization via `.vshrc`
- [ ] Plugin Support (Yikes!)

## License

Licensed under a Mozilla Public License.

For more information take a look at the license [here](./LICENSE).


