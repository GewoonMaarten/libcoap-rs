use cmake::Config;

fn main() {
  let dst = Config::new("libcoap")
    .define("USE_VENDORED_TINYDTLS", "ON")
    .define("ENABLE_TESTS", "OFF")
    .define("ENABLE_EXAMPLES", "OFF")
    .define("ENABLE_DOCS", "OFF")
    .build();

  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=static=coap-2");
}
