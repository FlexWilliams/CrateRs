FROM rust:latest

WORKDIR /var/www/crate-rs

# TODO: omit non-necesary files (e.g., project_mgmt, target folder)
COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["craters"]