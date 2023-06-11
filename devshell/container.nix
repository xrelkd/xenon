{ name
, version
, dockerTools
, xenon
, buildEnv
, ...
}:

dockerTools.buildImage {
  inherit name;
  tag = "v${version}";

  copyToRoot = buildEnv {
    name = "image-root";
    paths = [ xenon ];
    pathsToLink = [ "/bin" ];
  };

  config = {
    Entrypoint = [ "${xenon}/bin/xenon" ];
  };
}
