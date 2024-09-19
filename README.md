# Jumping Frog Problem

This CLI application is designed to simulate a given number of frogs in the [Jumping Frog Problem](#what-is-the-jumping-frog-problem) and compiles the results into a CSV spreadsheet.

## What is the Jumping Frog Problem

The jumping frog problem is a simplified version of A Gambler's Ruin where a frog exists on an infinite 1 dimensional line of lily pads and will jump either left or right randomly with a 50% chance of either event occurring.

## Usage

When started the programme will use the values specified in `config.toml` to configure itself.

`config.toml` consists of

- `total_frogs` - the total number of frogs to simulate
- `threads` - the number of threads to use

A default config would look like

```toml
total_frogs = 10000
threads = 8
```

## Running the Programme

```sh
git clone --depth 1 https://github.com/Lex-Xeno/Jumping_Frog_Problem.git
cd Jumping_Frog_Problem
cargo r --release
```

## On a Serious Note

This is the first public project I've ever worked on. Don't expect this to be amazing.

This project only serves as

- a fun programming project from my maths teacher
- a way for me to get familiarized with git
- Rust programming practice
  - multi-threading
  - best practices
  - getting used to the memory management system

If anyone has the time and wants to, comments on how I could tweak my code to make it look better or more efficient are more than appreciated! But once again I don't expect this from anyone.

## Still to Come

- A graphical interface (maybe)
- Optimizing the code more (if it need it)
- Unit tests for `Handler`
