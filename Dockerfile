FROM m.daocloud.io/docker.io/nginx:1.29.0-alpine

COPY dist/quagsire/browser /var/www
