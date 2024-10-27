use std::{fs, path::Path, process::Command};

pub fn run(executable: String) {
    let in_path = Path::new("./tests/in");
    let out_path = Path::new("./tests/out");
    if !in_path
        .try_exists()
        .expect("Can't check existance of input directory")
    {
        println!("input directory is Missing");
        println!("run thrl download to download test cases");
        return;
    }
    if !out_path
        .try_exists()
        .expect("Can't check existance of output directory")
    {
        println!("output directory is Missing");
        println!("run thrl download to download test cases");
        return;
    }
    let mut in_out: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir(in_path).expect("cannot read input directory") {
        let entry = entry.unwrap();
        let filename = entry.file_name().into_string().unwrap();
        let filename = filename.replace(".in", ".out");
        if out_path
            .join(&filename)
            .try_exists()
            .expect("can't read output directory for corresponding output")
        {
            let file_name = entry.path().to_str().unwrap().to_string();
            in_out.push((
                file_name,
                out_path.join(&filename).to_str().unwrap().to_string(),
            ))
        } else {
            println!("{} has no corresponding output", filename);
        }
    }
    for (in_file, out_file) in in_out {
        let output = Command::new("valgrind")
            .arg("--leak-check=full -s")
            .arg(&executable)
            .arg(format!("< {}", in_file))
            .output()
            .expect("can't run valgrind");

    }
}
