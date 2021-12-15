<div align="center">
<h1>vsh</h1>
A Blazingly fast shell made in Rust 🦀
</div>

#### Why make another shell?

[Nushell](https://github.com/nushell/nushell/) which is the current leading rust shell, is very opinionated. It brings a lot to the table for someone who just wants a shell but in rust. Namely, a new `ls` command, new scripting experiece etc. What is missing is just bash written in rust and vsh is here to deliver that. The planned scripting language will be a interchangable with bash and all of its features so that people don't feel *homesick* or dropped into a new space when they start using vsh. As the for shell prompt I plan to expand upon it to add plugins to accept a wide array of custom plugins all written in vsh. Till then feel free to contribute yourself!

#### Fonts Needed

Now that the prompt has been given a complete overhaul, you will need need to install nerd fonts to run `vsh` properly. [Click here](https://github.com/ryanoasis/nerd-fonts) to choose your own nerd font, no need to worry as the font that you are used to using should probably have its own nerd font counterpart. I recommend using [Jetbrains Mono Nerd font](https://github.com/ryanoasis/nerd-fonts/blob/master/patched-fonts/JetBrainsMono/Ligatures/Regular/complete/JetBrains%20Mono%20Regular%20Nerd%20Font%20Complete%20Mono.ttf) to get started!

#### Roadmap

- [x] Proper Prompt
- [x] Run commands
- [x] Exit with Ctrl+C & Ctrl+D via Rustyline
- [x] Good looking prompt
- [x] Multiple Commands
- [x] Command History
- [ ] Prompt Customization
- [ ] Piping
- [ ] Command Completion
- [ ] `vsh` Scripting language :eyes:
- [ ] Custom `ls` command
- [ ] Intergration with `git`, `node` and `cargo`
- [ ] Customization via `.vshrc`
- [ ] Plugin Support (Yikes!)
