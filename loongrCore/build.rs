use std::env;
use std::fs::File;
use std::include_bytes;
use std::io::Write;
use std::path::Path;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let link_script = Path::new(&outdir).join("link.lds");
    let mut script = File::create(&link_script).unwrap();
    script.write_all(include_bytes!("linker.ld")).unwrap();
    println!("cargo:rustc-link-arg=-T{}", &link_script.display());
    println!("cargo:rustc-link-arg=-nostdlib"); //关闭gcc的默认链接
                                                // println!("cargo:rustc-link-arg=-no-pie"); //rust默认连接到Scrt1.o，使用动态链接
                                                //println!("cargo:rustc-link-arg=-Wl,-Map=rust.map");
    println!("cargo:rerun-if-change=linker.ld");
    println!("cargo:rerun-if-changed=../user/target/");
    println!("cargo:rerun-if-changed=src/");
}
