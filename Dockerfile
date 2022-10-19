FROM rust:slim
COPY ./target/release/my-db-sync-report ./target/release/my-db-sync-report
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/my-db-sync-report"]