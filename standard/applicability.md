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
- **Implies:** has-ui, uses-network

### has-cli

- **Question:** Does it have a command-line interface?
- **Definition:** Yes when people or scripts run it from a command line.

### uses-network

- **Question:** Does it send or receive data over a network?
- **Definition:** Yes when it opens network connections or accepts them.

### collects-telemetry

- **Question:** Does it send usage or diagnostic data to the project or a third party?
- **Definition:** Yes when it sends data about how it is used, or crash and diagnostic reports, to the project or to a third party, rather than only to its own operator's systems.
- **Implies:** uses-network

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
- **Implies:** uses-network

### exposes-web-api

- **Question:** Does it accept HTTP-based requests from other software?
- **Definition:** Yes when other programs can send it requests over HTTP or a protocol carried over HTTP, such as REST, GraphQL, gRPC or WebSocket. Endpoints only for operating it (such as health checks and metrics), and listeners bound only to the loopback interface (such as a command-line tool's sign-in callback), do not count.
- **Implies:** exposes-network-api

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

### handles-personal-data

- **Question:** Does it handle personal data?
- **Definition:** Yes when it collects, stores or processes information about an identified or identifiable person, such as a name, an email address, an account identifier or a location.
- **Implies:** handles-sensitive-data

### uses-secrets

- **Question:** Does it need secrets to run?
- **Definition:** Yes when it needs passwords, API keys, tokens, private keys or similar secrets at run time.

### high-assurance

- **Question:** Could a failure cause serious harm?
- **Definition:** Yes when a failure could cause serious harm, such as to safety, to critical infrastructure or through a large financial loss.

### has-memory-unsafe-code

- **Question:** Does any of its own code run without memory safety?
- **Definition:** Yes when the project's own code includes a language that does not guarantee memory safety (such as C, C++ or assembly), or uses the unsafe features of a memory-safe language (such as Rust `unsafe` blocks, Go's `unsafe` package or C# `unsafe` code). Code in dependencies does not count.

### uses-concurrency

- **Question:** Does its code run in parallel and share data?
- **Definition:** Yes when its own code runs threads, goroutines, parallel tasks or processes that share memory or other mutable data. Asynchronous code on a single thread does not count.

### parses-untrusted-input

- **Question:** Does it parse data that an untrusted source could have written?
- **Definition:** Yes when it parses files, documents, images, archives, network messages or other structured input that a person or system it does not trust could have written. Configuration that only its operator writes does not count.

### uses-cryptography

- **Question:** Does it use cryptography?
- **Definition:** Yes when it encrypts data, signs data, hashes data for security, or manages keys.

### implements-cryptography

- **Question:** Is providing cryptography to other software the main purpose of the project?
- **Definition:** Yes when the project's main product is a library or component whose public interface is cryptographic primitives or protocols, such as a cryptography library or a TLS implementation. Code that only calls a cryptography library does not count, and neither does an application that also exposes cryptographic helpers of its own.
- **Implies:** uses-cryptography

### requires-fips

- **Question:** Must it run in a FIPS 140-3 validated configuration?
- **Definition:** Yes when it must run with cryptography in a FIPS 140-3 validated configuration.
- **Implies:** uses-cryptography

### ships-container-image

- **Question:** Does it publish a container image?
- **Definition:** Yes when it publishes an OCI container image.
- **Implies:** uses-container-images, publishes-releases

### deploys-to-kubernetes

- **Question:** Does it ship Kubernetes manifests or Helm charts?
- **Definition:** Yes when it ships Kubernetes manifests or Helm charts.
- **Implies:** uses-container-images

### uses-container-images

- **Question:** Does the repository use container images?
- **Definition:** Yes when the repository names a container image anywhere, such as in Dockerfiles, Compose files, dev container definitions, Kubernetes manifests, Helm charts, CI jobs or test code (for example with Testcontainers).

### is-library

- **Question:** Do others use it as a library or SDK?
- **Definition:** Yes when other projects use it as a library or an SDK.

### is-developer-tool

- **Question:** Is it a tool that software developers use in their work?
- **Definition:** Yes when its main users are software developers, who use it while they write, build, test or ship code, such as a compiler, a linter, a build tool, a test framework or a command-line tool for a developer service.

### distributed-as-package

- **Question:** Is it released as an installable package or binary?
- **Definition:** Yes when it is released as a package or a binary that people install.
- **Implies:** publishes-releases

### publishes-releases

- **Question:** Does it publish versioned releases?
- **Definition:** Yes when the project publishes numbered versions of its software for others to install, run or build on, such as packages, binaries, container images, charts or tagged source releases. A service that only the project itself deploys does not count.

### has-dependencies

- **Question:** Does it use third-party code?
- **Definition:** Yes when it uses code that it does not maintain itself: a third-party package declared in a manifest (including a tool such as `typescript` in `devDependencies`), a library that a package manager installs outside a manifest for the software to use (such as `pip install` in a Dockerfile), a library from another repository (such as a git submodule), or vendored code. The language's standard library and runtime, and toolchains and CI components installed outside a manifest, do not count; the Supply chain area covers them.

### uses-private-packages

- **Question:** Does it use packages from a private registry or repository?
- **Definition:** Yes when any of its dependencies comes from a package registry or repository that is not public.
- **Implies:** has-dependencies

### vendors-dependencies

- **Question:** Does it keep copies of third-party code in its repository?
- **Definition:** Yes when its repository holds third-party source or binaries that it builds or ships.
- **Implies:** has-dependencies

### accepts-external-contributions

- **Question:** Can people outside the core team contribute?
- **Definition:** Yes when people outside the core team may send changes to it.

### has-multiple-maintainers

- **Question:** Can more than one person approve and merge changes?
- **Definition:** Yes when two or more people regularly review and merge changes into its default branch. A backup administrator who takes no part in reviews does not count, and neither do bots and automation accounts.

### uses-ai-models

- **Question:** Does it call or run AI models?
- **Definition:** Yes when it calls an AI model or runs one.

### exposes-mcp-server

- **Question:** Does it provide an MCP server?
- **Definition:** Yes when it provides a Model Context Protocol (MCP) server.
- **Implies:** has-ai-tool-use

### has-ai-tool-use

- **Question:** Can an AI model make it take actions?
- **Definition:** Yes when output from an AI model can make the software take an action, such as calling a tool, a function or an API, running a command or changing data, rather than only being shown to a person. This includes software that runs the model itself and software that offers tools to an outside agent, such as an MCP server.

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
- **Definition:** Yes when it is placed on the market of the European Union: made available as a product, such as installable software, firmware or a device, in the course of a commercial activity. A service that the project only runs for its users does not count.
- **Implies:** publishes-releases

### processes-eu-personal-data

- **Question:** Does it process personal data of people in the EU?
- **Definition:** Yes when it processes personal data of people who are in the European Union.
- **Implies:** handles-personal-data

### covered-by-eaa

- **Question:** Is it a product or service that the European Accessibility Act covers?
- **Definition:** Yes when it is, or is part of, a product or a service that the European Accessibility Act covers and that is offered to consumers in the European Union, such as an e-commerce service, a consumer banking service, an e-book or its dedicated reading software, an electronic communications service, a service that gives access to audiovisual media, the website or mobile application of an air, bus, rail or waterborne passenger transport service, or a consumer general-purpose computer (a desktop, a notebook, a smartphone or a tablet) or an operating system for one. A product placed on the market before 28 June 2025 does not count. A service from a microenterprise (fewer than 10 persons and an annual turnover or balance sheet total of at most EUR 2 million) does not count.

### has-plugin-interface

- **Enabled by:** USS-EXT-001
- **Definition:** Yes when the project adopts USS-EXT-001: the software offers an interface through which plugins add features.

### has-plugin-marketplace

- **Enabled by:** USS-EXT-008
- **Definition:** Yes when the project adopts USS-EXT-008: the project runs or ships a marketplace through which users find and install plugins.
