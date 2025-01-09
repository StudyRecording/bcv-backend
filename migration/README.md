# Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```


- 生成entity/src文件

  ```sh
  sea-orm-cli generate entity -l --date-time-crate chrono --with-serde both -o entity/src
  ```

- 生成migration文件

  ```sh
  # 生成表创建文件
  sea-orm-cli migrate generate "create table {表明}"

  ```
- 建表
  ```sh 
  sea-orm-cli migrate up
  ```
