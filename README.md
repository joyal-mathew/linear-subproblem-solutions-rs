# Linear Subproblem Solutions (Rust)

Ported from [MATLAB](https://github.com/rpiRobotics/linear-subproblem-solutions).

## Usage

1. Clone the repository
2. Add the following to your project's `Cargo.toml`

    ```
    [dependencies.linear-subproblem-solutions-rs]
        path = "[PATH-TO-REPOSITORY]"
    ```

3. Add to your source files where needed (Optional)

    ```
    use linear_subproblem_solutions_rs as subproblems;
    use subproblems::nalgebra as na;
    ```

## Testing

1. Clone the repository
2. Run `cargo test`\
Run `cargo test -- --nocapture` for diagnostic info
