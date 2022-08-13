<div align="center">
<h1>urbaani - Urbaanisanakirja CLI</h1>
</div>

<p align=center>
A CLI tool for fetching Urbaanisanakirja definitions.
</p>

<p align=center>
  <img src=image.png>
</p>

# Installation
```
git clone https://github.com/jervw/urbaani-cli.git
cd urbaani-cli
cargo build --release
```
Add to PATH:

`sudo cp target/release/urbaani /usr/bin`


# Usage

`urbaani <term>` -- Search the term from dictionary

`urbaani hello world`

`urbaani <word> [n]` -- Show the `n` number of definitions

`urbaani nakki 5`
