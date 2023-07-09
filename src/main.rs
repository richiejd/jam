use rayon::prelude::*;
use serde::Deserialize;
use serde_json::{Result, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

//Struct for json file 
#[derive(Debug, Deserialize)]
pub struct JsonTree {
    json_data: Value,
}

impl JsonTree {
    //Initializes struct from a json file, given the file's path
    fn new(path: &Path) -> JsonTree {
        JsonTree { json_data: 
            {
                let file_content = fs::read_to_string(path).expect("Jam: error reading file");
                serde_json::from_str::<Value>(&file_content).expect("Jam: error serializing to JSON")
            }
         }
    }

    //Checks if a JsonTree has a workspaces field
    fn has_workspaces(&self) -> bool {
        self.json_data.get("workspaces").is_some()
    }
}

/// Reads a repository
///
/// Given the cwd, will read the package.json file there
/// and crawl the tree based on workspaces, creating a
/// representation of the repository for usage in dependency
/// management, shell execution, and other tasks
/// 
/// Current version simply iterates through directories and pushes any found json files to a vector
/// If no workspaces are found in a json file, does not look through any subdirectories
/// Utilizes rayon to dynamically generate threads for maximium performance while 
/// mainttaining data race freedom
pub fn read_repository(base_path: &Path) -> impl ParallelIterator<Item = JsonTree> {

    fn read_dir(json_files: Arc<Mutex<Vec<JsonTree>>>, s: &rayon::Scope<'_>, base_path: PathBuf) {
        let mut dir_paths: Vec<PathBuf> = Vec::new();

        for entry in fs::read_dir(base_path).unwrap() {
            let entry: fs::DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            let metadata: fs::Metadata = entry.metadata().unwrap();
             if metadata.is_file() && path.to_string_lossy().ends_with("package.json") {
                json_files.lock().unwrap().push(JsonTree::new(&path));
            }
            else if metadata.is_dir() {
                dir_paths.push(path);
            } 
        }

        //Checks if there is a workspace element in the most recently appended json
        //If there is a workspace, spawns a new thread for each subdirectory
        if let Some(top_element) = json_files.lock().unwrap().first() {
            if top_element.has_workspaces() {
                // Perform the loop for each nested directory
                for path in dir_paths {
                    let move_json_files: Arc<Mutex<Vec<JsonTree>>> = json_files.clone();

                    //Spawns a new thread to search through next directory
                    s.spawn(move |s1: &rayon::Scope<'_>| read_dir(move_json_files, s1, path));
                }
            }
        }
        
    }

    //Beginning of read repository function
    let json_files: Arc<Mutex<Vec<JsonTree>>> = Arc::new(Mutex::new(Vec::new()));
    let base_path: PathBuf = base_path.to_owned();
    let move_json_files: Arc<Mutex<Vec<JsonTree>>> = json_files.clone();

    //Creates initial thread to read first json file
    rayon::scope(move |s: &rayon::Scope<'_>| s.spawn(move |s1: &rayon::Scope<'_>| read_dir(move_json_files, s1, base_path)));

    //End of repository function
    let json_files: Vec<JsonTree> = Arc::try_unwrap(json_files).unwrap().into_inner().unwrap();
    json_files.into_par_iter()
}

fn main() -> Result<()> {
    
    //Simply tests functionality of read_repository function. Prints the name of each json file found.
    let path: &Path = Path::new(r#".\__fixtures__"#);
    let repository = read_repository(path);
    repository.for_each(|json_file: JsonTree| {
        println!("{:?}", json_file.json_data.get("name").expect("Jam: Json file missing name field").to_string());
    });

    Ok(())

}