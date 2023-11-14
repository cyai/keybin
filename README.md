
# keybin

`keybin` is a command-line interface tool built with Rust and CLAP for secure management of API and secret keys using the Panega Vault service.

## Installation

To install `keybin`, use Cargo:

```bash
$ cargo install keybin
```

## Initial Setup

Before using `keybin`, you need to perform the initial setup to configure your Panega Vault API token.

1. **Get Panega Vault API Token:**
   - Go to [Panega Vault](https://console.pangea.cloud/service/vault).
   - Obtain your API token from the dashboard.

2. **Create .env File:**
   - Create a new file named `.env` in the root directory of your project.
   - Open the `.env` file in a text editor.

3. **Add API Token to .env:**
   - Inside the `.env` file, add the following line, replacing `YOUR_API_TOKEN` with your actual Panega Vault API token:

     ```env
     PANEGA_VAULT_API_TOKEN=YOUR_API_TOKEN
     ```

   - Save and close the `.env` file.

Now, your `keybin` tool is configured with the Panega Vault API token, and you can start using the commands described below.

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

This command will copy the value associated with the specified secret to your clipboard.

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
```