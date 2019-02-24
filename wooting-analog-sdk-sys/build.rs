use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use bindgen;
use cc;
use pkg_config;

fn main() {
    // If enabled, attempt to find `wooting-analog-sdk` via pkg-config. Otherwise, we'll
    // need to build the SDK.
    let use_pkg_config = env::var("WOOTING_ANALOG_SDK_SHARED").is_ok();
    if use_pkg_config {
        if pkg_config::find_library("wooting-analog-sdk").is_ok() {
            return;
        }
    }

    // Clone submodules if that hasn't already happened.
    if !Path::new("vendor/.git").exists() || !Path::new("vendor/hidapi/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status();
    }

    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate bindings.
    let bindings = bindgen::Builder::default()
        .header("vendor/src/wooting-analog-sdk.h")
        .generate()
        .expect("Unable to generate bindings for the Wooting Analog SDK");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Unable to write Wooting Analog SDK bindings");

    // Build hidapi to link against.
    let mut cfg = cc::Build::new();
    cfg.warnings(false)
        .extra_warnings(false)
        .include("vendor/hidapi/hidapi");

    let use_hidraw_for_hidapi = env::var("WOOTING_ANALOG_SDK_HIDAPI_HIDRAW").is_ok();
    let compiled_hidapi = if target.contains("linux") {
        let use_pkg_config_for_hidapi = env::var("WOOTING_ANALOG_SDK_HIDAPI_SHARED").is_ok();
        match (use_pkg_config_for_hidapi, use_hidraw_for_hidapi) {
            (true, false) => {
                pkg_config::probe_library("hidapi-libusb").expect("Unable to find hidapi-libusb");
                false
            }
            (true, true) => {
                pkg_config::probe_library("hidapi-hidraw").expect("Unable to find hidapi-hidraw");
                false
            }
            (false, false) => {
                let hidapi =
                    pkg_config::find_library("libusb-1.0").expect("Unable to find libusb-1.0");
                for path in hidapi.include_paths {
                    cfg.include(path.to_str().unwrap());
                }

                cfg.file("vendor/hidapi/libusb/hid.c")
                    .compile("libhidapi.a");
                true
            }
            (false, true) => {
                pkg_config::probe_library("libudev").expect("Unable to find libudev");

                cfg.file("vendor/hidapi/linux/hid.c").compile("libhidapi.a");
                true
            }
        }
    } else if target.contains("windows") {
        cfg.file("vendor/hidapi/windows/hid.c")
            .compile("libhidapi.a");
        println!("cargo:rustc-link-lib=setupapi");
        true
    } else if target.contains("apple") {
        cfg.file("vendor/hidapi/mac/hid.c").compile("libhidapi.a");
        println!("cargo:rustc-link-lib=IOKit");
        println!("cargo:rustc-link-lib=CoreFoundation");
        true
    } else {
        panic!("Unsupported target for wooting-analog-sdk-sys");
    };

    // Build SDK to link against.
    let mut cfg = cc::Build::new();
    cfg.warnings(false)
        .extra_warnings(false)
        .file("vendor/src/wooting-analog-sdk.c");
    match (compiled_hidapi, use_hidraw_for_hidapi) {
        (true, _) => {
            cfg.include("vendor/hidapi/hidapi")
                .object(out_dir.join("libhidapi.a"));
        }
        (false, false) => {
            let lib =
                pkg_config::find_library("hidapi-libusb").expect("Unable to find hidapi-libusb");
            for path in lib.include_paths {
                cfg.include(path.to_str().unwrap());
            }
        }
        (false, true) => {
            let lib =
                pkg_config::find_library("hidapi-hidraw").expect("Unable to find hidapi-hidraw");
            for path in lib.include_paths {
                cfg.include(path.to_str().unwrap());
            }
        }
    }
    cfg.compile("wooting-analog-sdk");
}
