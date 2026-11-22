# Applicability facts

A fact is a yes or no statement about a project. Rules use facts in their `Applies when` expression to say which projects they cover.

An agent infers each fact from the code. Then it confirms the answer with the developer.

Some facts imply others. If a fact is yes, every fact it implies is also yes.

### has-ui

- **Question:** Do people use it through a user interface?
- **Definition:** Yes when people interact with it through any UI, such as a GUI, a web page or a terminal UI.

### has-web-ui

- **Question:** Does it serve a UI in a web browser?
- **Definition:** Yes when it serves a UI that people open in a web browser.
- **Implies:** has-ui

### has-cli

- **Question:** Does it have a command-line interface?
- **Definition:** Yes when people or scripts run it from a command line.

### stores-persistent-data

- **Question:** Does it keep data across restarts?
- **Definition:** Yes when data it writes is still there after it stops and starts again.

### stores-user-data

- **Question:** Does it store data about its users or created by them?
- **Definition:** Yes when it stores data about its users, or data that its users create.
- **Implies:** stores-persistent-data

### uses-database

- **Question:** Does it use a database engine?
- **Definition:** Yes when it uses a database engine to store or read data.
- **Implies:** stores-persistent-data

### uses-relational-database

- **Question:** Is the database a relational (SQL) database?
- **Definition:** Yes when the database it uses is relational and is queried with SQL.
- **Implies:** uses-database

### exposes-network-api

- **Question:** Does it accept requests over a network?
- **Definition:** Yes when other programs can send it requests over a network.

### is-network-service

- **Question:** Does it run as a long-lived network service?
- **Definition:** Yes when it keeps running and other programs connect to it over a network.
- **Implies:** exposes-network-api

### operates-service

- **Question:** Does the project run its own production deployment?
- **Definition:** Yes when the project itself runs a deployment in production, not only code that others run.

### handles-authentication

- **Question:** Does it authenticate users or clients?
- **Definition:** Yes when it checks who a user or client is before it allows an action.

### handles-sensitive-data

- **Question:** Does it handle sensitive data?
- **Definition:** Yes when it handles personal, financial, health or credential data, or data of similar sensitivity.

### high-assurance

- **Question:** Could a failure cause serious harm?
- **Definition:** Yes when a failure could cause serious harm, such as to safety, to critical infrastructure or through a large financial loss.

### uses-cryptography

- **Question:** Does it use cryptography?
- **Definition:** Yes when it encrypts data, signs data, hashes data for security, or manages keys.

### requires-fips

- **Question:** Must it run in a FIPS 140-3 validated configuration?
- **Definition:** Yes when it must run with cryptography in a FIPS 140-3 validated configuration.
- **Implies:** uses-cryptography

### ships-container-image

- **Question:** Does it publish a container image?
- **Definition:** Yes when it publishes an OCI container image.

### deploys-to-kubernetes

- **Question:** Does it ship Kubernetes manifests or Helm charts?
- **Definition:** Yes when it ships Kubernetes manifests or Helm charts.

### is-library

- **Question:** Do others use it as a library or SDK?
- **Definition:** Yes when other projects use it as a library or an SDK.

### distributed-as-package

- **Question:** Is it released as an installable package or binary?
- **Definition:** Yes when it is released as a package or a binary that people install.

### accepts-external-contributions

- **Question:** Can people outside the core team contribute?
- **Definition:** Yes when people outside the core team may send changes to it.

### uses-ai-models

- **Question:** Does it call or run AI models?
- **Definition:** Yes when it calls an AI model or runs one.

### exposes-mcp-server

- **Question:** Does it provide an MCP server?
- **Definition:** Yes when it provides a Model Context Protocol (MCP) server.

### has-multiple-users

- **Question:** Does more than one person or account use one installation?
- **Definition:** Yes when more than one person or account uses the same installation.

### is-commercial

- **Question:** Is any part of it sold?
- **Definition:** Yes when any edition, tier, feature or licence of it is sold.

### is-foss

- **Question:** Is it released under an OSI-approved licence?
- **Definition:** Yes when it is released under a licence that the Open Source Initiative (OSI) has approved.

### sold-in-eu

- **Question:** Is it placed on the EU market?
- **Definition:** Yes when it is placed on the market of the European Union.

### processes-eu-personal-data

- **Question:** Does it process personal data of people in the EU?
- **Definition:** Yes when it processes personal data of people who are in the European Union.
