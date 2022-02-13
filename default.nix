{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xenon";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-1Tx/UfLlZYH256P0/7H01C58/XFhjG45J2NGjhaR8xg=";

  meta = with lib; {
    homepage = "https://github.com/xrelkd/xenon";
    license = with licenses; [ mit asl20 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
