FROM rust:1.76 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/backend-service
COPY common/ common/
COPY backend/src backend/src
COPY backend/Cargo.toml backend/
WORKDIR /usr/src/backend-service/backend
EXPOSE 20103
RUN mkdir /target-binary && cargo install --root /target-binary/ --path . --bin redteam-demo-backend

FROM gcr.io/distroless/cc-debian12 as run
COPY --from=build /target-binary/bin/redteam-demo-backend /target-binary/bin/backend-service
CMD ["/target-binary/bin/backend-service"]
