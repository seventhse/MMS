# Marketing Monitor System (MMS)

Marketing Monitor System (MMS) provides a set of Just commands for development workflow automation.

## Commands

### Installation

Install required development tools:

```bash
just install
```

This will install:

- cargo-watch: For auto-reloading development server
- sea-orm-cli: For database migrations and entity generation

### Development Server

Run development server with auto-reload:

```bash
just server
```

Build and run production server:

```bash
just pre-server
```

### Database Management

Generate a new migration file:

```bash
just gen-migration <table_name>
```

Generate entity modules from database schema:

```bash
just gen-entity
```

Clean generated entity files:

```bash
just clean-entity
```

Run migration commands:

```bash
just migrate <command>
```

Where `<command>` can be:

- up: Apply all pending migrations
- down: Revert last migration
- fresh: Drop all tables and reapply migrations
- reset: Revert all migrations then reapply
- status: Show migration status
