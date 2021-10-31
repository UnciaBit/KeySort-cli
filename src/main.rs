use std::io;
use std::fs;
use std::path::{self, Path, PathBuf};

fn main() {


    //Take inputs and store in Vector separated by spaces
    let mut destFolders = String::new();
    println!("Drag and Drop destination folders (Separate each directory by space)");
    io::stdin().read_line(&mut destFolders).expect("Failed to read line");
    let destFolders = destFolders.replace("\\ ", " ");
    let destFolders = destFolders.replace("\\", "");
    let destFolders = destFolders.replace(" /", " //");
    let destFolders: Vec<&str> = destFolders.trim().split(" /").collect(); 
    

    // Bug where it unintentially cuts strings where there is a space in folder name

    // let n = destFolders[0];
    // println!("First item: {}", n);
    // println!("Number of inputs: {}", destFolders.len());

    // println!("{:?}", destFolders);
    let mut destDirs: Vec<Vec<String>> = Vec::new();

    for i in 0..destFolders.len() {
        let destFolder = Path::new(destFolders[i]);
        //Check if the path exists
        if destFolder.exists() && destFolder.is_dir(){
            println!("{} exists and is a directory", destFolders[i]);
            // 2d array of unknown size
            println!("Enter a chracter to assign to folder: \n {}", destFolders[i]);
            // Input only allows one character
            let mut charInput = String::new();
            io::stdin().read_line(&mut charInput).expect("Failed to read line");
            let charInput: char = charInput.trim().chars().next().unwrap();
            println!("charInput: {}", charInput);
            // Add destFolder and charInput in 2d array
            destDirs.push(vec![destFolders[i].to_string(), charInput.to_string()]);
            println!("{:?}", destDirs);
        }
        else {
            println!("{} does not exist, skipping...", destFolders[i]);
        }
    }

    // let mut destDirs: Vec<Vec<String>> = Vec::new();
    for i in 0..destDirs.len() {
        println!("destDir {:?}", destDirs[i][1]); // [i][0] is the folder name, [i][1] is the character
    }

    // Receive files and folders to move from user
    let mut sourceFiles = String::new();

    let source = loop {
        let mut option = String::new();
        println!("Is your file/foler all located in the same directory? (y/n)");
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let mut inputResult = sourceFileInput(&mut option);
        if inputResult != "" {
            break inputResult;
        }
    };
    // Split souce by regex that matches spaces that does come after forward slash
    let mut source: Vec<&str> = source.trim().split_whitespace().collect(); 
    // println!("{:?}", source[0]);
    let mut sourceContents: Vec<PathBuf> = vec![];
    if source.len() > 1 { // If there are multiple files/folders
        for i in 0..source.len() {
            sourceContents.push(Path::new(source[i]).to_path_buf());
        }
    } else if source.len() == 1 { // If there is only one folder, read folder contents
        println!("{:?}", source[0]);
        let sourceFolderContents = fs::read_dir(source[0]).expect("Failed to read directory");
        for file in sourceFolderContents {
            sourceContents.push(file.unwrap().path());
        }
    }
    println!("{:?}", sourceContents);



}

// Function returns a string
fn sourceFileInput(option: &mut String) -> String{
    let mut sourceDir = String::new();
    println!("{}", option);
    match option.trim() {
        "y" => {
            println!("Enter the directory where all files/folders are stored");
            io::stdin().read_line(&mut sourceDir).expect("Failed to read line");
        },
        "n" => {
            println!("Enter files/folders to move (Separate by spaces)");
            io::stdin().read_line(&mut sourceDir).expect("Failed to read line");
        },
        _ => {
            println!("Invalid input, try again");
        }
    }
    println!("Source Dir (In Function): {}", sourceDir);
    return sourceDir.trim().to_string();
}