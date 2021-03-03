/*!
 * Copyright 2021 Maarten de Klerk
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
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
    .use_core()
    .ctypes_prefix("libc")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(format!("{}/bindings.rs", out_dir))
    .expect("Couldn't write bindings!");
}
