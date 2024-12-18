FROM rust:1.81 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN rustup target add wasm32-unknown-unknown && cargo install trunk

WORKDIR /usr/src/frontend-service
COPY common/ common/
COPY frontend/ frontend/
WORKDIR /usr/src/frontend-service/frontend

ARG RT_HOSTNAME

EXPOSE 8080
RUN trunk build --release

FROM nginx:1.26.0 as run
WORKDIR /page
RUN rm /etc/nginx/conf.d/ -r && mkdir /etc/nginx/conf.d
COPY --from=build /usr/src/frontend-service/frontend/dist/ /page/dist/
COPY frontend/frontend.conf /etc/nginx/conf.d/
