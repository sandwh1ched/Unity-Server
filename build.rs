fn main() {
    #[cfg(not(target_os = "linux"))]
    panic!("this software currently only supports Linux-based OSes");
}
