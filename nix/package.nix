{ lib, rustPlatform, fetchFromGitHub }:

rustPlatform.buildRustPackage rec {
  pname = "metropolis";
  version = "0.1.2";

  src = fetchFromGitHub {
    owner = "5c0";
    repo = "metropolis";
    rev = "v${version}";
    hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # Placeholder
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # Placeholder

  meta = with lib; {
    description = "A cinematic, retro-cyberpunk system monitor for the terminal powered by Rust";
    homepage = "https://github.com/5c0/metropolis";
    license = licenses.mit;
    maintainers = with maintainers; [ ];
    mainProgram = "metropolis";
  };
}
