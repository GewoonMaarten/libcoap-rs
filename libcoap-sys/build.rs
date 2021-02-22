use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
  println!("cargo:rustc-link-lib=static=mbedtls");
  println!("cargo:rustc-link-lib=static=mbedcrypto");
  println!("cargo:rustc-link-lib=static=mbedx509");

  let mut dst = Config::new("libcoap")
    .define("DTLS_BACKEND", "mbedtls")
    .define("ENABLE_TESTS", "OFF")
    .define("ENABLE_EXAMPLES", "OFF")
    .define("ENABLE_DOCS", "OFF")
    .build();

  dst.push("lib");

  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=static=coap-2");

  println!("cargo:rerun-if-changed=wrapper.h");

  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .generate_comments(false) // true breaks the binding
    .whitelist_var("^(?i)coap_.*")
    .whitelist_type("^(?i)coap_.*")
    .whitelist_function("^(?i)coap_.*")
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
