# ls
An implementation of the `ls` command, in Rust.

<p align="center">
<img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExYzgwMjkxNWExMmI0YWMyMDEzOTQ5NWU3NDE0YjQ5MGEwMzlkOGU1OSZlcD12MV9pbnRlcm5hbF9naWZzX2dpZklkJmN0PWc/Idi0rO80vvpgSEJtLw/giphy.gif" width="65%" />
</p>

## Using it
To build the project, execute the following command:
```bash
cargo build
```

Once built, move the executable to the root of the directory.
```bash
mv ./target/debug/ls .
```

Afterwards, you can find the executable in the `./target/debug/` folder, and use it as you wish:
```bash
./ls <parameters> <arguments>
```

Here are a few command examples:
```bash
./ls -lraRt .
./ls -lr <first file> <second file>
./ls -aR this_file_does_not_exist
```

## Notes
- The following parameters are supported: `-a`, `-l`, `-r`, `-t` and `-R`.
- Only tested on MacOS.
- The output should look **exactly** like the actual command's output looks like, with a couple (very) minor differences.
- This is not meant to be used in production, but should normally work as intended (at least according to the testing that was made).
- Each function has a comment explaining its use.
- The focus is on short and modular functions, to avoid spaghetti code.

## License
This repository is released under the [MIT License](https://github.com/maxdesalle/ls/blob/main/LICENSE).
