FROM postgres:16-bullseye

ENV TZ Asia/Tokyo

COPY ./entrypoint /docker-entrypoint-initdb.d
RUN chmod +x -R /docker-entrypoint-initdb.d/

EXPOSE 5432