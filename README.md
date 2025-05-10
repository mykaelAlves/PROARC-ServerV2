# PROARC ServerV2

Based on: <https://github.com/EndPoint-Company/PROARC-server>

## Compilation and configuration

1. Clone the repository;
2. Run `cargo build` inside the cloned directory;
3. Create a *.env* file and configure your server in the following manner:

    ```text
    SERVER_ADDR = "<ip>:<port>"
    DATABASE_URL = "<database url>"
    LOG_PATH = "<path>"
    ADM_PASSWORD = "<password>"
    FILES_BUCKET = "<path>"
    MASTER_TOKEN = "<token name inside LOG_PATH>"
    ```

4. Then go into *target/* and execute the generated binary;
5. Done, the server should be running.
