use dotenvy;
fn main() {
    let _ = dotenvy::from_filename(".env");

    let keys = ["GITHUB_OAUTH_CLIENT_ID", "GITHUB_OAUTH_CLIENT_SECRET"];

    for (key, value) in std::env::vars() {
        if keys.contains(&key.as_str()) {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }

    println!("cargo:rerun-if-changed=.env");
}
