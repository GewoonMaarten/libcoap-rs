use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
  let mut dst = Config::new("libcoap")
    .define("DTLS_BACKEND", "tinydtls")
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
    .generate_comments(true)
    .use_core()
    .whitelist_type("coap_.*")
    .whitelist_function("coap_.*")
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
