FROM rust:1.81 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/backend-service
COPY Cargo.* .
# Dummy packages
RUN cargo new backend && cargo new frontend && cargo new common
# Only build dependencies
COPY backend/Cargo.toml backend/
COPY common/Cargo.toml common/
RUN cargo build
RUN cargo build --release
RUN rm -rf backend common

# Now add own source code
COPY common/ common/
COPY backend/src backend/src
COPY backend/Cargo.toml backend/

WORKDIR /usr/src/backend-service/backend
EXPOSE 20103
RUN cargo build --bin redteam-demo-backend --release

# Production image only running the final application
FROM gcr.io/distroless/cc-debian12 as run
COPY --from=build /usr/src/backend-service/target/release/redteam-demo-backend /target-binary/bin/backend-service
CMD ["/target-binary/bin/backend-service"]
