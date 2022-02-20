FROM rust:alpine AS builder

COPY dev-support /tmp/dev-support
RUN set -eu; \
  apk add --no-cache bash; \
  /tmp/dev-support/bin/install-deps;

WORKDIR /build
COPY . /build

ARG DEBUG=0
ARG CARGO_ARGS=""
ARG BIN_NAME="xenon"

ENV RUST_BACKTRACE=1
ENV PROTOC="/usr/bin/protoc"
ENV PROTOC_INCLUDE="/usr/include"

RUN set -eu; \
  if [ "${DEBUG}" == "1" ]; then \
    cargo build ${CARGO_ARGS}; \
    cp target/debug/${BIN_NAME} /usr/bin; \
  else \
    cargo build ${CARGO_ARGS} --release; \
    cp target/release/${BIN_NAME} /usr/bin; \
    INITIAL_SIZE="$(sh -c 'sz="$(du -sk /usr/bin/'${BIN_NAME}')" ; echo "${sz%%[[:space:]]*}"')"; \
    strip /usr/bin/${BIN_NAME}; \
    FINAL_SIZE="$(sh -c 'sz="$(du -sk /usr/bin/'${BIN_NAME}')" ; echo "${sz%%[[:space:]]*}"')"; \
    REMOVED_SIZE=$((INITIAL_SIZE - FINAL_SIZE)); \
    echo "Cleaning process removed ${REMOVED_SIZE}KB"; \
    echo "Dropped binary size from ${INITIAL_SIZE}KB to ${FINAL_SIZE}KB"; \
  fi; \
  file /usr/bin/${BIN_NAME}; \
  ldd /usr/bin/${BIN_NAME}; \
  ${BIN_NAME} --version;


FROM scratch

ENV RUST_BACKTRACE=${DEBUG}

COPY --from=builder /usr/bin/${BIN_NAME} /usr/bin/

ENV RUST_BACKTRACE=${DEBUG}

ENTRYPOINT [ "/usr/bin/xenon", "version" ]
