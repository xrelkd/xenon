{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "xenon";
  version = "0.2.0";

  src = ./.;

  cargoSha256 = "sha256-Qwc4AHgYC62eJmwr+YJf0pyYPSAwY7OPkb6t8MqSBxc=";

  meta = with lib; {
    homepage = "https://github.com/xrelkd/xenon";
    license = with licenses; [ mit asl20 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
