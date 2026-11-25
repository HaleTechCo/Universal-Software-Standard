# NullAway

- **Kind:** profile
- **Product:** NullAway
- **Vendor:** Uber
- **Maintainer:** community
- **Versions:** >=0.14.4, <0.15.0

NullAway is a nullness checker that runs as an Error Prone check. It reads the
nullness annotations of the code and reports a dereference of a value that can
be null, a `null` that reaches a place that forbids it, and a field that a
constructor leaves unset. The Java module recommends it in USS-JAVA-005. Put
the baseline below in the `maven-compiler-plugin` section of `pom.xml`, and add
its flags to the one `-Xplugin:ErrorProne` argument that the Error Prone
profile (`standard/vendors/google/error-prone.md`) also uses; a project that
does not use that profile keeps only the NullAway flags in the argument. The
baseline checks `@NullMarked` code in JSpecify mode and reports at ERROR (the
default severity of NullAway is a warning). With `OnlyNullMarked`, NullAway
checks only code that `@NullMarked` covers, and the Java module asks for that
annotation on the project's own code. JSpecify mode needs a `javac` that reads
type-use annotations from class files: JDK 22 or newer, or an OpenJDK build of
JDK 21.0.8 or later (or 17.0.19 or later) with the compiler argument
`-XDaddTypeAnnotationsToSymbol=true`, which the baseline sets. NullAway fails
the build when it does not get such a `javac`. The
NullAway documentation describes JSpecify mode as still under development, so
it can report a false positive, and an upgrade within the range can bring new
findings. NullAway needs JDK 17 or newer and Error Prone 2.36.0 or newer. The
flags and options that the profile names are those of NullAway 0.14.4, from its
README and wiki (read 2026-11-24).

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```xml
<!-- pom.xml, in <build><plugins>; add the dependency to <dependencies>. -->
<dependency>
  <groupId>org.jspecify</groupId>
  <artifactId>jspecify</artifactId>
  <version>1.0.1</version>
</dependency>

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
      <path>
        <groupId>com.uber.nullaway</groupId>
        <artifactId>nullaway</artifactId>
        <version>0.14.4</version>
      </path>
    </annotationProcessorPaths>
    <compilerArgs>
      <arg>-XDcompilePolicy=simple</arg>
      <arg>--should-stop=ifError=FLOW</arg>
      <!-- Needed on JDK 21 only; JDK 22 and newer read type-use annotations. -->
      <arg>-XDaddTypeAnnotationsToSymbol=true</arg>
      <arg>-Xplugin:ErrorProne -Xep:NullAway:ERROR -XepOpt:NullAway:OnlyNullMarked=true -XepOpt:NullAway:JSpecifyMode=true</arg>
    </compilerArgs>
  </configuration>
</plugin>
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-SAF-001 | Safety | `-Xep:NullAway:ERROR` with `OnlyNullMarked=true` and `JSpecifyMode=true` (a dereference of a nullable value, in `@NullMarked` code) | partial: NullAway checks nullness only, and only in `@NullMarked` code, so the other safety options that the Java module names (the `javac` options of USS-JAVA-001) are judged from the compiler arguments, and `@NullMarked` must cover the project's own code | Read the compiler arguments; run the build and expect exit status 0; check that `@NullMarked` covers each module, package or class of the project. |

## Check-integrity guards

NullAway has no check-integrity settings of its own: it has no setting that
reports a check that is turned off, a blanket suppression or a suppression that
suppresses nothing. A suppression is `@SuppressWarnings("NullAway")`, and the
auditor reads each one (USS-QLT-009, USS-QLT-010 and USS-QLT-012).
