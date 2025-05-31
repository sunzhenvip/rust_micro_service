fn main() {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["src/proto/account.proto"], &["proto"])
        .unwrap();
}