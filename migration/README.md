# Database Migrations

This directory contains SeaORM migrations for the church management system.

## Running Migrations

### Apply all pending migrations

```bash
cd migration
cargo run -- up
```

### Rollback the last migration

```bash
cargo run -- down
```

### Check migration status

```bash
cargo run -- status
```

### Refresh (rollback all and reapply)

```bash
cargo run -- refresh
```

### Reset (rollback all)

```bash
cargo run -- reset
```

## Migration Order

1. `m20260203_000001` - Create church schema
2. `m20260203_000002` - Create users table
3. `m20260203_000003` - Create user_profiles table
4. `m20260203_000004` - Create user_addresses table
5. `m20260203_000005` - Create zones table
6. `m20260203_000006` - Create cell_groups table
7. `m20260203_000007` - Create roles table
8. `m20260203_000008` - Create ministries table
9. `m20260203_000009` - Create user_memberships table
10. `m20260203_000010` - Create user_roles table
11. `m20260203_000011` - Create user_ministries table
12. `m20260203_000012` - Create attendances table
13. `m20260203_000013` - Create givings table

## Database Structure

All tables are created in the `church` schema:

- church.users
- church.user_profiles
- church.user_addresses
- church.user_memberships
- church.zones
- church.cell_groups
- church.roles
- church.user_roles
- church.ministries
- church.user_ministries
- church.attendances
- church.givings
