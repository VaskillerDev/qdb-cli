use std::env;
use std::env::current_dir;
use std::path::{Path, PathBuf};

// Set resource folder for files after build.
// It's create folder 'resource' in target output path
const RESOURCE_DIR: &str = "resource";

// Is function use for return parent path for target folder.
// Example: from <...>\qdb\target\debug\<...>\<...>\<...> to <...>\qdb\target\debug\
fn get_main_path(path: &Path) -> &Path {
    path.parent().unwrap().parent().unwrap().parent().unwrap()
}

fn main() {
    //Get source code path
    let src_dir = current_dir().unwrap().join(RESOURCE_DIR);

    // Get output path from var -> Get output path in Path type -> Get target output path
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let out_dir = get_main_path(out_dir);

    println!("SRC_DIR: {}", src_dir.join("qdb-conf.yml").display());
    println!("OUT_DIR: {}", out_dir.display());

    match std::fs::create_dir(out_dir.join(RESOURCE_DIR)) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    load_resource(&src_dir, out_dir);
}

// Load files from <RESOURCE_DIR> and copy to target/<...>/<RESOURCE_DIR>
fn load_resource(from: &PathBuf, to: &Path) {
    fn copy_file(from: &PathBuf, to: &Path, file: &str) {
        let to = &to.join(RESOURCE_DIR).join(file);
        let r = std::fs::copy(from.join(file), to);
        match r {
            Ok(_) => println!("File {} copied to {}", file, to.display()),
            Err(e) => println!("{}", e),
        }
    }
    // File's list for copy
    copy_file(&from, &to, "qdb-conf.yml");
    copy_file(&from, &to, "app-conf.yml");
}
