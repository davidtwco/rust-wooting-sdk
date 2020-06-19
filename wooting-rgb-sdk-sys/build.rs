use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use pkg_config::find_library;

fn main() {
    // If enabled, attempt to find `wooting-rgb-sdk` via pkg-config. Otherwise, we'll
    // need to build the SDK.
    let use_pkg_config = env::var("WOOTING_RGB_SDK_SHARED").is_ok();
    if use_pkg_config && find_library("wooting-rgb-sdk").is_ok() {
        return;
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
        .header("vendor/src/wooting-rgb-sdk.h")
        .header("vendor/src/wooting-usb.h")
        .generate()
        .expect("Unable to generate bindings for the Wooting RGB SDK");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Unable to write Wooting RGB SDK bindings");

    // Build hidapi to link against.
    let mut cfg = cc::Build::new();
    cfg.warnings(false)
        .extra_warnings(false)
        .include("vendor/hidapi/hidapi");

    if target.contains("linux") {
        let use_pkg_config_for_hidapi = env::var("WOOTING_RGB_SDK_HIDAPI_SHARED").is_ok();
        let use_hidraw_for_hidapi = env::var("WOOTING_RGB_SDK_HIDAPI_HIDRAW").is_ok();
        match (use_pkg_config_for_hidapi, use_hidraw_for_hidapi) {
            (true, false) => {
                let lib = find_library("hidapi-libusb").expect("Unable to find hidapi-libusb");
                for path in lib.include_paths {
                    cfg.include(path.to_str().unwrap());
                }
            }
            (true, true) => {
                let lib = find_library("hidapi-hidraw").expect("Unable to find hidapi-hidraw");
                for path in lib.include_paths {
                    cfg.include(path.to_str().unwrap());
                }
            }
            (false, false) => {
                let libusb = find_library("libusb-1.0").expect("Unable to find libusb-1.0");
                for path in libusb.include_paths {
                    cfg.include(path.to_str().unwrap());
                }

                cfg.file("vendor/hidapi/libusb/hid.c");
            }
            (false, true) => {
                let libudev = find_library("libudev").expect("Unable to find libudev");
                for path in libudev.include_paths {
                    cfg.include(path.to_str().unwrap());
                }

                cfg.file("vendor/hidapi/linux/hid.c");
            }
        }
    } else if target.contains("windows") {
        cfg.file("vendor/hidapi/windows/hid.c");
        println!("cargo:rustc-link-lib=setupapi");
    } else if target.contains("apple") {
        cfg.file("vendor/hidapi/mac/hid.c");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    } else {
        panic!("Unsupported target for wooting-rgb-sdk-sys");
    };

    // Build SDK to link against.
    cfg.file("vendor/src/wooting-rgb-sdk.c")
        .file("vendor/src/wooting-usb.c")
        .compile("wooting-rgb-sdk");
}
