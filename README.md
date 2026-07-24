# 🛠️ cr-storage

`cr-storage` is a Rust-based Command Line Interface (CLI) utility designed to automate the creation, mounting, and unmounting of both encrypted (LUKS2) and unencrypted storage containers. It simplifies the process of turning a regular file into a usable filesystem by utilizing Linux loop devices (`/dev/loop`).

## ✨ Features

- **⚙️ Flexible Container Creation**: Generate a blank image file of a specific size, set its ownership, and format it as either a standard unencrypted filesystem or a secure LUKS2 encrypted volume.
- **🔌 Seamless Mount/Umount**: Easily mount your file-backed containers to any directory and safely unmount them when done.
- **🔒 Smart Encryption Toggle**: Automatically handles password prompts and device mapping when `--encrypt` is used, or mounts standard images directly without asking for credentials.
- **🔄 Automated Lifecycle**: Manages loop device attachment and detachment under the hood.

## 📋 Prerequisites

This tool interacts with Linux kernel subsystems and requires the following utilities installed on your system:
- `cryptsetup` (only required when using LUKS2 encryption features)
- `losetup` (for loop device management)
- `mount` and `umount`
- `sudo` privileges (required for device and filesystem manipulation)

## 🚀 Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/nav-uue/cr-storage.git
cd cr-storage
cargo build --release
```

The compiled binary will be available at `./target/release/cr-storage`.

## 💻 Usage

> ⚠️ **Note**: Since this tool modifies block devices and mounts filesystems, all commands must be run with `sudo`.

### 1. Create and Format an Image (`diskmake`)

Creates a new image file of a given size, sets its owner, links it to an available loop device, formats it, and mounts it.

#### 📂 Option A: Unencrypted Container (Default)
Creates a standard, unencrypted filesystem. No encryption password will be requested.
```bash
sudo cr-storage diskmake --user file_owner --image /path/to/image.img --size 32GB --path /your/mount/point
```

#### 🔒 Option B: Encrypted Container (LUKS2)
Adding the `--encrypt` (or `-e`) flag triggers `cryptsetup` to format the device with LUKS2 encryption and will prompt you for a password.
```bash
sudo cr-storage diskmake --encrypt --user file_owner --image /path/to/image.img --size 32GB --path /your/mount/point
```

**Short Flags Example (Encrypted):**
```bash
sudo cr-storage diskmake -e -u file_owner -i /path/to/image.img -s 32GB -p /your/mount/point
```

---

### 2. Mount an Existing Image (`mount`)

Finds an available loop device, links the image, and mounts the filesystem to your directory.

#### 📂 Option A: Unencrypted Container
Mounts a regular unencrypted image directly without prompting for a password.
```bash
sudo cr-storage mount --image /path/to/image.img --path /your/mount/point
```

#### 🔒 Option B: Encrypted Container
Unlocks the LUKS2 container (prompts for the password) and maps it before mounting.
```bash
sudo cr-storage mount --encrypt --image /path/to/image.img --path /your/mount/point
```

**Short Flags Example (Encrypted):**
```bash
sudo cr-storage mount -e -i /path/to/image.img -p /your/mount/point
```

---

### 3. Unmount the Image (`umount`)

Safely unmounts the filesystem and detaches the loop device. If the volume was encrypted, it automatically closes the LUKS2 secure mapping as well.

```bash
sudo cr-storage umount --path /your/mount/point
```

**Short Flags:**
```bash
sudo cr-storage umount -p /your/mount/point
```

## ⚙️ Arguments Reference

- `--encrypt` / `-e` : Optional. Enables LUKS2 encryption features for creation or mounting.
- `--user` / `-u` : Sets the owner of the created image file.
- `--image` / `-i` : Path where the container file is or will be stored.
- `--size` / `-s` : Size of the container (e.g., `32MB`, `10GB`).
- `--path` / `-p` : The target directory directory where the filesystem will be mounted.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
