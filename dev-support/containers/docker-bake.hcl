group "default" {
  targets = ["xenon", "xenon-distroless"]
}

variable "TAG" {
  default = "develop"
}

variable "REPOSITORY" {
  default = "ghcr.io/xrelkd/xenon"
}

target "xenon" {
  dockerfile = "dev-support/containers/scratch/Containerfile"
  tags       = ["${REPOSITORY}:${TAG}"]
  platforms  = ["linux/amd64"]
}

target "xenon-distroless" {
  dockerfile = "dev-support/containers/debian/Containerfile"
  tags       = ["${REPOSITORY}:${TAG}-bullseye-slim"]
  platforms  = ["linux/amd64"]
}
