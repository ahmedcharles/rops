# Goals and Non-Goals

### Goals

* Full `sops` encrypted file compatibility. Decrypt any `sops` supported format using `rops` and vice versa.
* Be consistent in how credentials are used, set and retrieved across integrations.
* Disincentivize unsecure operations.
* Support standard input (stdin) as an alternative to file paths whenever possible.
* Available as a rust library.
* Support a wide variety of file formats and integrations.

|File formats  | Integrations                  |
| ---          | ---                           |
| ✅ YAML      | ✅ `age` - Asymmetric         |
| ✅ JSON      | ✅ `aws_kms` - Symmetric [^1] |
| ✅ TOML [^2] | ❎`pgp` [^3]                  |
| ❎ INI       | ❎`gcp_kms`                   |
| ❎ ENV       | ❎`azure_kv`                  |
| ❎ BINARY    | ❎`hashicorp_kv`              |

The difference between asymmetric and symmetric integrations is later explained in the [concepts chapter](concepts.md#integration-key-id).

[^1]: AWS KMS effectively becomes a symmetric encryption scheme when it requires private credentials to a remote encryption service, even if that service uses asymmetric encryption internally.

[^2]: [Currently](https://github.com/getsops/sops/pull/812) exclusive to `rops`.

[^3]: Awaiting status update for: [OpenPGP Crypto Refresh](https://datatracker.ietf.org/doc/draft-ietf-openpgp-crypto-refresh/).


### Non-Goals

* Identical CLI to `sops` with full feature parity, see [preliminary non-goals](#preliminary-non-goals).

#### Preliminary Non-Goals

This list includes a collection of SOPS features which are currently not under consideration to be included in `rops`. Nothing here is set in stone, so feel free to open up an [issue](https://github.com/gibbz00/rops/issues) if there's anything you don't agree with 🙂

- The `--output` flag: Use `rops decrypt > FILE_NAME` instead?

- Partial retrieval: Use `rops decrypt FILE | jq` instead?

- Partial modification: 
  Use `rops edit` or `rops decrypt FILE | jq map | rops encrypt --format FORMAT` instead? This will unfortunately skip initialization vector reuse of unchanged values.

- The `--ignore-mac` flag: Deemed too insecure. `rops` files are instead encouraged to be placed under and then recovered with version control systems such as git.

- Manual key rotation (`--rotate/-r`): `rops` will automatically rotate the secret data key upon integration key id removal.

- Integrated formatting configuration: Might be better achieved by piping output through more powerful formatters.

- Integrated secrets publishing: This too might be better handled externally.

- `rops` as a remote key service: Possibly as a separate crate+binary conforming to [KMIP 2.1](https://en.wikipedia.org/wiki/Key_Management_Interoperability_Protocol) or higher.

- Access logging: Better handled by the respective integrations for now. Might become relevant to include in the remote key service.

### Currently missing features

- Sub-process secret passing. 
- Key groups.
- Storing file comments.
- Compute an additional MAC over active integration keys to prevent against manual removal without rotating the secret data key. (Currently not done by SOPS either.)
- Specify keys by `--key-file INTEGRATION PATH` flag.
- Show decrypted metadata with `--show-metadata/-s`. (Note that directly modifying the metadata will most likely break its integrity and prevent future decryption.)
- Integration sub-features such as AWS [Profiles](https://github.com/getsops/sops#27kms-aws-profiles), [Roles](https://github.com/getsops/sops#28assuming-roles-and-using-kms-in-various-aws-accounts) and [Context](https://github.com/getsops/sops#29aws-kms-encryption-context).
