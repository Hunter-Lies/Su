fn main() {
    println!("cargo:rerun-if-changed=../src/index.html");
    println!("cargo:rerun-if-changed=../src/css/styles.css");
    println!("cargo:rerun-if-changed=../src/js/main.js");
    println!("cargo:rerun-if-changed=../src/assets/fonts");
    println!("cargo:rerun-if-changed=web/send.html");
    println!("cargo:rerun-if-changed=web/download.html");
    println!("cargo:rerun-if-changed=web/bundle_multi.html");
    println!("cargo:rerun-if-changed=tauri.conf.json");
    tauri_build::build()
}
