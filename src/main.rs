/// Reads a repository
///
/// Given the cwd, will read the package.json file there
/// and crawl the tree based on workspaces, creating a
/// representation of the repository for usage in dependency
/// management, shell execution, and other tasks
fn read_repository(cwd: &PathBuf) -> Repository {

    // for each package.json, you need to 
    // - read it into memory
    // - parse the json
    // - convert to a struct
    // - see if it has a workspaces field
    //     and if it does, iterate over the workspaces and do the same

    // main thread queue of 300
    // spawn 20 worker threads, whenever I get a workspace
    // I need to allocate it to one of the threads
    // 

    for let packageMatch in matches {
        let workspace = read_workspace(&packageMatch);
    }
}

fn main() {
    println!("Hello, world!");
}
