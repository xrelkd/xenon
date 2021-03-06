{ stdenv
, lib
, rustPlatform
, openssl
, pkg-config
, libiconv
, darwin
}:

rustPlatform.buildRustPackage rec {
  pname = "xenon";
  version = "0.5.0";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [ openssl ]
    ++ lib.optional stdenv.hostPlatform.isDarwin [ darwin.apple_sdk.frameworks.Security libiconv ];

  nativeBuildInputs = [ pkg-config ];

  meta = with lib; {
    homepage = "https://github.com/xrelkd/xenon";
    license = with licenses; [ mit asl20 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
