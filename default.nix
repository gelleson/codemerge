{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "codemerge";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ pkgs.pkg-config ];
  buildInputs = [ pkgs.openssl ];

  meta = with pkgs.lib; {
    description = "CodeMerge - a command-line tool for merging multiple code files into a single output file";
    homepage = "https://github.com/gelleson/codemerge";
    license = licenses.mit;
    maintainers = [ maintainers.gelleson ];
  };
}