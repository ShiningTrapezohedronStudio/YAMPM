{
  description = "YAMPM Dev Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable&shallow=1";

    flake-parts.url = "github:hercules-ci/flake-parts?shallow=1";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    rust-overlay = {
      url = "github:oxalica/rust-overlay?shallow=1";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-compat = {
      url = "github:edolstra/flake-compat?shallow=1";
      flake = false;
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { self', pkgs, system, ... }:
        let

          runtimeDeps = with pkgs; [
            openssl
          ];

          buildDeps = with pkgs; [
            pkg-config
            rustPlatform.bindgenHook
          ];

          devDeps = with pkgs; [
            lefthook
            statix
            mold
            clang
            libllvm
            yamlfmt
          ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.workspace.package.rust-version;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.minimal;
            rustc = pkgs.rust-bin.stable.latest.minimal;
          };

          rustPackage = features:
            rustPlatform.buildRustPackage {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
                allowBuiltinFetchGit = true;
              };
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeDeps;
            };

          mkDevShell = rustc:
            pkgs.mkShell.override {
              stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
            } {
              RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') []) ++ ["-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold"];
              BINDGEN_EXTRA_CLANG_ARGS =
                (builtins.map (a: ''-I"${a}/include"'') [pkgs.glibc.dev]) ++ [
                  ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
                ];
              CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
              shellHook = ''
                export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeDeps}"
              '';
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };
        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          packages = {
            yampm = rustPackage "";
            default = self'.packages.yampm;
          };

          devShells = {
            nightly = mkDevShell (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" ];
            }));
            stable = mkDevShell pkgs.rust-bin.stable.latest.default;
            msrv = mkDevShell pkgs.rust-bin.stable.${msrv}.default;

            default = self'.devShells.nightly;
          };
        };
    };
}
