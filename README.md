# Copy in rust

### Running
- Install rust
- `cargo run -- ./test/input ./test/output`

### Notes
- Copy only work with directory's as commands
- Program utilizes all CPU cores
- Errors throw panics and exit the program


### Assumptions
- Copying directories means copying its content as well
- Empty directories are not copied
- Program should be multithreaded