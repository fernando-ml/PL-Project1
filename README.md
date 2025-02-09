# Programming Languages Project 1

Our project implements a procedural and object-oriented approach to parsing and processing a JSON file containing information about a solar system.

## Student Information
- Name: Fernando Martinez

## Project Structure
- `ProceduralApp`: Rust program
- `ObjectOrientedApp`: Python program
- `data`: JSON files



## Requirements
- Rust (latest stable version)
- Python 3.X+
- Cargo (comes with Rust)

## How to run

### Rust

```
cd ProceduralApp
cargo run -- ../data/JSONPlain.txt
```

### Python
```
cd ObjectOrientedApp
python solar_system.py ../data/JSONPlain.txt
```
### Trying it with a different JSON file
- For the Rust program:
```
cargo run -- ../data/<your_json_file>.txt
```
- For the Python program:
```
python solar_system.py ../data/<your_json_file>.txt
```
