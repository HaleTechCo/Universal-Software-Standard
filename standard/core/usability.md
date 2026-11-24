# Usability

- **Prefix:** UX
- **Kind:** area

This area covers how people use the software through its user interface or its
command line, without confusion. Its rules count toward Accessibility &
Usability, except the rule on deceptive design, which counts toward Fairness
because it is about honesty toward users. Access for people with disabilities
belongs to the Accessibility (ACC) area, and the language and locale of the
interface to the Internationalization (I18N) area. The documentation of settings
belongs to the Documentation area (USS-DOC-003), and the log-level option to the
Observability area (USS-OBS-002). Confirmation of an action that a model starts
is in the AI features area (USS-AI-004); the confirmation in this area is for
actions that a user starts. USS-UX-001 and USS-UX-002 cite WCAG success criteria
that USS-ACC-001 also requires, and that overlap is accepted.

## All interfaces

### USS-UX-001: Say what went wrong and how to fix it

- **Level:** MUST
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Verify:** Collect every error message that users can see: the strings in
  the resource files and in the code that the UI shows, or the command line
  prints, when an operation fails or an input is refused. Read each one. It
  says what went wrong (in words that name the thing, such as the field, the
  file or the command, and not only a code), and it says what the user can do
  about it (a corrected input, a step to take, or where to find help). A
  message with only a code or a generic text (such as "An error occurred"),
  or one that gives no next step, fails.
- **References:** WCAG 2.2 (W3C Recommendation, 12 December 2024), 3.3.1 Error
  Identification and 3.3.3 Error Suggestion; Command Line Interface
  Guidelines (clig.dev, read 2026-11-24), Errors

Each error message that users see MUST say what went wrong and what they can
do about it.

**Rationale:** An error that says only that something failed leaves the user
to guess, and they guess wrong or give up. The two parts are what let a person
recover alone, without the support line or the source code.

### USS-UX-002: Confirm or undo destructive actions

- **Level:** MUST
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Verify:** List the actions that a user can start and that delete or
  overwrite user data (delete, empty, reset, overwrite on save or import,
  bulk change). For each, read the code and try it in a test instance. In a
  UI, the action first asks for confirmation (a dialog that names what will be
  lost), or an undo is available afterwards (an undo command, a recycle bin,
  or a restore from history) and brings the data back. For a command-line
  command, the command either asks for confirmation when it runs interactively,
  or does the action only when an explicit option such as `--force` or `--yes`
  is given. An action that destroys data at once, with no confirmation, no
  option and no undo, fails.
- **References:** WCAG 2.2 (W3C Recommendation, 12 December 2024), 3.3.4 Error
  Prevention (Legal, Financial, Data); Command Line Interface Guidelines
  (clig.dev, read 2026-11-24), Arguments and flags

An action that a user starts and that deletes or overwrites user data MUST ask
for confirmation first or be undoable; a command-line option such as `--force`
counts as confirmation.

**Rationale:** Users click and type by habit, and a mistake that cannot be
taken back costs their work. A confirmation or an undo turns a disaster into a
moment's delay.

### USS-UX-003: Start with zero configuration

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli or is-network-service
- **Category:** Accessibility & Usability
- **Verify:** Take the first task that the quick-start section of the user
  documentation describes (or the main purpose that the README states, when
  there is no quick start). In a clean environment with no configuration file,
  no project-specific environment variable and no flags, install the software as
  the documentation says and run that task. It starts and completes the task. A
  first-run step that asks for the software's own first credentials, as
  USS-SEC-016 accepts, passes. A step that stops for any other missing setting
  (a path, an address, a credential for another service) fails.
- **References:** none

The software SHOULD start and do its main job with default settings and no
configuration file, apart from setting its own first credentials on first run
(USS-SEC-016).

**Rationale:** Each setting that is required before the first run is a place
where a new user stops. Sensible defaults let people start at once and adjust
later; the settings that do exist are documented (USS-DOC-003).

### USS-UX-004: No deceptive design

