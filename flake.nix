{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  # Had some issues getting wasm-pack to compile /src
  # Some relevant links:
  # https://github.com/rust-lang/rust/issues/125321
  # https://github.com/NixOS/nixpkgs/issues/24744
  # https://discourse.nixos.org/t/help-understanding-error-compiling-rust-to-wasm-in-nixos/27619
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            llvmPackages.bintools
            nixfmt
            nodejs
            wasm-pack
          ];
          CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
        };
      });
}
