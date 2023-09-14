mod init_surreal_db;

#[tokio::main]
async fn main() {
    // println!("cargo:rustc-cfg=uuid_unstable");
    println!("cargo:rerun-if-changed=NULL");
    init_surreal_db::create_db().await.unwrap();
    tauri_build::build();
}
