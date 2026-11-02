---
url: "https://github.com/horsicq/Detect-It-Easy"
title: "GitHub - horsicq/Detect-It-Easy: Program for determining types of files for Windows, Linux and MacOS."
site: "GitHub"
captured: "2026-11-02T19:03:30Z"
via: "live"
sha256: "45641bb8e7e736616ccea0e2ceb7781d634e5546047b5b6a02efc2b1cdfe06d1"
wayback: "https://web.archive.org/web/20261014235723/https://github.com/horsicq/detect-it-easy"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
---

# GitHub - horsicq/Detect-It-Easy: Program for determining types of files for Windows, Linux and MacOS.

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://github.com/horsicq/Detect-It-Easy>
> Snapshot: <https://web.archive.org/web/20261014235723/https://github.com/horsicq/detect-it-easy>

https://github.com/horsicq/Detect-It-Easy/blob/master/docs/logo_text.png

https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=NF3FBD3KHMXDN http://ntinfo.biz http://ntinfo.biz https://github.com/horsicq/XTranslation

Detect It Easy (DiE) is a cross-platform tool for file identification and static inspection, used by malware analysts, cybersecurity experts, and reverse engineers. It combines native format parsers with an extensible DiE-JS analysis layer, supporting both compact signatures and multi-stage static analysis across Windows, Linux, and MacOS.

## 🚀 Getting started

https://github.com/horsicq/Detect-It-Easy#-getting-started

