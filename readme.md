# N-Puzzle
> Content
1. [Goal State](#goal-states)
2. [Puzzle solvability check](#puzzle-solvability-check)
3. [Heuristic Weights](#heuristic-weights)
4. [Deployment (How to run)](#deployment)
5. [Modifying variables](#modifying-variables)
## Goal States
A goal state is defined starting from a set of natural numbers including 0 incrementing from left to right, top to bottom, starting with the number zero.
e.g. a goal state for $n=4$ would be the following:

| 0  | 1 | 2 | 3  |
|----| -- | -- |----|
| 4  | 5 | 6 | 7  |
| 8  | 9 | 10 | 11 |
| 12 | 13 | 14 | 15 |

### Assignment goal state:
Specifically for $n=3$:

| 1 | 2 | 3 |
|---|---|---|
| 8 | 0 | 4 |
| 7 | 6 | 5 |


## Puzzle solvability check:

A random grid of $n*n$ is generated as a permutation of a set of natural numbers up to $n$, including 0.
### If $N=odd$:
The puzzle is solvable if the number of $inversions = odd$.
### If $N=even$:
The puzzle is solvable if $inversions + f = even$, where $f$ is defined as the number of rows the $0th$ tile is from the bottom of the grid.

### Inversion
An inversion is defined as a pair of numbers <a appears before $b$ but


## Heuristic Weights
The heuristic is defined as $f(n) = g(n) + h(n$),
- Where $f(n)$ is defined as the total heuristic function
- $g(n)$ is defined as the number of moves taken to get to the current state
- $h(n)$ is defined as the value given from the chosen heuristic

### Misplaced tiles
- $h(n)$ is defined as the number of tiles that differ from the goal state
### Manhattan Distance
- $h(n)$ is defined as the total distance of tiles from their goal state position, calculated in Manhattan distance
- Typically results in less nodes explored  than the misplaced tiles heuristic and thus a faster A* runtime.
- Is weighted $3*$ as much as it should to improve A* speed
  - Keep in mind that this increases the total `f(n)`


## Deployment

### Running the program (Rust)
- The whole directory contains the rust libraries needed for cargo to run.
- Assuming rust is installed, running `cargo run` in the main directory will run the code
- Check Rust and Cargo versions by running:
  - `rustc --version`
  - `cargo --version`
- Note: Rust is already installed on the ECS machines

## Modifying variables
### Changing $n$
- `Line 7` contains the initialization of the first grid state. The parameter `size` may be modified that acts as the $n$ variable.
- The `heuristic` parameter lets you choose between the misplaced tiles heuristic and the Manhattan heuristic. 
  - `heuristic: 0` uses the misplaced tiles heuristic
  - `heuristic: 1` uses the Manhattan tile heuristic

### Setting the initial state to a non-random one
- Lines `10-14` may be used to initialize a vector containing a set of preset values if you wish to use a non-random one.

