# Personal attempts at solving advent of code 2023 in Rust

In every `ex{day}` folder there are my solutions to that day's problems.
The day's input sits in that folder in a file named `input` (I don't publish them here).

The programs get their input from a stdin with an end.
This can be accomplished, for instance, by:

- redirecting the input file into the program,
- heredocs,
- typing the input followed by a `^D`.

Certain solutions have caveats: they have a comment at the beginning with details.
These are:

- Day 5, part 2: inefficient implementation that takes 6m 14s.
- Day 10, part 2: submitted about 1h 20min after day 11 started.

## Run
```sh
cd ex${DAY}

# Both parts
./run.sh

# Only one part
cargo run --bin p${PART} < input
```

## Generate code for a new day
```sh
./generate-ex.sh ${DAY}
```
