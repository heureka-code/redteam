FROM rust:1.76 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN rustup target add wasm32-unknown-unknown && cargo install trunk

WORKDIR /usr/src/frontend-service
COPY common/ common/
COPY frontend/ frontend/
WORKDIR /usr/src/frontend-service/frontend

ARG HOSTNAME
RUN echo "pub fn get_database_url() -> std::sync::Arc<str> { \"https://${HOSTNAME}/redteam-backend\".into() }" > src/api/config.rs

EXPOSE 8080
RUN trunk build --release
CMD ["trunk", "serve", "--release"]

#RUN mkdir /target-binary && cargo install --root /target-binary/ --path backend

#FROM gcr.io/distroless/cc-debian10

#COPY --from=build /target-binary/bin/redteam-demo-backend /usr/local/bin/backend-service

#CMD ["/target-binary/bin/redteam-demo-backend"]