- [💎 Download release/beta](https://github.com/horsicq/DIE-engine/releases)
- [🧪 DiE API Library (for Developers)](https://github.com/horsicq/die_library)
- [📋 Changelog](https://github.com/horsicq/Detect-It-Easy/blob/master/changelog.txt)
- [💬 Contribute to Translations](https://github.com/horsicq/XTranslation)

Warning

detectiteasy.com is not affiliated with the Detect It Easy project or its official website. Do not trust it as an official source or use it to download DiE. Use only the release links provided in this repository.

https://github.com/horsicq/Detect-It-Easy/blob/master/docs/1.png

## 💡 Why use Detect It Easy?

https://github.com/horsicq/Detect-It-Easy#-why-use-detect-it-easy

Detect It Easy combines format-aware parsing, signature matching, and programmable analysis for malware analysis, reverse engineering, and digital forensics. DiE-JS modules can use parsed structures and low-level inspection primitives to implement bounded algorithms while keeping detection policy reviewable in the database.

https://github.com/horsicq/Detect-It-Easy/blob/master/docs/2.png

### Key advantages:

https://github.com/horsicq/Detect-It-Easy#key-advantages

- Extensible Analysis Modules: DiE-JS supports compact signatures as well as stateful, format-aware algorithms.
- Cross-Platform Support: Runs on Windows, Linux, and MacOS.
- Conservative Heuristics: Independent evidence, architecture gates, bounded scans, and explicit antipatterns help control false positives.

## 🧠 Heuristic engine

https://github.com/horsicq/Detect-It-Easy#-heuristic-engine

### PE heuristic engine: analysis beyond a signature match

https://github.com/horsicq/Detect-It-Easy#pe-heuristic-engine-analysis-beyond-a-signature-match

A signature can tell you what a file resembles. The [Generic Heuristic Analysis engine (__GenericHeuristicAnalysis_By_DosX.7.sg)](https://github.com/horsicq/Detect-It-Easy/blob/master/db/PE/__GenericHeuristicAnalysis_By_DosX.7.sg) goes further: it reports concrete structural and behavioral anomalies together with the evidence behind them. The PE heuristic engine is created and maintained by [DosX](https://github.com/DosX-dev).

Note

DiE-JS is the implementation language of this analysis engine, not a shorthand for a few native detector calls. The native DiE core supplies format parsing, bounded reads, address translation, searching, and disassembly primitives. The PE heuristic engine builds analysis algorithms on those primitives: it selects candidates, maintains state, traverses bounded control flow, validates relationships across structures, rejects benign explanations, correlates independent evidence, and reconciles results.

Heuristic analysis is not merely a fallback used when signatures fail. When enabled, it runs as a separate higher-level analysis engine and can corroborate, qualify, or reject earlier database results while keeping every heuristic conclusion visibly marked.

With heuristic scanning enabled, the PE heuristic engine makes a series of specialized passes over native and managed images. The file is never launched. Instead, the engine works with headers, data directories, sections, imports, exports, resources, .NET metadata, bytecode, overlays, debug records, and reachable startup code rooted at the entry point. The same coverage extends to DLL initialization code, closing a common blind spot when protected or suspicious behavior begins inside a library rather than an application. This makes it useful both when an exact signature is known and when a sample has been modified enough to evade ordinary identification.

The PE heuristic engine's native-code analysis combines cached linear disassembly with bounded traversal of reachable startup code and purpose-built state machines. It tracks the register, flag, stack, address-provenance, and instruction-boundary facts required by each rule without pretending to be a sandbox or full CPU emulator. This allows DiE to expose opaque and degenerate branches, synthetic and indirect transfers, overlapping instruction streams, position-independent and self-modifying stubs, bitstream unpackers, anti-analysis probes, and irregular control flow used by polymorphic packers and protectors. The checks remain effective across register substitution, neutral padding, equivalent arithmetic forms, and bounded reordering of independent instructions commonly produced by commercial generators and private cryptors.

For managed code, the PE heuristic engine uses an internal MSIL opcode model to build operand-aware bytecode patterns for indirect calls and function pointers, control-flow transformations, arithmetic mutations, invalid instruction sequences, and other forms of obfuscation. It is not a CLR emulator, but it allows the engine to reason about executable IL patterns instead of treating a managed assembly as little more than metadata and strings.

Much of the analysis comes from joining evidence across layers. The engine can relate version-resource identity to Authenticode state, Rich build metadata, runtime model, and detected protection; cross-check the file-system extension; interpret mangled import and export symbols used by MSVC, GNU/MinGW, Borland, and Swift; and recognize fingerprints left by dumpers, unpackers, and PE reconstructors. Native checks are architecture-aware across x86/x64 and the ARM family, while security-mitigation flags are checked as meaningful combinations rather than isolated bits.

A single scan may independently identify a known protector, recognize its characteristic section layout, describe entry-point transformations, recover toolchain or game-engine provenance, and associate licensing metadata. These findings come from separate analysis paths; no single backend predicate produces the complete report.

These are the main passes rather than a complete inventory of every check:

#### Code, protection, and evasion

https://github.com/horsicq/Detect-It-Easy#code-protection-and-evasion

(click) 🧩 .NET obfuscation

- Operand-aware MSIL patterns, modified managed entry points, odd CLR constructors and sections, indirect calls, control-flow tricks, integer confusion, encrypted strings, invalid opcodes, anti-tamper, fake metadata, watermarks, and virtualization-like code.

(click) 🕵️ .NET anti-analysis

- Static signs of anti-debugging and anti-dumping, plus checks aimed at dnSpy, ILSpy, SandBoxie, Cuckoo, Wine, VMs, and some security products.
- This gives the analyst fair warning when ordinary debugging, decompilation, or dumping may be deliberately obstructed.

(click) 🛡️ Native anti-analysis

- State-aware inspection of startup code exposes direct PEB and debug-register access, Trap Flag tricks, low-level system and virtual-environment probes, privileged instructions, direct syscalls, and other attempts to detect or disrupt analysis before the main program begins.
- The engine also recognizes manual, hash-based API resolution that avoids an ordinary import trail, including variants that do not depend on one fixed hash constant.

(click) 🧱 Native anomalies

- Architecture-aware inspection follows reachable control flow around the entry point and recognizes synthetic or indirect transfers, overlapping instructions, Call/Pop and FPU-based position recovery, self-modifying stubs, bitstream unpackers, opaque branches, and irregular polymorphic control flow.
- Validated x86/x64 stack pivots and concealed RET-based transfers are covered as part of the same startup-flow analysis.
- Section permissions, declared code size, TLS startup, and the actual placement of executable code are cross-checked with IAT/EAT, image flags, linker values, and other structural signals.

(click) 📦 Packers and protectors

- Packers, cryptors, SFX archives, RunPE-like behavior, compression and crypto clues, overlays, high entropy, embedded PE files, and damaged unpacking results.
- A large curated database covers positional import hashes, section and resource names, managed-object sets, and artifacts left by dumpers and reconstructors such as Scylla, ImpRec, and OllyDump.

(click) 🔑 Licensing / DRM

- .NET licensing APIs and attributes, license managers, activation strings, SteamStub, Denuvo markers, and similar traces.
- This context can explain why a commercial application is packed or unusually structured without presenting license enforcement itself as malicious behavior.

#### Structure, provenance, and triage

https://github.com/horsicq/Detect-It-Easy#structure-provenance-and-triage

(click) 🩺 Format integrity

- Headers, entry points, alignment, relocations, IAT/EAT/resources, CLR metadata and version strings, and Authenticode placement are checked for damage or tampering.
- Deeper consistency checks cover overlapping sections and data-directory mappings, AMD64 unwind records, Guard CF metadata, and chained WIN_CERTIFICATE entries.
- Instead of merely calling a file broken, DiE points to the affected structures—useful when deciding whether a sample is original, damaged, dumped from memory, or only partially reconstructed. The Corrupted data verdict also states the impact: Uncritical covers damage limited to optional metadata, Degraded marks structural inconsistencies that may leave the image operational, and Critical identifies damage to structures required for reliable loading or execution.

(click) 🧾 Debug leftovers

- Debug sections, exported symbols, .NET Native data, absolute, portable, or embedded PDB records, and Costura.Fody artifacts.
- These leftovers can expose build paths and project names, clarify how the image was produced, and give reverse engineers a useful starting point even in an otherwise opaque release build.

(click) ☣️ Malware-related patterns

- Correlations between imports, strings, bytecode, opcodes, metadata, resources, payload markers, and PE structure.
- Identity checks connect version fields, signing state, build metadata, and protection results to expose masquerading, fake system files, and suspicious builds.

(click) 🛠️ Toolchain / platform

- Compiler, linker, and language inference from mangled symbols, Rich records, runtime libraries, section layout, and source-file residue, plus Windows-facing markers such as AppContainer and Game Definition File data.
- Together, these clues can recover useful build provenance even when no exact compiler signature survives.

(click) 🏷️ Filename anomalies

- PE-aware classification instead of a blanket warning for every unusual suffix.
- AutoCAD, Total Commander, 3ds Max, Microsoft Excel, Borland/Delphi, CPython, Node.js, MATLAB, and other ecosystems deliberately use PE modules with specialized extensions.
- DiE names known roles directly—for example, .bpl as Borland Package, .xll as Microsoft Excel Add-In, .arx as AutoCAD ObjectARX Module, or .wcx as Total Commander Packer Plug-In—while missing, custom, or misleading extensions, application images carrying a .dll suffix, and DLL images presented as .exe files are reported separately.

This is not a black-box malware score. DiE produces a practical evidence map for triage: what probably built the file, whether the image looks original, damaged, dumped, or reconstructed, which protection or DRM may explain its structure, and which artifacts deserve attention next. A heuristic result is a lead, not automatic proof of malicious intent. Verbose scan messages keep general, managed-code, and instruction-derived evidence visibly separated as [HEUR/Any], [HEUR/.NET], and [HEUR/EMU]. Use --heuristicscan together with --verbose to see the fullest report.

### Malware clues without pretending to be an antivirus

https://github.com/horsicq/Detect-It-Easy#malware-clues-without-pretending-to-be-an-antivirus

DiE is not an antivirus, and the PE heuristic engine is not designed to declare a file safe. It provides no real-time protection or disinfection, and a clean report only means that the enabled rules did not find the static evidence they know how to recognize. Reputation services, an antivirus, dynamic analysis, and manual reverse engineering still answer different questions.

What DiE can do is expose a surprisingly broad range of threat-related clues while the file is still on disk. The examples below are only a small part of the actual rule set, not a complete catalogue. The heuristic engine evolves faster than this README and new families, markers, and cross-checks are added regularly.

- Remote-access trojans and backdoors, including generic RAT patterns and families such as NjRAT, AsyncRAT, NanoCore, Orcus, Gh0st RAT, DarkComet, NetWire, Remcos, BitRAT, and many others.
- Stealers and spyware, including generic stealer scoring, Mars Stealer, Echelon Stealer, StormKitty, MAX Spyware, and other family or behavior indicators.
- Ransomware, lockers, and destructive malware, with checks for WannaCry, UX-Locker, Liberium WinLocker, Amp WinLocker, Olympic Destroyer, and related threats.
- File-infecting viruses. Dedicated static infection checks currently cover Ramnit, Neshta, Slugin, Win9X.CIH, Win9X.Dupator, Parite, and Polip, with the list continuing to grow.
- Targeted and unusual threats, including markers associated with Slingshot APT, Equation Group tooling, RAT injectors, maliciously generated assemblies, fake or infected system files, and similar cases.
- Hidden payloads and delivery techniques, from Base64-encoded executables and RunPE-like behavior to PE files concealed in resources, sections, or overlays. A built-in known-plaintext attack (KPA) pass derives repeating XOR/XNOR, ADD/SUB, or reverse-subtraction keys up to 20 bytes from invariant PE-header fields, then validates the decoded image structurally instead of trusting a plain MZ match. Other payload and build anomalies are covered as well.

These are explainable static detections built from entry-point code, import fingerprints, metadata, strings, resources, section structure, and other relationships inside the image. They are valuable leads, but they are not a promise of complete malware-family coverage: modified samples may evade a rule, and an unusual clean program may share part of a suspicious pattern.

### Heuristic results stay visible

https://github.com/horsicq/Detect-It-Easy#heuristic-results-stay-visible

Heuristic findings are never silently mixed into ordinary signature matches. Every result produced by the heuristic layer is kept separate and marked with (Heur), so it is always clear which conclusion came from an exact rule and which one was inferred from a combination of evidence.

The engine does more than append extra lines. In a few deliberate cases it can reject a misleading result from the main signature database and replace it with a more precise heuristic conclusion. Programming-language detection is a typical example: a compiler signature may suggest C or C++, while stronger structural and runtime evidence identifies Rust. The replacement still carries (Heur) and never passes itself off as an exact signature match. The same reconciliation mechanism is used to suppress known fake packer, protector, and dongle signatures left behind by obfuscators.

A protected .NET sample may produce a report like this:

```
PE32
    Operation system: Windows (95) [I386, 32-bit, GUI]
    Linker: Microsoft Linker (6.0)
    Compiler: VB.NET
    Language: VB.NET
    Library: .NET Framework (CLR 2.0.50727)
    Protector: .NET Reactor (4.8-4.9) [Anti-ILDASM]
    (Heur) Cryptor: Generic [Assembly invoke + RSACryptoServiceProvider + RunPE + Section #2 (".rsrc") compressed + Section #2 (".rsrc") has wrong size + High entropy]
    (Heur) Protection: Obfuscation [Modified managed EP + Anti-ILDASM + Bad namings + Fake .cctor name + Math mutations]

```

For a native executable, signatures and heuristic evidence can complement each other while remaining visibly distinct:

```
PE32
    Operation system: Windows (95) [I386, 32-bit, GUI]
    Linker: Microsoft Linker (6.0)
    Compiler: Microsoft Visual C/C++ (12.00.8168) [C++/std]
    (Heur) Language: Rust
    Protector: HyperTech Crackproof
    (Heur) Protection: Generic [Strange sections + Stack-push address near EP + Rdtsc near EP + Section #4 ("naN") has RWX + EP-section #4 ("naN") zero padding + IAT directory empty]
    (Heur) Packer: Generic [EntryPoint + Pushal at EP + Last section EP + Imports like UPX (v0.59-0.93) + Sections like fake UPX + Section #0 ("") compressed + High entropy]
    (Heur) Debug data: Contains [Embedded PDB (release)]

```

### Smaller heuristics for everyday files

https://github.com/horsicq/Detect-It-Easy#smaller-heuristics-for-everyday-files

The PE heuristic engine is accompanied by smaller heuristic modules for other everyday file types:

- The [JavaScript heuristic](https://github.com/horsicq/Detect-It-Easy/blob/master/db/Binary/__MiniJavaScriptHeuristic_By_DosX.7.sg) recognizes common JavaScript variants, distinguishes text from bytecode, and spots minified or compiled-looking code without blindly matching content inside ordinary strings.
- The [file-extension heuristic](https://github.com/horsicq/Detect-It-Easy/blob/master/db/Binary/__MiniExtensionsHeuristic_By_DosX.7.sg) provides a broad fallback catalogue of formats and programming languages, cross-checking the extension against whether the file is actually textual or binary.
- The [Batch-script heuristic](https://github.com/horsicq/Detect-It-Easy/blob/master/db/Binary/__MiniBatchHeuristic_By_DosX.7.sg) catches UTF-16LE obfuscation and non-textual content hidden inside BAT and CMD files.

## 🧰 Several engines under one GUI

https://github.com/horsicq/Detect-It-Easy#-several-engines-under-one-gui

The desktop version of DiE is not limited to its own scanning engine. It brings several independent analyzers into the same interface, each with a different rule model and a different idea of what constitutes a useful match. On a difficult or unfamiliar file, running them in turn can expose details that one database alone would miss. Their output is complementary rather than a vote: three engines repeating a weak signature do not turn it into proof.

- Detect It Easy (DiE) is the primary, format-aware scanner. Its native core parses file formats and exposes bounded inspection primitives, while DiE-JS modules range from compact signatures to stateful analysis engines. The PE heuristic engine operates at this higher level, combining executable structures, metadata, imports, sections, entry-point code, antipatterns, and bounded byte searches. The signature database can also recover application context from recognizable .NET dependencies spanning data access, logging and background jobs, HTTP and browser automation, content tooling, cryptography, testing, and more.
- [Nauz File Detector](https://github.com/horsicq/Nauz-File-Detector) (NFD) provides an independent view of linkers, compilers, tools, and packers. It has no user-rule workflow comparable to DiE-JS or YARA, its heuristic logic is much simpler, and its database is updated relatively infrequently. That makes it useful as a second opinion, not as a replacement for the main engine.
- [YARA](https://github.com/VirusTotal/yara) adds direct rule-based matching with textual, binary, and logical conditions. It is a de facto standard for malware researchers and threat hunters, and DiE ships its own [basic](https://github.com/horsicq/Detect-It-Easy/blob/master/yara_rules/DiE_BasicHeuristics_by_DosX.yar) and [enhanced](https://github.com/horsicq/Detect-It-Easy/blob/master/yara_rules/DiE_EnhancedHeuristics_by_DosX.yar) YARA-side heuristics. These provide a lighter cross-check of suspicious PE traits rather than duplicating the full DiE heuristic engine.
- [PEiD](https://github.com/horsicq/XPEID) is included for compatibility with the classic "old-school" detector and its userdb ecosystem. The [bundled database](https://github.com/horsicq/Detect-It-Easy/blob/master/peid_rules/PE) preserves a large amount of historical material imported from the original PEiD rules. It remains useful for reproducing legacy detections, but many signatures are noisy by modern standards and can produce convincing-looking false positives, so its results should be treated as secondary evidence.

## 🧩 Anatomy of a minimal detection rule

https://github.com/horsicq/Detect-It-Easy#-anatomy-of-a-minimal-detection-rule

DiE database entries are DiE-JS modules ranging from compact signatures to stateful, multi-stage format analyzers. The example below intentionally demonstrates only the result contract of a minimal standalone rule; it is not representative of the PE heuristic engine.

A typical standalone rule declares the kind of result it produces, inspects the current file through the format API, fills optional result fields, and returns the engine-built result:

```
// Detect It Easy: detection rule file
// Author: Your Name <you@example.com>

// Optional reference URL
meta("compiler", "Example Compiler");

function detect() {
    if (PE.isSectionNamePresent(".lz-algo")) {
        sVersion = "1.0";
        sOptions = "LZMA";
        bDetected = true;
    }

    sLang = "C/C++";

    return result();
}
```

The result variables are supplied by the DiE engine and must not be redeclared:

| Field  | Purpose

| bDetected  | Marks the rule as matched. This is the field a normal rule must set when its conditions succeed.

| sName  | Overrides the default name from meta() when the exact variant is only known at runtime.

| sVersion  | Version, build, generation, or another short version-like value.

| sOptions  | Architecture, mode, modification state, or other useful qualifiers.

| sLang  | Adds a programming-language result. It is intentionally uncommon, used mostly by compiler rules, and conventionally assigned at the end of detect(), immediately before the blank line preceding return result();.

| sType  | Rare runtime override for the result category; reserve it for multi-purpose modules.

meta("type", "name") supplies the normal result category and default name. Optional shared helpers are loaded with includeScript("module") between meta() and detect(). Simple rules finish with return result();; keep one empty line immediately before it.

Before submitting a rule or script module, follow the complete [DiE-JS code-formatting standard](https://github.com/horsicq/Detect-It-Easy/blob/master/CODE-FORMATTING.md).

### Before writing a real rule

https://github.com/horsicq/Detect-It-Easy#before-writing-a-real-rule

The skeleton above demonstrates syntax and result construction only. DiE determines the file class first and then runs the rules from the matching directory, so format-specific logic belongs beside that format: PE rules use db/PE, ELF rules use db/ELF, and so on. db/Binary is intended for unclassified or genuinely format-independent data, not as a shortcut for code that belongs to a more specific parser.

The scripting API is documented in help. Start with the [global functions](https://github.com/horsicq/Detect-It-Easy/blob/master/help/Global.md), the common [Binary API](https://github.com/horsicq/Detect-It-Easy/blob/master/help/Binary.md), and the [signature-pattern reference](https://github.com/horsicq/Detect-It-Easy/blob/master/help/Signatures.md), then use the class reference for the format being inspected: [PE](https://github.com/horsicq/Detect-It-Easy/blob/master/help/PE.md), [.NET metadata](https://github.com/horsicq/Detect-It-Easy/blob/master/help/DOTNET.md), [ELF](https://github.com/horsicq/Detect-It-Easy/blob/master/help/ELF.md), [Mach-O](https://github.com/horsicq/Detect-It-Easy/blob/master/help/MACH.md), or another document from the same directory.

DiE byte signatures are a small pattern language embedded inside JavaScript strings. Hex bytes match literally, .. and ?? are byte wildcards, text is written inside single quotes, and $/# forms describe relative or address-dependent values. They are not regular expressions. The complete syntax and examples live in the [signature reference](https://github.com/horsicq/Detect-It-Easy/blob/master/help/Signatures.md).

Rules can inspect the scan mode with isHeuristicScan(), isDeepScan(), isAggressiveScan(), and isVerbose(). These calls reflect options selected by the user; they do not make an unnecessarily expensive rule acceptable. Even optional code must use cheap structural gates and antipatterns before bounded signature searches.

Repository placement also carries a quality meaning. db is the reviewed main database, with extra or unapproved rules tagged with the extra tag. dbs_min is generated output and must not be edited by hand.

Finally, a rule that works on one private sample is not yet maintainable coverage. The [contribution requirements](https://github.com/horsicq/Detect-It-Easy/blob/master/CONTRIBUTING.md#new-detection-rule-pull-requests) require reproducible samples, expected DiE output, independently checkable sources, and relevant false-positive material. The same document explains the [performance requirements](https://github.com/horsicq/Detect-It-Easy/blob/master/CONTRIBUTING.md#detection-rule-performance-requirements) that are enforced during review.

## 📄 Supported file types

https://github.com/horsicq/Detect-It-Easy#-supported-file-types

Detect It Easy supports a wide range of executable and archive types, including:

- PE (Portable Executable format for Windows)
- ELF (Executable and Linkable Format for Linux)
- APK (Android Application Package)
- IPA (iOS Application Package)
- JAR (Java Archive)
- ZIP (Compressed archives)
- ISO9660 (Optical media format)
- DEX (Dalvik Executable for Android)
- MS-DOS (MS-DOS executable files)
- COM (Simple executable format for DOS)
- LE/LX (Linear Executable for OS/2)
- MACH (Mach-O files for MacOS)
- NPM (JavaScript packages)
- Amiga (Executable format for Amiga computers)
- Binary (Other unclassified files)

And that's not all... The list is expanding as the tool is updated

Unknown formats undergo heuristic analysis, providing identification for both known and unrecognized files.

## 🔑 Key features

https://github.com/horsicq/Detect-It-Easy#-key-features

- Native Format Core: Parse executable structures and expose bounded reading, addressing, searching, and disassembly primitives.
- Extensible Analysis Engines: Use the DiE-JS ES5 runtime for compact signatures or stateful, multi-stage detection algorithms.
- Explainable Heuristics: Keep inferred results separate and report the structural, metadata, or instruction-derived evidence behind them.
- Cross-Platform Compatibility: Available for Windows, Linux, and MacOS.
- False-Positive Controls: Combine independent evidence with architecture gates, bounded scans, and explicit benign-case exclusions.

## 📥 Installation

https://github.com/horsicq/Detect-It-Easy#-installation

### 📦 Install via package managers

https://github.com/horsicq/Detect-It-Easy#-install-via-package-managers

-

Windows:

- [Chocolatey](https://community.chocolatey.org/packages/die)
- [Microsoft Store](https://apps.microsoft.com/detail/9nq58d7ghb2x)

-

Linux:

- Parrot OS: Package name detect-it-easy
- Arch Linux: AUR package [detect-it-easy-git](https://aur.archlinux.org/packages/detect-it-easy-git/)
- openSUSE: [OBS](https://build.opensuse.org/package/show/home:mnhauke/detect-it-easy)
- REMnux: Malware analysis distribution

https://repology.org/project/detect-it-easy/versions

Note

Use Detect It Easy bot via Telegram to quickly check files: [@detectiteasy_bot](https://t.me/detectiteasy_bot)

### ⚙️ Build from source

https://github.com/horsicq/Detect-It-Easy#️-build-from-source

See the [BUILD.md](https://github.com/horsicq/Detect-It-Easy/blob/master/docs/BUILD.md) for detailed instructions.

### 🐳 Docker installation

https://github.com/horsicq/Detect-It-Easy#-docker-installation

Run DiE in a Docker container:

```
git clone --recursive https://github.com/horsicq/Detect-It-Easy
cd Detect-It-Easy/
docker build . -t horsicq:diec
```

## 🖥️ Usage

https://github.com/horsicq/Detect-It-Easy#️-usage

Detect It Easy offers three versions:

- die - Graphical interface.
- diec - Command-line version for batch processing.
- diel - Lightweight GUI version. (scanner only)

For detailed usage, refer to the [RUN.md](https://github.com/horsicq/Detect-It-Easy/blob/master/docs/RUN.md).

### 🔎 Example use cases

https://github.com/horsicq/Detect-It-Easy#-example-use-cases

- 🦠 Malware Analysis: Identify file types, packers, or protections. Heuristic engine detects multiple malware and file virus families.
- 🛡 Security Audits: Determine executable potential security risks.
- 🔎 Software Forensics: Inspect software components and validate compliance.

## 💬 Our community

https://github.com/horsicq/Detect-It-Easy#-our-community

👋 Hello! / Привет! Welcome to the Detect It Easy community!

Have questions, ideas, or just want to chat? Here's where to find us:

- GitHub Discussions: Start a conversation in [Discussions](https://github.com/horsicq/Detect-It-Easy/discussions)
- GitHub Issues: Report bugs or request features via [Issues](https://github.com/horsicq/Detect-It-Easy/issues)

## 🏆 Special thanks

https://github.com/horsicq/Detect-It-Easy#-special-thanks

-

⭐️ Thanks to [DosX](https://github.com/DosX-dev)

-

⭐️ Thanks to [PELock](https://www.pelock.com)

## 🤝 Thanks to all contributors

https://github.com/horsicq/Detect-It-Easy#-thanks-to-all-contributors

https://github.com/horsicq/Detect-It-Easy/graphs/contributors

https://github.com/horsicq/Detect-It-Easy/blob/master/docs/logo2.png
