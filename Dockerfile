####################

FROM node AS frontend
COPY frontend /frontend
WORKDIR /frontend
RUN yarn
RUN yarn build

####################

FROM rust AS backend
COPY backend /backend
WORKDIR /backend
RUN cargo build --release

####################

FROM debian:jessie-slim AS release

ENV STATIC_FILES /static
ENV STORAGE /store

COPY --from=frontend /frontend/dist /static
COPY --from=backend /backend/target/release/scaling-engine /

EXPOSE 8080

ENTRYPOINT /scaling-engine

####################
