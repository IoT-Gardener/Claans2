# Claans2

A Rust rebuild of the Claans app.

## Introduction

Claans2 lets you create Claans that you can assign Users to, to facilitate team-building and healthy competition in the workplace.

## Key Features

<!-- TODO: Populate with actual features -->
- Automatic score calculations and submissions
- Long term score tracking across multiple seasons
- Helper utilities for actions like shuffling teams, resetting scores, etc
- Dynamic rule definitions to let you customize how points are earned

> [!IMPORTANT]
> This list is incomplete and subject to drastic changes

## How to Build Locally

To build this project locally, you'll need [Git](https://git-scm.com/), [Docker](https://www.docker.com/) (including compose), and [Rust](https://www.rust-lang.org/) (via rustup).

Clone and enter the repository

```Shell
git clone https://github.com/IoT-Gardener/Claans2 && cd Claans2
```

From inside the repository, create a new environment file with `touch .env`, open the file with your favourite text editor and insert the following template with updated values

```Dotenv
POSTGRES_USER=<root>
POSTGRES_PASSWORD=<password>
POSTGRES_DB=<postgres>
APP_DB_USER=<claans>
APP_DB_PASS=<claans>
APP_DB_NAME=<claans>
DATABASE_URL=postgres://${APP_DB_USER}:${APP_DB_PASS}@localhost:5433/${APP_DB_NAME}
```

> [!NOTE]
> Values with surrounding '< >' brackets need updating before running.
> [!NOTE]
> The default postgres port is 5432, but the compose.yml configures the host port as 5433 to avoid clashes with any local postgres instances.
> You can update this as required.

With the environment file created, use docker compose to start the postgresql database.
This should pull the postgres image then create and configure the database.

```Shell
docker compose up
```

Next, using Cargo, install the diesel cli tool, which will manage the database tables.

```Shell
# diesel depends on the libpq package for postgres support
apt-get install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
diesel setup
```

Finally, start the project

```Shell
cargo run
```
