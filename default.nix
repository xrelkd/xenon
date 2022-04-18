{ lib
, rustPlatform
, openssl
, pkg-config
}:

rustPlatform.buildRustPackage rec {
  pname = "xenon";
  version = "0.3.0";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config ];

  meta = with lib; {
    homepage = "https://github.com/xrelkd/xenon";
    license = with licenses; [ mit asl20 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
