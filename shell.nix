{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let
  llvmPackages = llvmPackages_latest;
  clang = llvmPackages.clang;
  libclang = llvmPackages.libclang;
  llvm = llvmPackages.llvm;

  stdenv = llvmPackages.stdenv;

in stdenv.mkDerivation {
  name = "msdfgen";

  LIBCLANG_PATH = lib.makeLibraryPath [ libclang ];

  nativeBuildInputs = [
    pkgconfig
    gdb
  ];

  buildInputs = [
    glibc_multi.dev
    clang
    llvm
    libclang
    openssl
    libssh
    libgit2
    freetype
    fontconfig
  ];
}
