# KeySort-cli

keysort is a small utility to sort files and folders into different folders by assigning a key/character to folders, then
pressing the assigned keys for each files and folders

> **As of version 0.1.0-alpha.2B, this application only works with macOS unless the 
terminal input is correctly formatted**

## Installation

Install from Crates.io
```
cargo install keysort
```

## Usage

_With the current version of keysort, it requires you to enter the syntax correctly and not have exisitng files/folders at the destination, otherwise the program will panic and close_

1. **Start keysort:**
```
keysort
```

2. **Input the **destination** folders (Absolute path recommended)**
```
$ keysort
Drag and Drop destination folders (Separate each directory by space)
<directory1> <directory2> <directory3>

// Example:
/Users/john/Desktop /Users/john/Documents /Users/Documents/Folder\ 1
```
> On macOS, this input can be done just by dragging and dropping select folders into the terminal window,
however on Linux and Windows, that will append apostrophes (' ') at each end of the path. This could potentially cause problem with the program,
therefore remove the apostrophes (') from the directory name if you are using Windows and Linux

3. **Assign keys/characters to the folders**

The program will loop through the folders you inputted in step 2, 
```
Enter a character to assign to folder:
/directory/inputted/1
<character> //User input here

Enter a character to assign to folder:
/directory/inputted/2
<character>

//Example
Enter a character to assign to folder:
/Users/john/Desktop
a //Type 'a' and hit Enter

Enter a character to assign to folder:
/Users/john/Documents
b //Type 'b' and hit Enter
...........
```

4. **Selecting which files/folders to move**

After assigning keys to destination folders, the program will prompt you:
```
Is your file/folder all located in the same directory? (y/n)
```

If you have a folder containing all the files and folders that you intend to move, 
select **yes** (type 'y') to this option. This option will scan for files and folders to move in the **first level only (contents of the selected folder and not subdirectories)**.

So when you select yes and choose allContents as the folder where all the files and folders are located
```
// allContents/
// |
// +- File1.txt
// +- File2.rar
// +- File3.md
// +- Folder1
//   |
//   +- fileInFolder.txt
//   +- fileInFolder2.rar
// +- Folder2
//   |
//   +- Folder3
//   +- Folder4

Is your file/folder all located in the same directory? (y/n)
y

Enter the directory where all files/folders are stored
/path/to/allContents <- Only input one directory, then hit Enter
```
The following are going to be moved
```
File1.txt
File2.rar
File3.md
Folder1 // And its contents (recursive)
Folder2 // And its contents (recursive)
```

If you chose **no** for this option, you can select individual files and folders to be moved. Just like step 2,
separate each item with space:
```
Is your file/folder all located in the same directory? (y/n)
n

Enter files/folders to move (Separate by spaces)
<directory1> <file1.ext> <file2.ext> <directory3>

// Example
Enter files/folders to move (Separate by spaces)
/Users/john/Desktop/Folder1 /Users/john/Documents/file.rar /Users/john/Music/MusicFolder
```

> As of right now, this program is slow at moving big files as it is using fs_extra library which attempts to copy folders bit by bit, 
> and it is limited by the buffer size that it allocates. Currently, this buffer size is set to 100MB. I would recommend avoiding this tool if you have a file in excess 
> of 5 GB, it definately moves the file, but the command line will be unresponsive for a while

5. **Moving files**

The program will now go through each item that you selected in the previous step.

All you have to do now is enter the key/character of the destination folder that you wish to move to. 
```
Move [source file/folder] to:
a //Type the character assigned in step 3 then hit Enter

Move [source file/folder] to:
b
```
Immediately after entering the character, it wil start moving the file.
Depending on the size of the file, it may take time to show the next 'Move' prompt, wait patiently.
Once all the files are moved, the program will exit

## Todos and known issues
### Todo
- Add support for Windows and Linux drag and drop syntax
- Console decoration
    - https://docs.rs/dialoguer/0.9.0/dialoguer/
    - https://docs.rs/indicatif/0.16.2/indicatif/
    - https://docs.rs/console/0.15.0/console/
- Accept flags/arguments
  - Recursive mode (Move all files individually upto nth level of folder)
  - Safe mode (Perform at the end of program after confirmation)
  - Input .json or .txt for destination and source file/folder input

### Known Issues
- When folder with same name exists at destination, it does not move the folder
- When file already exists, it panics
- fs_extra is too slow for moving large directories/files
    - Workaround: Increase buffersize
        - Downside: Less efficient
