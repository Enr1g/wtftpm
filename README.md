# WTFTPM

## Disclaimer

This is a dangerous thing to use because:

* That's a pet project made for fun. The quality is corresponding.
* It helps you to **lessen** your security. No two ways about it.

Think twice before using it. Don't use it if you're not able to write something like this yourself.

## Why

Some of us are using hardware security devices (e.g. TPM) to store private keys. Some of this devices require a mandatory pin to access this private keys. This wrapper helps you to cache pin and login automatically.

## Features

* All functions from [pkcs11.h](include/pkcs11.h) that weren't explicitly wrapped are accessible.
* Storing secrets with libsecret is supported (gnome-keyring and other compatible should work).
* Auto-login on `C_OpenSession` and `C_Login` for slots with cached pins and only for them.

## Limitations

* Tested on Linux x86_64 only.
* Tested on https://github.com/tpm2-software/tpm2-pkcs11/ only but is probably going to work with any compatible pkcs11 implementation.
* Supports caching pin for slot labels only. Slot IDs are not supported.

## Usage

Project provides you with binary helper `wtftpm` and shared library `libwtftpm.so`.

`wtftpm` is used for basic information gathering and to store pin using libsecret.

```
Usage: wtftpm <COMMAND>

Commands:
  libsecretexample       Print example of config using libsecret
  plainexample           Print example of plaintext config
  configpath             Print config path
  checkconfig            Check current config (warning: will print your plain secrets)
  completion             Generate completion for shell
  storepinwithlibsecret  Store pin using libsecret
  help                   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

`libwtftpm.so` is used to wrap your pkcs11 compatible library. It reads config from `wtftpm configpath`. The typical config can be obtained via `wtftpm libsecretexample` or `wtftpm plainexample`.

Create your config pointing to the pkcs11 library you want to wrap. Use `libwtftpm.so` instead in your applications.

## Installation

```bash
git clone https://github.com/Enr1g/wtftpm
cd wtftpm
cargo build --release
# If you want to install wtftpm to cargo bins
# cargo install --path .
# Otherwise
cp target/release/wtftpm /some/location/in/your/PATH/
cp target/release/libwtftpm.so /some/location/for/libs/
```

## FAQ

Q: What are namespaces in config?

A: Only the matter of convenience for those who wants to store pins for multiple TPM in libsecret. Default is good enough for most cases.

Q: X still asks me for pin.

A: Check if you're using `libwtftpm.so`. If so, the problem is probably in you having other available slots with no cached pin. Chromium-based browsers will ask you for pin anyway if you're trying to list avaiable certificates or just searching in settings. Empty pin will suffice if you're sure your pin for slot is cached. Ignoring the prompt is fine too if you don't want to list certificates. Also, `libwtftpm.so` can't prevent arbitrary software to ask you for a pin in prior.

Q: How can I use your wrapper with X?

A: The same way you used your original `whatever-pkcs11.so`. If you think that some of this commands are confusing or hard, just drop it. Forget about it and enter your goddamn pin. Some examples:

Generate public key for OpenSSH:

```bash
ssh-keygen -D libwtftpm.so
```

Connect to ssh host:

```bash
ssh -I libwtftpm.so your_host
```

Firefox:

* Add `libwtftpm.so` as a Security Device in Settings.

Chromium-based (close applications that use nss first, really):

```bash
modutil -dbdir "sql:$HOME/.pki/nssdb/" -libfile libwtftpm.so -add WTFTPM
```

Undo the previous command (again, close applications that use nss first):

```bash
modutil -dbdir "sql:$HOME/.pki/nssdb/" -delete WTFTPM
```

OpenVPN:

* Don't use `libwtftpm.so` for OpenVPN if you can cache your pin with e.g. NetworkManager.
* If you can't, change `pkcs11-providers /your/original/pkcs11_compatible.so` to `pkcs11-providers libwtftpm.so` in your config (don't forget to backup your original config).

Q: How do your store pins in memory?

A: I use [secstr](https://docs.rs/secstr/latest/secstr/). It should prevent secrets from getting to swap or coredump. Also they are zeroed on destruction.

Q: Do you consider issues and pull requests?

A: Yes, but no guarantees.

Q: Can I use your software to implement something of my own?

A: Absolutely.

