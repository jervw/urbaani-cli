<div align="center">
<h1>urbaani - Urbananisanakirja CLI</h1>
</div>

<p align=center>
A CLI tool for fetching Urbaanisanakirja definitions.
</p>

<p align=center>
  <img src=>
</p>

# Installation
```
git clone https://github.com/jervw/urbaani.git
cd urbaani
cargo build --release
```
Add to PATH
`cp target/release/urbaani /usr/bin`


# Usage

`urbaani <term>` -- Search the term from dictionary

`urbaani hello world`

`urbaani <word> [n]` -- Show the `n` number of definitions

`urbaani nakki 5`
