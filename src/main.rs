use std::io;
use std::fs;
use std::path::{self, Path, PathBuf};

extern crate fs_extra;
use fs_extra::dir::*;
use fs_extra::error::*;
use std::{thread, time};
use std::sync::mpsc::{self, TryRecvError};
use crossterm::{QueueableCommand, cursor};
use std::{io::{stdout, Write}};
use fs_extra::file::*;

fn main() {
    //Take inputs and store in Vector separated by spaces
    let mut destFolders = String::new();
    println!("Drag and Drop destination folders (Separate each directory by space)");
    io::stdin().read_line(&mut destFolders).expect("Failed to read line");

    let destFolders: Vec<String> = dirListFormat(&destFolders);

    let mut destDirs: Vec<Vec<String>> = Vec::new(); // 2d Vector of destination directories

    for i in 0..destFolders.len() {
        let destFolder = Path::new(&destFolders[i]);
        if destFolder.exists() && destFolder.is_dir(){ // Check if destination folder exists and is a directory
            
            // println!("{} exists and is a directory", destFolders[i]);
            println!("Enter a chracter to assign to folder: \n {}", destFolders[i]);
            
            // Input only allows one character
            let mut charInput = String::new();
            io::stdin().read_line(&mut charInput).expect("Failed to read line");
            let charInput: char = charInput.trim().chars().next().unwrap();
            // println!("charInput: {}", charInput);

            // Add destFolder and charInput in 2d Vector
            destDirs.push(vec![destFolders[i].to_string(), charInput.to_string()]);
            // println!("{:?}", destDirs);
        }
        else {
            println!("{} does not exist, skipping...", destFolders[i]);
        }
    }

    // for i in 0..destDirs.len() {
    //     println!("destDir {:?}", destDirs[i][1]); // [i][0] is the folder name, [i][1] is the character
    // }

    // Receive files and folders to move from user
    let mut sourceFiles = String::new();

    let mut inputResult = String::new();
    let mut resultTuple = (String::new(), true); //If all files and folders are in the same directory, true if yes

    let source = loop {
        let mut option = String::new();
        println!("Is your file/foler all located in the same directory? (y/n)");
        io::stdin().read_line(&mut option).expect("Failed to read line");
        resultTuple = sourceFileInput(&mut option);
        if &resultTuple.0 != "" { // If input was valid, returns string of list of directories/files
            break &resultTuple.0;
        }
    };

    // println!("Result Tuple 0 {}", &resultTuple.0);
    // println!("Result Tuple 1 {}", &resultTuple.1);

    // Put this in function later
    let source: Vec<String> = dirListFormat(&source);
    //

    let mut sourceContents: Vec<PathBuf> = vec![];
    if source.len() > 1 || resultTuple.1 == false{ // If there are multiple files/folders
        for i in 0..source.len() {
            sourceContents.push(Path::new(&source[i]).to_path_buf());
        }
    } else if source.len() == 1 && resultTuple.1 == true{ 
        // If there is only one folder and "All file/folders located in same directory" is true
        // println!("{:?}", source[0])
        let sourceFolderContents = fs::read_dir(&source[0]).expect("Failed to read directory");
        for file in sourceFolderContents {
            sourceContents.push(file.unwrap().path());
        }
    }
    // println!("{:?}", sourceContents);

    // Loop through sourceContents and move to destDirs
    for i in 0..sourceContents.len() {
        match sourceContents[i].is_dir(){
            true => {
                loop{
                    let mut option = String::new();
                    println!("Move \n {} \n to", sourceContents[i].display());
                    io::stdin().read_line(&mut option).expect("Failed to read line");
                    let mut result = moveContent(&sourceContents[i], &option, &destDirs, true);
                    // println!("{:?}", result);
                    if result == true {
                        break;
                    } else{
                        println!("Folder assosiated with {} was not found", option)
                    }
                }
            }
            false => {
                loop{
                    let mut option = String::new();
                    println!("Move {} to: ", sourceContents[i].file_name().unwrap().to_str().unwrap());
                    io::stdin().read_line(&mut option).expect("Failed to read line");
                    let mut result = moveContent(&sourceContents[i], &option, &destDirs, false);
                    // println!("{:?}", result);
                    if result == true {
                        break;
                    } else{
                        println!("Folder assosiated with {} was not found", option)
                    }
                }
            }
        }
        // println!("{}", sourceContents[i].display());
    }



}


