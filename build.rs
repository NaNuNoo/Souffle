use std::error;
use std::process;
use std::fs;
use std::path;

type BoxErr = Box<error::Error>;

fn list_cpp(mod_name: &str) -> Result<Vec<String>, BoxErr> {
    let mod_path = path::Path::new(
        &format!(".\\src\\{}", mod_name)
    );
    if !mod_path.is_dir() {
        return Err(
            format!("Not a mod {}", mod_name)
        );
    }
    let file_name_vec: Vec<String> = Vec::new();
    for iter in try!(fs::read_dir(mod_path)) {
        let file_path = try!(iter).path();
        if !file_path.is_file() {
            continue;
        }
        let ext_name = file_path.extension().unwrap_or("");
        if ext_name == ".cpp" {
            let file_name_os = try!(
                file_path.file_stem().ok_or(Err(file_path))
            );
            let file_name = String::from(
                try!(file_name_os.to_str())
            );
            file_name_vec.push(file_name);
        }
    }
    return Ok(file_name_vec);
}

fn compile_cpp(mod_name: &str, file_name_vec: &[String]) -> Result<(), BoxErr> {
    let cpp_name_vec: Vec<String> =
        file_name_vec.iter()
        .map(|file_name|{
            format!("..\\..\\src\\{}\\{}.cpp", mod_name, file_name)
        })
        .collect();
    try!(
        process::Command::new("cl.exe")
        .current_dir(
            format!(".\\target\\native\\{}", mod_name)
        )
        .args(&["/c", "\\Ox"])
        .args(&cpp_name_vec)
        .status()
    );
    return Ok(());
}

fn link_cpp(mod_name: &str, file_name_vec: &[String]) -> Result<(), BoxErr> {
    let obj_name_vec: Vec<String> =
        file_name_vec.iter()
        .map(|file_name|{
            format!("{}\\{}.obj", mod_name, file_name)
        })
        .collect();
    try!(
        process::Command::new("lib.exe")
        .current_dir(".\\target\\native")
        .args(&obj_name_vec)
        .arg(
            format!("/OUT:\"{}.lib\"", mod_name)
        )
        .status()
    );
    return Ok(());
}

fn build_cpp(mod_name: &str) -> Result<(), BoxErr> {
    let file_base_vec = try!(list_cpp(mod_name));
    try!(compile_cpp(mod_name, &file_base_vec));
    try!(link_cpp(mod_name, &file_base_vec));
    return Ok(());
}

fn main() {
    fs::create_dir(".\\target");
    fs::create_dir(".\\target\\native");
    build_cpp("");
    //println!("cargo:rustc-link-search=./target/native/");
}
