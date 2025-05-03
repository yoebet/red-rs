use std::fs;
use std::path::Path;

fn pdir(d: &Path) {
    println!("dir {:?}", d.file_name());
    for fe in fs::read_dir(d).unwrap().into_iter() {
        let f = fe.unwrap();
        let path = &f.path();
        if f.metadata().unwrap().is_file() {
            pfile(path);
        } else {
            pdir(path);
        }
    }
}

fn pfile(f: &Path) {
    let ns = f.to_str().unwrap();
    if ns.ends_with(".js") {
        let nn = ns.replace(".js", ".ts");
        println!("rename {:?} -> {:?}", ns, nn);
        // fs::rename(ns, nn).unwrap();
    }
}

#[test]
fn test_rename() {
    let d = Path::new(r"C:\ws\node\classic\src\routes");
    pdir(d);
}
