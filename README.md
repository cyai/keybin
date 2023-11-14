# keybin

`keybin` is a command-line interface tool built with Rust and CLAP for secure management of API and secret keys using the Panega Vault service.

## Installation

To install `keybin`, use Cargo:

```bash
$ cargo install keybin
```

## Usage

### Adding a Secret

Add a secret to the vault.

```bash
$ keybin add [NAME]
```

Prompts:
- Enter the name/key for this secret: [user input]
- Enter the value for this secret: [user input]

### Getting a Secret

Retrieve a secret from the vault.

```bash
$ keybin get [NAME]
```

This command will output the value associated with the specified secret.

### Listing All Secrets

List all secrets stored in the vault.

```bash
$ keybin list
```

### Deleting a Secret

Delete a specific secret from the vault.

```bash
$ keybin delete [NAME]
```

### Updating a Secret

Update the information of a stored secret.

```bash
$ keybin update [NAME]
```

Prompts will allow modifications to the secret's details.

### Getting Help

To access command-specific help or general information:

```bash
$ keybin help [COMMAND]
```

Replace `[COMMAND]` with the command you need help with.
