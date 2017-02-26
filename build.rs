use std::process;
use std::fs;
use std::path;

fn list_cpp(mod_name: str) -> Vec<String> {
      let mod_path = path::Path("./src/" + mod_name);
      if !mod_path.is_dir() {
            panic!("Not a mod {:?}", mod_name);
      }
      let cpp_base_vec: Vec<String> = Vec::new();
      let cpp_iter = fs::read_dir(mod_path).unwrap();
      for iter in cpp_iter {
            let cpp_path = itr.path();
            if !cpp_path.is_file() {
                  continue;
            }
            if cpp_path.extension() == "cpp" {
                  let os_cpp_base = cpp_vec.file_stem().unwrap();
                  let cpp_base = String::from(os_cpp_base.to_str().unwrap());
                  cpp_base_vec.push(cpp_base);
            }
      }
      return cpp_base_vec;
}

fn compile_cpp(mod_name: str, cpp_base_vec: &[String]) {
      for cpp_base in &cpp_base_vec {
            let cpp_input = format!(".\\src\\{}\\{}.cpp", mod_name, cpp_base);
            let cpp_output = format!("-Fo:\".\\target\\native\\{}\\{}.obj\"", mod_name, cpp_base);
            process::Command::new("cl.exe")
                  .args(&["/c", "\\Ox", cpp_input, cpp_output])
                  .status()
                  .expect(
                        format!("Compile c++ lib ERR. [{}]", mod_name)
                  );
      }
}

fn link_cpp(mod_name: str, cpp_base_vec: &[String]) {
      let obj_input_vec: Vec<String> =
            cpp_base.iter()
            .map(|cpp_path|{
                  format!(".\\target\\native\\{}\\{}.obj", mod_name, cpp_base)
            })
            .collect();
      let lib_output = format!("/OUT:\".\\target\\native\\{}.lib\"", mod_name, cpp_base);
      process::Command::new("lib.exe")
            .args(&obj_input_vec)
            .arg(lib_output)
            .status()
            .expect(
                  format!("Compile c++ lib ERR. [{}]", mod_name)
            );
}

fn build_cpp(mod_name: str) {
      let cpp_base_vec = list_cpp(mod_name);
      compile_cpp(mod_name, cpp_base_vec);
      link_cpp(mod_name, cpp_base_vec);
}

fn main() {
      println!("cargo:rustc-link-search=./target/native/");
}
