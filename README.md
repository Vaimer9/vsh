<h1 align="center", text="bold">vsh</h1>
<div align="center">A Blazingly fast shell made in Rust 🦀</div>

## Why make another shell?
Because the [current leading rust shell](https://github.com/nushell/nushell/) is very opinionated, atleast to me. As it brings a lot to the table for someone who just wants a shell but in rust, new `ls` command, new scripting experiece etc. What's needed is just bash but in rust and vsh is there to deliver, the planned scripting language will be a 1:1 copy of bash and all of its features so that people don't feel *homesick* or dropped into a new space when they start using vsh. As the for shell prompt I plan to expand upon it to add plugins to accept a wide array of custom plugins all written in vsh. Till then feel free to contribute yourself!

## Roadmap
- [x] Proper Prompt
- [x] Run commands
- [x] Exit with Ctrl+C & Ctrl+D via Rustyline
- [ ] Multiple Commands
- [ ] Piping
- [ ] `vsh` Scripting language :eyes:
- [ ] Custom `ls` command
- [ ] Intergration with `git`, `node` and `cargo`
- [ ] Customization via `.vshrc`
- [ ] Plugin Support (Yikes!)