fn sourceFileInput(option: &mut String) -> (String, bool){
    let mut sourceDir = String::new();
    let mut sourceOption = true;
    // println!("{}", option);
    match option.trim() {
        "y" => {
            println!("Enter the directory where all files/folders are stored");
            io::stdin().read_line(&mut sourceDir).expect("Failed to read line");
            sourceOption = true;
        },
        "n" => {
            println!("Enter files/folders to move (Separate by spaces)");
            io::stdin().read_line(&mut sourceDir).expect("Failed to read line");
            sourceOption = false;
        },
        _ => {
            println!("Invalid input, try again");
        }
    }
    // println!("Source Dir (In Function): {}", sourceDir);
    return (sourceDir.trim().to_string(),sourceOption);
}


fn moveContent(source: &PathBuf, dest: &str, destDirs: &Vec<Vec<String>>, mode: bool) -> bool{ // mode: true = folder, false = file
    let source = source.to_path_buf();
    let mut iter = destDirs.iter();
    let dest = dest.trim();
    let mut destFound = false;
    let mut destPath = PathBuf::new();
    let mut stdout = stdout();
    let copyOptionsDir = fs_extra::dir::CopyOptions::new();
    let copyOptionsFile = fs_extra::file::CopyOptions{
        buffer_size: 10000,
        ..Default::default()
    };

    // println!("moveContent Function source: {:?}", source);
    // println!("moveContent Function dest: {:?}", dest);

    // Find dest from 2d vector destDirs
    // Create new boolean varaible to store result of match
    for i in 0..destDirs.len() {
        if destDirs[i][1].to_string() == dest{
            // println!("Equal: destDir: {}, dest: {}", destDirs[i][1], dest);
            destFound = true;
            destPath = Path::new(&destDirs[i][0]).to_path_buf();
            break;
        }
    }

    // println!("Found: {}", &destFound);    


    if destFound == true {
        match mode {
            true => {// If folder
                let handleDir = |process_info: fs_extra::dir::TransitProcess| {
                    // stdout.queue(cursor::SavePosition).unwrap();
                    // stdout.write(format!("{} Bytes of {} Moved", process_info.copied_bytes, process_info.total_bytes).as_bytes()).unwrap();
                    // stdout.queue(cursor::RestorePosition).unwrap();
                    // stdout.flush().unwrap();
                    fs_extra::dir::TransitProcessResult::ContinueOrAbort
                };

                // println!("Moving: {}\n to: {}", source.display(), destPath.display());

                move_dir_with_progress(source, destPath, &copyOptionsDir, handleDir).expect("Failed to move file");
            },
            false => {// If file
                let handleFile = |process_info: fs_extra::file::TransitProcess| {
                    // stdout.queue(cursor::SavePosition).unwrap();
                    // stdout.write(format!("{} Bytes of {} Moved", process_info.copied_bytes, &process_info.total_bytes).as_bytes()).unwrap();
                    // stdout.queue(cursor::RestorePosition).unwrap();
                    // stdout.flush().unwrap();
                };

                // println!("Moving: {}\n to: {}", source.display(), destPath.display());

                // If the character at the end of destPath is not /, add it
                if destPath.to_str().unwrap().chars().last().unwrap() != '/' {
                    destPath.join("/");
                }
                // Add souce filename at the end of destPath
                let mut destPath = destPath.join(source.file_name().unwrap());
                // println!("{:?}", destPath);
                move_file_with_progress(source, destPath, &copyOptionsFile, handleFile).expect("Failed to move file");
            },
        }
        
    }
    return destFound;
}

fn dirListFormat(dir: &str) -> Vec<String>{
    let dirList = dir.to_string();
    let dirList = dirList.replace("\\ ", " ");
    let dirList = dirList.replace("\\", "");
    let dirList = dirList.replace(" /", " //");
    let dirList: Vec<String> = dirList.trim().split(" /").map(|x| x.to_string()).collect(); 
    return dirList;
}