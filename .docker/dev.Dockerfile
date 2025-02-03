FROM "node:slim" AS node_dev
ARG UID=1000
ARG GID=1000

USER ${UID}:${GID}
