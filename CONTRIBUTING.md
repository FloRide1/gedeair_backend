# How to Contribute :)

You want to add more features, edit one or submit a fix ? Great ! Here is some command that can help you

> [!NOTE]
> I provide `flake.nix` for [Nix](https://nixos.wiki/wiki/Nix_package_manager), don't hesitate.

```
# Dependencies (Optional)
nix develop

# Add the exemple as the current .env file (You can edit it)
cp .env.default .env

# Launch the dev docker compose environment (with PostgresSQL + PgAdmin)
docker-compose up -d

# Run tests
cargo test --all-features --workspace

# Run dev Program with -h argument
cargo run -- -h
```

> [!WARNING]
> I won't accept any commit containing a .vscode, .idea, etc... folder/file, i'm happy that you love your IDE / Text Editor but please keep it (and it's files) for yourself
