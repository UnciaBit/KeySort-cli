# KeySort-cli

## Todo
- Add support for Windows and Linux drag and drop syntax
- Console decoration
    - https://docs.rs/dialoguer/0.9.0/dialoguer/
    - https://docs.rs/indicatif/0.16.2/indicatif/
    - https://docs.rs/console/0.15.0/console/

**Features:**
- Recursive mode
    - https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html

**Known Bugs**
- When folder with same name exists at destination, it does not move the folder
- When file already exists, it panics
- fs_extra is too slow for moving large directories/files
    - Workaround: Increase buffersize
        - Downside: Less efficient
