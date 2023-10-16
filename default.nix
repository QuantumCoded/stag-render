{ rustPlatform
, libclang
, clang
, pkg-config
}:

rustPlatform.buildRustPackage rec {
  name = "pictura-${version}";
  version = "0.4.3";

  src = ./.;

  cargoHash = "";

  LIBCLANG_PATH = "${libclang.lib}/lib";

  buildInputs = [
    clang
    libclang
    pkg-config
  ];
}
