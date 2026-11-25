# SpotBugs with Find Security Bugs

- **Kind:** profile
- **Product:** SpotBugs with Find Security Bugs
- **Vendor:** SpotBugs and Find Security Bugs
- **Maintainer:** community
- **Versions:** spotbugs-maven-plugin >=4.10.5.0, <5.0.0;
  findsecbugs-plugin >=1.14.0, <2.0.0

SpotBugs reads the compiled classes of a Java project and reports bug patterns
in them, in categories such as correctness, bad practice, security and
performance. Find Security Bugs is a SpotBugs plugin that adds detectors for
security flaws: injection, unsafe deserialization, unsafe XML parsing, weak
cryptography and hard-coded secrets. The Java module recommends the pair in
USS-JAVA-004. Put the baseline below in the `build` section of `pom.xml`; the
`check` goal binds to the `verify` phase and runs the `spotbugs` goal first.
The plugin version 4.10.5.0 uses SpotBugs 4.10.5. The settings that the
profile names are those of spotbugs-maven-plugin 4.10.5.0 (its source and goal
pages) and of Find Security Bugs 1.14.0 (its bug-pattern list and metadata),
read 2026-11-24. `effort` takes `Min`, `Default` or `Max`, and `threshold`
takes `High`, `Default`, `Low`, `Ignore` or `Exp`; `Low` reports findings of low
priority and above. The `check` goal ends the build with an error when SpotBugs
reports a finding or an error, so Maven exits with a non-zero status; it
tolerates findings only when `maxAllowedViolations` is above 0, and it
counts only findings at or above `failThreshold` when that is set. The plugin
analyses the test classes only when `includeTests` is `true`, which the
baseline leaves at its default. A suppression is `@SuppressFBWarnings` with
the bug pattern in `value` and the reason in `justification`; the tool does
not require `justification`, and an annotation with no `value` suppresses every
warning in its scope without a report, so the auditor reads each annotation.
The baseline has no exclude filter, so no detector is switched off.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```xml
<!-- pom.xml, in <build><plugins>. Run with: mvn verify -->
<plugin>
  <groupId>com.github.spotbugs</groupId>
  <artifactId>spotbugs-maven-plugin</artifactId>
  <version>4.10.5.0</version>
  <configuration>
    <effort>Max</effort>
    <threshold>Low</threshold>
    <failOnError>true</failOnError>
    <maxAllowedViolations>0</maxAllowedViolations>
    <!-- No excludeFilterFile, omitVisitors or failThreshold: nothing is
         switched off and no finding is tolerated. -->
    <plugins>
      <plugin>
        <groupId>com.h3xstream.findsecbugs</groupId>
        <artifactId>findsecbugs-plugin</artifactId>
        <version>1.14.0</version>
      </plugin>
    </plugins>
  </configuration>
  <executions>
    <execution>
      <goals>
        <goal>check</goal>
      </goals>
    </execution>
  </executions>
</plugin>
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-JAVA-009 | Safety | `OBJECT_DESERIALIZATION` (use of object deserialization, such as `ObjectInputStream.readObject`) | partial: the pattern description does not mention a deserialization filter, so each finding is read for the filter, and a stream that the detector does not recognise is not reported | Run `mvn verify`; for each finding, read the filter of the stream. |
| USS-JAVA-010 | Safety | `XXE_DOCUMENT`, `XXE_SAXPARSER`, `XXE_XMLREADER`, `XXE_XMLSTREAMREADER`, `XXE_XPATH`, `XXE_DTD_TRANSFORM_FACTORY`, `XXE_SCHEMA_FACTORY`, `XXE_VALIDATOR` (an XML parser or processor that is open to XML external entities) | partial: the detectors judge the settings that they know for each API, whether or not the input is untrusted, and a parser API that they do not recognise is not reported | Run `mvn verify`; search for XML parsers that the detectors do not cover. |
| USS-SEC-004 | Safety | `HARD_CODE_PASSWORD`, `HARD_CODE_KEY` (a password or a cryptographic key in the code) | partial: only a password or a key that the code uses, not other tokens, configuration files or the history of the repository | Run `mvn verify`; run a secret scan of the tree and the history (USS-SEC-005). |
| USS-SEC-009 | Safety | `PATH_TRAVERSAL_IN`, `PATH_TRAVERSAL_OUT`, `UNVALIDATED_REDIRECT`, `XPATH_INJECTION`, `LDAP_INJECTION`, `TRUST_BOUNDARY_VIOLATION` (untrusted input that reaches a file path, a redirect, a query or a trusted store without validation) | partial: the detectors cover these sinks only, and they do not check that the input is validated against an allow-list of its type, length, range and format at each entry point | Run `mvn verify`; read each entry point that takes outside input. |
| USS-SEC-010 | Safety | `COMMAND_INJECTION` (an API that runs a system command with input that may be untrusted) | partial: the detector knows the command APIs of Java and its libraries, and the auditor reads each finding to see whether the input is untrusted | Run `mvn verify`; read each finding. |
| USS-SEC-011 | Safety | `OBJECT_DESERIALIZATION`, `JACKSON_UNSAFE_DESERIALIZATION` (an unsafe Jackson configuration), `XML_DECODER` (use of `XMLDecoder`) | partial: only the deserializers that the detectors know, and a data-only format or a type allow-list is read from the code | Run `mvn verify`; search for other deserializers that read untrusted data. |
| USS-SEC-013 | Safety | `SQL_INJECTION`, `SQL_INJECTION_JDBC`, `SQL_INJECTION_JPA`, `SQL_INJECTION_HIBERNATE`, `SQL_INJECTION_SPRING_JDBC` (a query built from input that is not a bound parameter) | partial: only the database APIs that the detectors know | Run `mvn verify`; search for other query-building code. |
| USS-SEC-014 | Safety | `XSS_SERVLET`, `XSS_JSP_PRINT` (output in a servlet or a JSP that is not escaped), `CRLF_INJECTION_LOGS` (input written to a log without neutralising line breaks), `HTTP_RESPONSE_SPLITTING` (input in an HTTP header) | partial: output into HTML from servlets and JSPs, log entries and HTTP headers only, so output into URLs, terminals and other formats is read from the code | Run `mvn verify`; search for output into other contexts. |
| USS-SEC-016 | Safety | `HARD_CODE_PASSWORD` (a password in the code) | partial: the detector reports a hard-coded password that the code uses, and not a default or shared token, key or password in configuration, images or installers | Run `mvn verify`; search configuration, images and installers. |
| USS-SEC-022 | Safety | `UNENCRYPTED_SOCKET`, `UNENCRYPTED_SERVER_SOCKET` (a plain socket), `INSECURE_SMTP_SSL` (an SMTP connection without server identity check), `SSL_CONTEXT` (a weak `SSLContext` protocol), `DEFAULT_HTTP_CLIENT` (a client that is not compatible with TLS 1.2) | partial: the detectors cover these socket, SMTP and HTTP client APIs only, so TLS versions, other protocols and settings are read from the code and the configuration | Run `mvn verify`; search the code and the configuration for other network connections. |
| USS-SEC-023 | Safety | `WEAK_TRUST_MANAGER` (a `TrustManager` that accepts any certificate), `WEAK_HOSTNAME_VERIFIER` (a `HostnameVerifier` that accepts any host) | partial: other ways to switch off validation, such as a setting or a flag, are not reported, and the user opt-out that the rule allows needs a read of the code | Run `mvn verify`; search for settings that disable validation. |
| USS-SEC-024 | Safety | `CUSTOM_MESSAGE_DIGEST` (a custom hash function implementation) | partial: only a custom `MessageDigest`, so other self-made ciphers, protocols and random number generators are read from the code | Run `mvn verify`; search for other cryptographic code. |
| USS-SEC-026 | Safety | `WEAK_MESSAGE_DIGEST_MD5`, `WEAK_MESSAGE_DIGEST_SHA1`, `DES_USAGE`, `TDES_USAGE`, `ECB_MODE` (MD2, MD4, MD5, SHA-1, DES, 3DES and the ECB mode), `RSA_KEY_SIZE`, `BLOWFISH_KEY_SIZE` (a short RSA or Blowfish key) | partial: the detectors report these algorithms for every use, SHA-1 included, check key sizes for RSA and Blowfish only, and do not cover the other algorithms and key sizes that NIST SP 800-131A or OWASP ASVS Appendix C disallow | Run `mvn verify`; check the other key sizes and algorithms in the code and the configuration. |
| USS-SEC-028 | Safety | the baseline plugin configuration with Find Security Bugs, and its bug patterns for injection, deserialization, XML parsing, cryptography and secrets | partial: CI must run `mvn verify` (or the `check` goal) on each pull request and each push to the default branch and fail on a finding, and the detectors cover Java code that the bug-pattern list covers, not the other languages of the project | Read the CI job and its triggers; run `mvn verify` and expect exit status 0. |
| USS-JAVA-002 | Maintainability | the baseline plugin configuration with Find Security Bugs (SpotBugs is a bug-pattern checker that reads compiled classes) | partial: CI must run the check on all of the project's code and fail on a finding, test classes are analysed only with `includeTests` set to `true`, and SpotBugs reads Java classes only | Read the CI job; run `mvn verify` and expect exit status 0. |
| USS-QLT-001 | Maintainability | the baseline plugin configuration (`effort` `Max`, `threshold` `Low`, `failOnError`, `maxAllowedViolations` 0) | partial: CI must run the check on each pull request and each push to the default branch, test classes are analysed only with `includeTests` set to `true`, and SpotBugs reads Java classes only | Read the CI job; run `mvn verify` and expect exit status 0. |
| USS-QLT-004 | Maintainability | `UPM_UNCALLED_PRIVATE_METHOD` (a private method that is never called), `UUF_UNUSED_FIELD`, `URF_UNREAD_FIELD` (an unused or unread field), `DLS_DEAD_LOCAL_STORE` (a value that is stored and never read) | partial: public and package-private members, unreachable code and unused types are not reported, and commented-out code is not covered | Run `mvn verify`; search for commented-out code. |
| USS-QLT-005 | Maintainability | `DE_MIGHT_IGNORE`, `DE_MIGHT_DROP` (a method that might ignore or drop an exception), `RV_RETURN_VALUE_IGNORED`, `RV_RETURN_VALUE_IGNORED_BAD_PRACTICE` (an ignored return value or exceptional return value) | partial: the detectors report an ignored exception whether or not a comment explains it, and a `catch` block that has statements and does not handle the error is not reported | Run `mvn verify`; read the comment at each place that ignores an error. |
| USS-QLT-006 | Maintainability | the baseline `effort`, `threshold`, `plugins`, `failOnError` and `maxAllowedViolations`, with no `excludeFilterFile` and no `omitVisitors` | partial: the project's configuration must set every option of the baseline at its value or stricter, and a detector that the project turns off is judged by USS-QLT-007 | Compare the project's SpotBugs configuration with the baseline above. |
| USS-QLT-012 | Maintainability | `US_USELESS_SUPPRESSION_ON_CLASS`, `US_USELESS_SUPPRESSION_ON_FIELD`, `US_USELESS_SUPPRESSION_ON_METHOD`, `US_USELESS_SUPPRESSION_ON_METHOD_PARAMETER`, `US_USELESS_SUPPRESSION_ON_PACKAGE` (a `@SuppressFBWarnings` that suppresses nothing, reported at normal priority, which `threshold` `Low` includes), failing the build through `check` | partial: test classes are analysed only with `includeTests` set to `true`, so a stale suppression in test code is not reported with the baseline | Confirm `threshold` and `check` in the plugin configuration; run `mvn verify` and expect exit status 0; read the suppressions in test code. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| A suppression that suppresses nothing is a finding | `US_USELESS_SUPPRESSION_ON_*`, reported at `threshold` `Low` | USS-QLT-012 |
| No finding is tolerated | `maxAllowedViolations` at 0 and no `failThreshold` | USS-QLT-001 |
| No detector is switched off | no `excludeFilterFile` and no `omitVisitors` | USS-QLT-006 |
