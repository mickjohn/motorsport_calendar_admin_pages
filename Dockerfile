FROM ubuntu:18.04

RUN mkdir -p "/var/motorsport_calendar_admin/" \
  && apt-get update \
  && apt-get install libssl1.1 -y


WORKDIR /usr/bin
COPY docker_config.toml config.toml
COPY log4rs.yml log4rs.yml
COPY target/release/motorsport_calendar_admin_pages .

WORKDIR /var/motorsport_calendar_admin/
COPY www www
# COPY certs certs

WORKDIR /usr/bin
EXPOSE 7000
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="7000"
ENV ROCKET_LOG="normal"
ENV ROCKET_TEMPLATE_DIR="/var/motorsport_calendar_admin/www/"
# ENV ROCKET_CERTS="/var/motorsport_calendar_admin/certs/cert.pem"
# ENV ROCKET_KEY="/var/motorsport_calendar_admin/certs/key.pem"

CMD ["./motorsport_calendar_admin_pages"]
