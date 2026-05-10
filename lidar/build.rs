fn main() {
    cc::Build::new()
        .cpp(true)
        .file("wrapper/ydlidar_wrapper.cpp")
        .include("/home/dikety/codes/YDLidar-SDK/src")
        .include("/home/dikety/codes/YDLidar-SDK/core/common")
        .flag("-std=c++11")
        .compile("ydlidar_wrapper");

    println!(
        "cargo:rustc-link-search=native=/home/dikety/codes/YDLidar-SDK/build"
    );

    println!("cargo:rustc-link-lib=static=ydlidar_sdk");

    println!("cargo:rustc-link-lib=stdc++");
}
~
