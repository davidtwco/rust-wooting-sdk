let
  sources = import ./nix/sources.nix;
  mozillaOverlay = import (sources.nixpkgs-mozilla);
  pkgs = import sources.nixpkgs { overlays = [ mozillaOverlay ]; };
in
pkgs.mkShell {
  name = "rust-wooting-sdk";
  buildInputs = with pkgs; [
    (rustChannelOf { rustToolchain = ./rust-toolchain; }).rust

    git

    pkg-config
    libusb
    libxml2
    openssl

    # Required by bindgen
    clang
    clang-tools
    llvmPackages.llvm
    llvmPackages.libclang.lib
  ];

  # Always show backtraces.
  RUST_BACKTRACE = 1;

  # Set `LIBCLANG_PATH` for bindgen.
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}

# vim:foldmethod=marker:foldlevel=0:ts=2:sts=2:sw=2:nowrap
