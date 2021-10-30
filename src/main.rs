use std::io;
use std::fs;
use std::path::Path;

fn main() {
    //Take inputs and store in Vector separated by spaces
    let mut destFolders = String::new();
    println!("Drag and Drop destination folders (Separate each directory by space)");
    io::stdin().read_line(&mut destFolders).expect("Failed to read line");
    let destFolders: Vec<&str> = destFolders.trim().split_whitespace().collect(); 
    // Bug where it unintentially cuts strings where there is a space in folder name

    // let n = destFolders[0];
    // println!("First item: {}", n);
    // println!("Number of inputs: {}", destFolders.len());

    // println!("{:?}", destFolders);

    for i in 0..destFolders.len() {
        let destFolder = Path::new(destFolders[i]);
        //Check if the path exists
        if destFolder.exists() && destFolder.is_dir(){
            println!("{} exists and is a directory", destFolders[i]);
        }
        else {
            println!("{} does not exist", destFolders[i]);
        }
    }
    

}
