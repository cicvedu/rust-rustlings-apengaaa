fn main() {
    let desired_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", desired_timestamp);
    {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
