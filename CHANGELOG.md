# 0.2.1

## Breaking changes
- None

## New features
- None

## Other changes
- There is no longer a need for 7zip. The creation of a secure archive is now done using the zip-rs library.

# 0.2.0

## Breaking changes
- On output, a single archive file is generated, containing the database dump and system files in the form of zip/tar.gz. 
- The archive filename has been modified.

## New features
- Added the ability to create a password-protected archive.

## Other changes
- None

# 0.1.1

## Breaking changes
- None

## New features
- The ability to specify which files to exclude from the backup process has been added. Additionally, all files related to SQLite are always excluded from backup, as the database itself is backed up separately in the first step.

## Other changes
- None

# 0.1.0

## Breaking changes
- None

## New features
- Added functionality for backup both databases (PostgreSQL, MySQL, SQLite) and files.

## Other changes
- None