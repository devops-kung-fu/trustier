![trustier](img/trustier128x128.png)

# trustier

[![](https://img.shields.io/badge/Status-ALPHA-red)](CONTRIBUTING.md)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/devops-kung-fu/trustier)

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Arguments](#arguments)
- [Contributing](#contributing)
- [License](#license)

## Overview

`trustier` is an application that enriches CycloneDX Software Bill of Materials with activity, provenance, and activity information from [Trusty](https://trustypkg.dev).

The team at [Stacklok](https://stacklok.com) created [Trusty](https://trustypkg.dev) which they describe as a search for an open source packages to understand their trustworthiness based on activity, provenance, and more. Brought to you by the founders of projects such as Kubernetes and Sigstore.

## The importance of undetstanding supply chain attacks

Supply chain attacks are crucial to address because they can compromise the integrity and security of software systems, even if an organization's proprietary code is secure. These attacks exploit vulnerabilities in the software supply chain, which includes third-party libraries, development tools, and other components used to build, test, deploy, and operate systems. [1]

Trustworthiness of a component is essential because it helps to identify that a component has not been tampered with or compromised by malicious actors. If a component is not trustworthy, it can introduce vulnerabilities or backdoors into the software system, potentially leading to data breaches, system failures, or other security incidents.

Provenance, or the origin and history of a component, is important because it helps establish trust in the component. By understanding where a component came from and how it was developed, organizations can better assess its trustworthiness and potential risks. Components with a well-documented and transparent provenance are generally more trustworthy than those with an unknown or obscure origin.

Reputation is also a crucial factor in assessing the trustworthiness of a component. Components developed and maintained by reputable organizations or individuals with a track record of producing high-quality and secure software are generally more trustworthy than those from unknown or untrusted sources.

To mitigate the risks of supply chain attacks and ensure the trustworthiness of components, organizations should adopt a comprehensive software supply chain security strategy. This strategy may include:

- Regularly updating and auditing third-party components for vulnerabilities and potential threats.

- Monitoring development environments and ensuring secure software development practices are followed by all components used.

- Implementing package origin controls to prevent dependency substitution attacks.

- Conducting thorough vetting and risk assessments of third-party components before integrating them into the software system.

- Maintaining a software bill of materials (SBOM) to track the provenance and dependencies of all components used in the software system.

- Establishing trusted sources and repositories for obtaining components and updates.

- Implementing secure software delivery pipelines and continuous integration/continuous deployment (CI/CD) processes to ensure the integrity of the software supply chain.

By prioritizing the trustworthiness, provenance, and reputation of components, organizations can significantly reduce the risks of supply chain attacks and enhance the overall security and integrity of their software systems.

Sources:

[1] Anti-patterns for security testing - DevOps Guidance [https://docs.aws.amazon.com/wellarchitected/latest/devops-guidance/anti-patterns-for-security-testing.html](https://docs.aws.amazon.com/wellarchitected/latest/devops-guidance/anti-patterns-for-security-testing.html)

## Installation

## Application Arguments

| Argument               | Description                                                                                            |
| ---------------------- | ------------------------------------------------------------------------------------------------------ |
| `<SBOM>`               | The SBOM (Software Bill of Materials) to process. This argument is required.                           |
| `--ratelimit <MS>`     | The time in milliseconds to pause before making requests to https://trustypkg.dev. Defaults to 500 ms. |
| `--output_file <FILE>` | Optional file name to write JSON output to. If not provided, output will be printed to the console.    |

## Example Usage

```sh
# Required sbom argument
trustier sbom_file.json

# Optional ratelimit argument
trustier sbom_file.json --ratelimit 1000

# Optional output_file argument
trustier sbom_file.json --output_file output.json
```

## Troubleshooting

During testing, we found there were some required fields needed in the SBOM in order to be considered valid. Ensure at minimum you have the following fields in your components:

- `name`
- `purl`
- `type`

## Credits

A big thank-you to our friends at [Flaticon](https://www.flaticon.com) for the `trustier` logo.
