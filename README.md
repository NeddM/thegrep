# thegrep

Script that allows you to grep files, it's just a grep clone made for learning purpose. I practiced things like argument reading, file reading and much more!

## Prerequisites

Before using this script, ensure that you have the following prerequisites installed on your system:

-   Rust: You can install Rust by following the instructions on the official website: Rust Installation.
-   Git: Make sure Git is installed on your system and configured with the repository where you want to make commits.

## How to Use

Follow these steps to use the "commit2" script:

1. Clone or navigate to the Git repository where you want to make commits.

2. Save the "thegrep" Rust script to a file, e.g., thegrep.rs.

3. Compile the Rust script by running the following command:

```shell
cargo build --release
```

4. Save the exe on `System32` in Windows or `/usr/local/bin/` in Linux.

5. Run the compiled executable:

```shell
thegrep
```

6. Follow the on-screen prompts:

```shell
thegrep -h
```

## After and before options

You have two flags that allows you to get lines after and before your grepped line. Using `-A` and `-B`.
