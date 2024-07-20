# Jumping Frog Problem
This CLI application concurrently simulates the frogs in the [Jumping Frog Problem](#What is the Jumping Frog Problem?) and compiles the results into a CSV spreadsheet.
## What is the Jumping Frog Problem?
The jumping frog problem is a simplified version of a gambler's ruin where a frog exists on an infinite 1 dimensional line of lily pads and will jump either left or right randomly with a 50% chance of either event occurring.
## Usage
When started the programme will prompt you with 2 questions
1. "How many frogs per group do you want to simulate?" - this will determine the number of threads at a time the programme will spawn
2. "How many groups do you want to simulate?" - this will determine the number of times the programme will simulate the amount of frogs specified
## Installation
You can use the programme on my [Replit](https://replit.com/@LexXeno/JumpingFrogProblem?v=1)

or...

You can build it form source

Install Rustup from its [official page](https://www.rust-lang.org/tools/install)
```sh
git clone --depth 1 https://github.com/Lex-Xeno/Jumping_Frog_Problem.git
cd Jumping_Frog_Problem
cargo r --release # this runs the programme
```
## Still to Come...
- A worker based model where the user will only be prompted to enter the number of frogs to simulate
- A graphical interface:
    - that prompts the user for input
    - that displays the statistics gathered from the simulations
- A method of exiting early from the simulation loop with what data that has been collected