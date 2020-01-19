fn main() {
    if let Some(datastax_dir) = option_env!("CASSANDRA_SYS_LIB_PATH") {
        for p in datastax_dir.split(";") {
            println!("cargo:rustc-link-search={}", p);
        }
    }

    if let Some(_) = option_env!("CASSANDRA_SYS_STATIC_LINK") {
        println!("cargo:rustc-link-lib=static=cassandra_static");
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-lib=static=stdc++");
        println!("cargo:rustc-link-lib=static=uv");
        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-search={}", "/usr/lib");
        println!("cargo:rustc-link-search={}", "/lib");
    } else {
        println!("cargo:rustc-flags=-l dylib=cassandra");
        println!("cargo:rustc-flags=-l dylib=crypto");
        println!("cargo:rustc-flags=-l dylib=ssl");
        println!("cargo:rustc-flags=-l dylib=stdc++");
        println!("cargo:rustc-flags=-l dylib=uv");
        println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-search={}", "/usr/local/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-search={}", "/usr/local/lib64");
        println!("cargo:rustc-link-search={}", "/usr/local/lib");
        println!("cargo:rustc-link-search={}", "/usr/lib64/");
        println!("cargo:rustc-link-search={}", "/usr/lib/");
    }
}
