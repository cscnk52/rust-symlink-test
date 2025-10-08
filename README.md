# Git Symlink Test on Windows

This repository demonstrates Git's symlink behavior on Windows systems and how the `core.symlinks` configuration affects file handling.

## Problem Description

On Windows, Git's treatment of symbolic links depends on the `core.symlinks` configuration setting. When this setting is disabled (the default), Git treats symlinks as regular text files containing the link target path. When enabled, Git creates actual symbolic links on the filesystem.

## Behavior Comparison

### With `core.symlinks=false` (Default)

When cloning with symlinks disabled, the symlink file will be treated as a plain ASCII text file:

```powershell
> git clone -c core.symlinks=false https://github.com/cscnk52/rust-symlink-test
> dir .\rust-symlink-test\src\fixture\

    Directory: C:\code\clone\rust-symlink-test\src\fixture

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---           2025/10/8    22:04             25 this_is_a_symlink_file.txt
-a---           2025/10/8    22:04             19 this_is_a_text_file.txt
```

### With `core.symlinks=true` (Recommended)

When cloning with symlinks enabled, Git creates actual symbolic links:

```powershell
> git clone -c core.symlinks=true https://github.com/cscnk52/rust-symlink-test
> dir .\rust-symlink-test\src\fixture\

    Directory: C:\code\clone\rust-symlink-test\src\fixture

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
la---           2025/10/8    21:57              0 this_is_a_symlink_file.txt -> .\this_is_a_text_file.txt
-a---           2025/10/8    21:57             19 this_is_a_text_file.txt
```

## Recommended Configuration

To properly handle symlinks on Windows, set the global Git configuration:

```powershell
git config --global core.symlinks true
```

## Prerequisites

Creating symbolic links on Windows requires one of the following:

- **Administrator privileges**: Run Git from an elevated command prompt
- **Developer Mode**: Enable Developer Mode in Windows Settings (Windows 10/11)
- **SeCreateSymbolicLinkPrivilege**: The user account must have this privilege

## Troubleshooting

If symlinks still cannot be created even with `core.symlinks=true`, please check:

1. Ensure you have the necessary permissions (see Prerequisites above)
2. Verify your Git version supports symlinks on Windows
3. Check if your filesystem supports symbolic links (NTFS does, FAT32 doesn't)

If you continue to experience issues, please [open an issue](https://github.com/cscnk52/rust-symlink-test/issues).

## License

This is a test/demonstration repository for educational purposes.
