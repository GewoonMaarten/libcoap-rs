use std::env;

fn main() {
  println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
  println!("cargo:rustc-link-lib=static=mbedtls");
  println!("cargo:rustc-link-lib=static=mbedcrypto");
  println!("cargo:rustc-link-lib=static=mbedx509");

  let mut dst = cmake::Config::new("libcoap")
    .define("DTLS_BACKEND", "mbedtls")
    .define("ENABLE_TESTS", "OFF")
    .define("ENABLE_EXAMPLES", "OFF")
    .define("ENABLE_DOCS", "OFF")
    .build();

  dst.push("lib");

  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=static=coap-2");

  println!("cargo:rerun-if-changed=libcoap");

  let out_dir = env::var("OUT_DIR").unwrap();

  let bindings = bindgen::Builder::default()
    .header(format!("{}/build/include/coap2/coap.h", out_dir))
    .clang_arg(format!("-I{}/include/coap2", out_dir))
    .generate_comments(false) // true breaks the binding. see: https://github.com/rust-lang/rust-bindgen/issues/426
    .whitelist_var("^(?i)(lib)?coap_.*")
    .whitelist_type("^(?i)(lib)?coap_.*")
    .whitelist_function("^(?i)(lib)?coap_.*")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(format!("{}/bindings.rs", out_dir))
    .expect("Couldn't write bindings!");
}
