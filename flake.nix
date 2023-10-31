{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    with inputs;
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };
    in
    {
      packages.x86_64-linux.default = pkgs.callPackage ./. { };

      devShells.x86_64-linux.default = pkgs.mkShell {
        # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        # LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
        ];
      };
    };
}
