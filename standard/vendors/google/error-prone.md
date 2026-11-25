# Error Prone

- **Kind:** profile
- **Product:** Error Prone
- **Vendor:** Google
- **Maintainer:** community
- **Versions:** >=2.53.0, <3.0.0

Error Prone is a static analysis tool that runs inside the `javac` compiler and
reports bug patterns in Java code as compile errors and warnings. Each check has
a default severity (ERROR, WARNING or off), and a flag changes it. The Java
module recommends it in USS-JAVA-003. Put the baseline below in the
`maven-compiler-plugin` section of `pom.xml`. Error Prone runs on JDK 21 or
newer, and when the compiler does not fork, `.mvn/jvm.config` needs the lines
that follow the baseline, because the JDK encapsulates the compiler's
internals. The checks and flags that the profile names are those of Error Prone
2.53.0, from its documentation and bug-pattern pages (read 2026-11-24). Every
check that is on by default at ERROR is part of the baseline without a flag.
The baseline raises ten WARNING checks to ERROR: the ones that find ignored
errors (`EmptyCatch`, `CatchAndPrintStackTrace`, `FutureReturnValueIgnored`),
unused code (`UnusedVariable`, `UnusedMethod`, `UnusedNestedClass`), a missing
`@Override` (`MissingOverride`), a comparison of references where the values
should be compared (`ReferenceEquality`), an `enum` switch that misses a case
(`MissingCasesInEnumSwitch`) and a `@SuppressWarnings` with no explanation
(`SuppressWarningsWithoutExplanation`). The other WARNING checks stay warnings,
and they fail the build only when `javac` runs with `-Werror` (USS-JAVA-001).
A suppression is `@SuppressWarnings("CheckName")`, which names the check; the
annotation has no reason field, so the reason is a comment next to it. Error
Prone ends the compilation with an error for a finding at ERROR, so Maven's
compiler plugin fails and the build fails.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```xml
<!-- pom.xml, in <build><plugins>. Error Prone runs on JDK 21 or newer. -->
<plugin>
  <groupId>org.apache.maven.plugins</groupId>
  <artifactId>maven-compiler-plugin</artifactId>
  <version>3.16.0</version>
  <configuration>
    <annotationProcessorPaths>
      <path>
        <groupId>com.google.errorprone</groupId>
        <artifactId>error_prone_core</artifactId>
        <version>2.53.0</version>
      </path>
    </annotationProcessorPaths>
    <compilerArgs>
      <arg>-XDcompilePolicy=simple</arg>
      <arg>--should-stop=ifError=FLOW</arg>
      <!-- One -Xplugin:ErrorProne argument holds all Error Prone flags. -->
      <arg>-Xplugin:ErrorProne -Xep:EmptyCatch:ERROR -Xep:CatchAndPrintStackTrace:ERROR -Xep:FutureReturnValueIgnored:ERROR -Xep:UnusedVariable:ERROR -Xep:UnusedMethod:ERROR -Xep:UnusedNestedClass:ERROR -Xep:MissingOverride:ERROR -Xep:ReferenceEquality:ERROR -Xep:MissingCasesInEnumSwitch:ERROR -Xep:SuppressWarningsWithoutExplanation:ERROR</arg>
    </compilerArgs>
  </configuration>
</plugin>
```

The `.mvn/jvm.config` lines for a compiler that does not fork:

- `--add-exports jdk.compiler/com.sun.tools.javac.api=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.file=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.main=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.model=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.parser=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.processing=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.tree=ALL-UNNAMED`
- `--add-exports jdk.compiler/com.sun.tools.javac.util=ALL-UNNAMED`
- `--add-opens jdk.compiler/com.sun.tools.javac.code=ALL-UNNAMED`
- `--add-opens jdk.compiler/com.sun.tools.javac.comp=ALL-UNNAMED`

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-JAVA-002 | Maintainability | the baseline `-Xplugin:ErrorProne` with every check that is on by default at ERROR (such as `ReturnValueIgnored` and `CheckReturnValue`) and the WARNING checks that the baseline raises to ERROR | partial: CI must compile the main and the test code with these arguments, a WARNING check that the baseline does not raise fails the build only with `-Werror` (USS-JAVA-001), and Error Prone reads Java code only | Read the compiler arguments and the CI job; run the build and expect exit status 0. |
| USS-QLT-001 | Maintainability | the baseline `-Xplugin:ErrorProne` and its `-Xep:` flags | partial: CI must run the build on each pull request and each push to the default branch and fail on a finding, a WARNING check that the baseline does not raise fails only with `-Werror`, and Error Prone lints Java code only | Read the CI job; run the build and expect exit status 0. |
| USS-QLT-004 | Maintainability | `UnusedVariable` (private fields and parameters of private methods), `UnusedMethod` (private methods), `UnusedNestedClass` (unused nested classes), all at ERROR | partial: members that are not private and unreachable code are not reported (the `javac` flags of USS-JAVA-001 and a SpotBugs run cover other cases), and commented-out code is not covered | Run the build; search for commented-out code. |
| USS-QLT-005 | Maintainability | `EmptyCatch` (a `catch` block with no statement and no comment in it), `CatchAndPrintStackTrace` (an exception that is only printed), `FutureReturnValueIgnored` (a `Future` that nobody reads), all at ERROR | partial: a `catch` block that has statements and does not handle the error, and a return code that is dropped, are not reported, and the comment that `EmptyCatch` accepts is not read for its reason | Run the build; read each comment in an empty `catch` block; search for places that drop an error. |
| USS-QLT-006 | Maintainability | the baseline `annotationProcessorPaths` and `compilerArgs`, with no `-Xep:` flag that sets a baseline check to `WARN` or `OFF` | partial: the project's compiler arguments must enable every flag of the baseline at ERROR, and a check that the project turns off is judged by USS-QLT-007 | Compare the project's compiler arguments with the baseline above. |
| USS-QLT-009 | Maintainability | `SuppressWarningsWithoutExplanation` at ERROR (each `@SuppressWarnings` needs a comment that says why the warning is safe to ignore) | partial: the check requires a comment, not a reason, and does not read where it is, `@SuppressWarnings` names a `javac` or Error Prone check (the annotation has no reason field), and the suppressions of other tools are outside the check | Run the build; read each reason; search for `@SuppressWarnings`. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| A `@SuppressWarnings` with no comment that explains it is an error | `-Xep:SuppressWarningsWithoutExplanation:ERROR` | USS-QLT-009 |

Error Prone has no setting that reports a check that is turned off, a blanket
suppression or a suppression that suppresses nothing, so the auditor reads the
`-Xep:` flags and each `@SuppressWarnings` (USS-QLT-007, USS-QLT-010 and
USS-QLT-012).
