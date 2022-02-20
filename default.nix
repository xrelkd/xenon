{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xenon";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-vVr4/S0wW1FcQxoC8N/3mLI2i0KQUoQpvyhjL8pIcCk=";

  meta = with lib; {
    homepage = "https://github.com/xrelkd/xenon";
    license = with licenses; [ mit asl20 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
