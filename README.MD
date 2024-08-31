# Systemic

**Systemic** is a versatile, command execution tool designed to automate the running of commands across multiple remote hosts using SSH. Perfect for DevOps engineers, system administrators, or any power users who need to efficiently manage and monitor large-scale environments.

## Features

- **Parallel Execution**: Run commands simultaneously on multiple hosts.
- **Flexible Host Grouping**: Define host groups in a configuration file to execute commands on specific sets.
- **Error Handling and Retries**: Automatically retries commands on failure, ensuring maximum reliability.
- **Customizable Output Formats**: Supports output in JSON, CSV, or plain text.
- **Detailed Reporting**: Displays consolidated results in a clean tabular format.
- **Seamless SSH Integration**: Leverages the power of SSH2 for secure and reliable remote command execution.
- **Configuration Driven**: Simple JSON files to configure commands and host groups.

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/copyleftdev/systemic.git
   cd systemic
   ```

2. **Build the Project:**

   Make sure you have Rust installed and build the project with:

   ```bash
   cargo build --release
   ```

3. **Configure Environment Variables:**

   Define the following environment variables:

   ```bash
   export SSH_USERNAME=your_username
   export SSH_PASSWORD=your_password
   ```

4. **Set Up Configurations:**

   Create or modify your `config.json` to define your host groups:

   ```json
   {
       "groups": {
           "dev": ["host1", "host2"],
           "prod": ["host3", "host4"],
           "test": ["host5"]
       }
   }
   ```

   Define the commands to run in `commands.json`:

   ```json
   {
       "commands": [
           "ls -al",
           "uname -a"
       ]
   }
   ```

## Usage

Run **Systemic** with:

```bash
./target/release/syntemic --output-format Json --output-file output.json --group prod --retries 3 --command-file commands.json
```

### Arguments

- `--output-format` : Output format (Json, Csv, PlainText)
- `--output-file` : Path to save the output
- `--group` : Host group to target (e.g., "dev", "prod", "test")
- `--retries` : Number of retries on failure
- `--command-file` : JSON file containing the list of commands to execute

## Example

Execute the following to run `ls -al` and `uname -a` on all `prod` hosts:

```bash
./target/release/syntemic --output-format Csv --output-file results.csv --group prod --retries 2 --command-file commands.json
```

## Contributing

We welcome contributions! Feel free to open issues or submit pull requests to help make **Systemic** even better.

## Contact

Created by [@copyleftdev](https://github.com/copyleftdev) - feel free to reach out!