- **Level:** MUST NOT
- **Applies when:** has-ui
- **Category:** Fairness
- **Verify:** Walk through each flow in the UI where the user gives consent,
  pays, signs up, changes a plan or cancels. Check for each of these: (1) a
  cost that the user first sees after the flow has started (every mandatory
  charge is shown at the first step that shows a price); (2) a choice to
  decline worded to shame or blame the user (such as "No, I don't want to save
  money"); (3) a consent or opt-in box that is ticked or on when the screen
  opens; (4) a cancellation that takes more steps than the sign-up took, or
  that needs a channel that the sign-up did not (such as a phone call when the
  sign-up was online). For other designs that the EDPB guidelines and the FTC
  report list as deceptive (such as disguised ads, or a button to decline that
  is hidden), check the same flows. One such pattern fails the rule.
- **References:** EDPB Guidelines 03/2022 on deceptive design patterns in
  social media platform interfaces (Version 2.0, adopted 14 February 2023); FTC
  staff report, Bringing Dark Patterns to Light (September 2022)

The UI MUST NOT use deceptive design, such as hidden costs, confirmshaming,
pre-ticked consent, or a cancellation that is harder than the sign-up.

**Rationale:** A deceptive pattern gets a result that the user would not have
chosen with a clear view, and it takes trust to do so. The EDPB guidelines are
written for social media interfaces and the FTC report for commerce, and both
give examples that apply to any UI. This rule counts toward Fairness because it
is about honesty toward users.

### USS-UX-005: Keep files where the platform expects them

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Verify:** Run the software on each platform that it supports, use it, and
  list the files that it creates for configuration, data, cache and state
  (compare the user's home and profile folders before and after). Each is in the
  location that the platform's conventions name. On Linux, that is the directory
  of the matching XDG variable or its default: `XDG_CONFIG_HOME` (`~/.config`),
  `XDG_DATA_HOME` (`~/.local/share`), `XDG_CACHE_HOME` (`~/.cache`) and
  `XDG_STATE_HOME` (`~/.local/state`), and the software honours the variable
  when it is set. On Windows, it is a Known Folder (such as the roaming or local
  application data folder) that the software gets from the system and does not
  build by hand. On macOS, it is the matching folder under `~/Library` (such as
  `Application Support`, `Caches` or `Preferences`). Software that runs as a
  system service (started by the platform's service manager, not by a user) uses
  the system locations instead, and the auditor also compares those: on Linux,
  the directories of the Filesystem Hierarchy Standard (such as `/etc` for
  configuration, `/var/lib` for data and `/var/cache` for cache); on Windows,
  the program data Known Folder; on macOS, the matching folder under `/Library`.
  A file that the software creates directly in the home folder or in another
  place that the conventions do not name fails.
- **References:** XDG Base Directory Specification 0.8 (8 May 2021); Windows
  Known Folders (Microsoft Learn, read 2026-11-24); Apple, File System
  Programming Guide, File System Basics (updated 2018-04-09); Filesystem
  Hierarchy Standard 3.0 (freedesktop.org, read 2026-11-24), sections 3.7, 5.5
  and 5.8

Configuration, data, cache and state files SHOULD go in the locations that the
platform's conventions name (such as the XDG Base Directory Specification on
Linux).

**Rationale:** Users and tools know where to look to back up settings, clear a
cache or move a profile. A file in the wrong place is lost in a move, and a
cache among the data is backed up for nothing.

## Command-line interfaces

### USS-UX-006: Follow the Command Line Interface Guidelines

- **Level:** SHOULD
- **Applies when:** has-cli
- **Category:** Accessibility & Usability
- **Verify:** Read the Command Line Interface Guidelines and the project's
  commands, and, for each guideline that is about a feature that the command
  line has (help, output, errors, arguments and flags, interactivity,
  subcommands, robustness, configuration, environment variables, naming), check
  the command line against it by running it. A command line that contradicts a
  guideline about a feature that it has fails. Guidelines about features that
  it does not have (such as subcommands in a single-command tool) are not
  checked. The guidelines that a rule of this area already holds (USS-UX-007
  to USS-UX-011) are decided by those rules.
- **References:** Command Line Interface Guidelines (clig.dev, read
  2026-11-24)

The command-line interface SHOULD follow the Command Line Interface
Guidelines.

**Rationale:** People carry their habits from one command line tool to the next.
A tool that follows the common guidelines is learnt in minutes, and works with
scripts and pipes the way that they expect.

### USS-UX-007: Provide --help and --version

- **Level:** MUST
- **Applies when:** has-cli
- **Category:** Accessibility & Usability
- **Verify:** Run the command with `--help`, then with `--version`. Each exits
  with code 0 and prints non-empty text: the first prints usage, and the second
  prints a version. A command that rejects either option, or fails to print,
  fails.
- **References:** Command Line Interface Guidelines (clig.dev, read
  2026-11-24), Help and Arguments and flags

The command-line interface MUST support `--help` and `--version`.

**Rationale:** These two options are the first things that a person tries.
Help tells them what the tool does, and the version tells them, and the people
who help them, which tool they have.

### USS-UX-008: Use meaningful exit codes

- **Level:** MUST
- **Applies when:** has-cli
- **Category:** Accessibility & Usability
- **Verify:** The documentation lists the exit codes: 0 for success, and each
  non-zero code with its meaning (or one non-zero code for all failures, with a
  statement to that effect). Run a successful command and trigger at least one
  failure of each kind that the documentation lists: success exits with 0, and
  each failure exits with the documented non-zero code. A failure that exits
  with 0, or a code that the documentation does not list, fails.
- **References:** Command Line Interface Guidelines (clig.dev, read
  2026-11-24), The Basics

Commands MUST exit with 0 on success and with a non-zero, documented code on
failure.

**Rationale:** Scripts, schedulers and CI decide what to do next from the exit
code. A failure that returns 0 passes silently into the next step.

### USS-UX-009: Offer machine-readable output

- **Level:** SHOULD
- **Applies when:** has-cli
- **Category:** Accessibility & Usability
- **Verify:** List the commands that print data (a list, a record, a report),
  as against only a status line. Each has an option (such as `--json` or
  `--format json`) that prints the data in a machine-readable format that a
  standard parser reads (such as JSON, or CSV). Run each with the option and
  parse the output with a parser for that format: it parses with no error. A
  command that prints data and has no such option fails.
- **References:** Command Line Interface Guidelines (clig.dev, read
  2026-11-24), Output

Commands that print data SHOULD be able to print it in a machine-readable
format, such as JSON.

**Rationale:** Text made for people changes when the wording or the layout
improves, and scripts that parse it break. A machine format is a promise that
scripts can rely on.

### USS-UX-010: Honour NO_COLOR

- **Level:** MUST
- **Applies when:** has-cli
- **Category:** Accessibility & Usability
- **Verify:** Run each command that prints, in a pseudo-terminal (so that the
  software would use colour by default), first with `NO_COLOR` unset and then
  with `NO_COLOR=1`. With it set, the output has no ANSI escape sequences that
  set colour (for example, the sequences that set the foreground or
  background colour). Colour in the output when `NO_COLOR` is set to a value
  that is not empty fails. When it is unset or empty, colour is permitted.
- **References:** NO_COLOR (no-color.org, read 2026-11-24)

The interface MUST print no ANSI colour when the `NO_COLOR` environment
variable is set to a value that is not empty.

**Rationale:** Colour does not help every user: some cannot tell the colours
apart, and some terminals and logs show the escape codes as clutter. One
variable lets a person turn colour off for all their tools together.

### USS-UX-011: Keep output and messages apart

- **Level:** MUST
- **Applies when:** has-cli
- **Category:** Accessibility & Usability
- **Verify:** Run each command in a success case and a failure case, with
  standard output and standard error sent to separate files. The result of the
  command (the data that it exists to print) is in standard output, and
  the messages (progress, warnings, errors and logs) are in standard error. A
  command whose error text appears in standard output, or whose standard output
  holds progress or log text mixed with its result, fails.
- **References:** Command Line Interface Guidelines (clig.dev, read
  2026-11-24), The Basics

Commands MUST write their output to standard output, and their messages and
errors to standard error.

**Rationale:** A pipe or a redirect carries only standard output. If messages
are mixed into it, the next program reads them as data, and if errors go to
it, they vanish into a file.
