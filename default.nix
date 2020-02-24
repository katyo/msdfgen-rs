{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let
  llvmPackages = llvmPackages_latest;
  clang = llvmPackages.clang-polly-unwrapped;
  libclang = llvmPackages.libclang;
  llvm = llvmPackages.llvm-polly;

  stdenv = llvmPackages.stdenv;

in stdenv.mkDerivation {
  name = "msdfgen";

  LIBCLANG_PATH = "${libclang}/lib";

  nativeBuildInputs = [
    pkgconfig
    gdb
    #clang
    #llvm
    #libclang
  ];

  buildInputs = [
    glibc_multi.dev
    clang
    llvm
    libclang
    openssl
    libssh
    libgit2
    freetype.dev
  ];
}
