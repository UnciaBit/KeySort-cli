# KeySort-cli

## Todo
 **Features:**
- Recursive mode
    - https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html

**Known Bugs**
- When folder with same name exists at destination, it does not move the folder
- When file already exists, it panics
- fs_extra is too slow for moving large directories/files
    - Workaround: Increase buffersize
        - Downside: Less efficient