# guess_the_pokemon
A simple CLI game where you guess randomly generated Pokemon up to generation 7.

## Usage
Before you can run the game, you need the `assets-pokemon` folder in the same directory as the binary. Sprites can be found in the releases page.

Sprites are provided by [PokéSprite](https://msikma.github.io/pokesprite/).

## Preview
![](https://i.imgur.com/JscV6QD.png)
![](https://i.imgur.com/aCOMmt5.png)
![](https://i.imgur.com/6szeNHY.png)

## Building from source
Rust automatically handles build targeting for your operating system when `cargo build --release` is ran.

If, for whatever reason, you want to compile to a non-native target, refer to the following commands:

### Targeting
Before you can cross-compile, you must first add the appropriate toolchain with

`rustup target add [TOOLCHAIN]`

|Operating System|Toolchain|
|----------------|------------|
|Windows|`x86_64-pc-windows-gnu`|
|Linux|`x86_64-unknown-linux-gnu`|
|OSX (Mac)|`x86_64-apple-darwin`|

Other toolchains can be listed with `rustup target list`

### Building
|Operating System|Bash Command|
|----------------|------------|
|Windows|`cargo build --release --target x86_64-pc-windows-gnu`|
|Linux|`cargo build --release --target x86_64-unknown-linux-gnu`|
|OSX (Mac)|`cargo build --release --target x86_64-apple-darwin`|
