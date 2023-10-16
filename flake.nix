{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs:
    with inputs;
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        config.allowUnfree = true;
      };
    in
    {
      packages.x86_64-linux.default = pkgs.callPackage ./. { };

      devShells.x86_64-linux.default = pkgs.mkShell {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rustfmt

          xorg.libxcb
          xorg.libX11
        ];
      };
    };
}
