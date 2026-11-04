---
url: "https://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf"
title: "A Look In the Mirror: Attacks on Package Managers"
captured: "2026-11-02T19:03:48Z"
via: "live"
sha256: "5280b8061da7cb3057cd7168e786b0ee86e974425397593851f97ac45375f97a"
wayback: "https://web.archive.org/web/20190324215414/http://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# A Look In the Mirror: Attacks on Package Managers

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf>
> Snapshot: <https://web.archive.org/web/20190324215414/http://www2.cs.arizona.edu/stork/packagemanagersecurity/ccs2008.pdf>

          A Look In the Mirror: Attacks on Package Managers

                          Justin Cappos                   Justin Samuel              Scott Baker       John H. Hartman
                                            Department of Computer Science, University of Arizona
                                                          Tucson, AZ 85721, U.S.A.
                                             {justin, jsamuel, bakers, jhh}@cs.arizona.edu



ABSTRACT                                                                             systems [1, 2, 3, 21, 22, 25, 26, 28, 31, 32]. Package managers
This work studies the security of ten popular package man-                           provide a privileged, central mechanism for the management
agers. These package managers use different security mech-                           of software on a computer system. As packages are installed
anisms that provide varying levels of usability and resilience                       by the superuser (root), their security is essential to the
to attack. We find that, despite their existing security mech-                       overall security of the computer.
anisms, all of these package managers have vulnerabilities                              This paper evaluates the security of the eight most popu-
that can be exploited by a man-in-the-middle or a malicious                          lar [9, 19] package managers in use on Linux: APT [1], APT-
mirror. While all current package managers suffer from vul-                          RPM [2], Pacman [3], Portage [21], Slaktool [25], urpmi [28],
nerabilities, their security is also positively or negatively im-                    YaST [31], and YUM [32]. Also examined is the popular
pacted by the distribution’s security practices. Weaknesses                          package manager for BSD systems called ports [22] and a
in package managers are more easily exploited when distri-                           popular package manager in the research community called
butions use third-party mirrors as official mirrors. We were                         Stork [26]. These package managers use one of four differ-
successful in using false credentials to obtain an official mir-                     ent security models: no security, cryptographic signatures
ror on all five of the distributions we attempted. We also                           embedded within packages, signatures on detached package
found that some security mechanisms that control where a                             metadata, or signatures on the root metadata (a file that
client obtains metadata and packages from may actually de-                           contains the secure hashes of the package metadata).
crease security. We analyze current package managers to                                 This work demonstrates that there is an ordering to the
show that by exploiting vulnerabilities, an attacker with a                          amount of security provided by the different package man-
mirror can compromise or crash hundreds to thousands of                              ager security models. This order is preserved even as other
clients weekly. The problems we disclose are now being cor-                          security weaknesses in the package managers are corrected.
rected by many different package manager maintainers.                                Having no signatures allows the most egregious attacks, fol-
                                                                                     lowed by having only package signatures, having signatures
                                                                                     on detached package metadata, and finally having signa-
Categories and Subject Descriptors                                                   tures on the root metadata, which provides the most secu-
K.6.5 [Security and Protection]: Invasive software; C.2.0                            rity. However, there are usability concerns with different
[Computer-Communication Networks]: General—Se-                                       signature mechanisms, most notably the ability to verify a
curity and protection; K.4.1 [Social Issues]: Abuse and                              stand-alone package (a package obtained from a source other
Crime Involving Computers                                                            than the main repository). Signatures on root metadata do
                                                                                     not provide a convenient way to verify stand-alone packages
General Terms                                                                        and so the user is likely to install such packages without us-
                                                                                     ing security checks. In contrast, package managers that use
Security
                                                                                     signatures on detached package metadata or signatures on
                                                                                     packages can verify stand-alone packages.
Keywords                                                                                Because of the usability strengths and weaknesses of dif-
Package Management, Mirrors, Replay Attack                                           ferent techniques for providing security, this work recom-
                                                                                     mends a layered approach created by combining two tech-
1.     INTRODUCTION                                                                  niques: signatures on the root metadata and either signa-
                                                                                     tures on packages or package metadata. This technique pro-
  Package managers are a popular way to distribute software
                                                                                     vides the security strengths and the usability strengths of
(bundled into archives called packages) for modern operating
                                                                                     both types of signatures with an overhead of between 2-5%.
                                                                                     The layered approach has been added to the Stork package
                                                                                     manager and is now in use by thousands of clients around
Permission to make digital or hard copies of all or part of this work for            the world.
personal or classroom use is granted without fee provided that copies are               While we find that vulnerabilities in package managers
not made or distributed for profit or commercial advantage and that copies           do exist, vulnerabilities are not always exploitable in the
bear this notice and the full citation on the first page. To copy otherwise, to      real world. We examine this by looking at the security of
republish, to post on servers or to redistribute to lists, requires prior specific
permission and/or a fee.
                                                                                     popular distributions. We find that it is trivial for an at-
CCS’08, October 27–31, 2008, Alexandria, Virginia, USA.                              tacker to control an official package mirror for a popular
Copyright 2008 ACM 978-1-59593-810-7/08/10 ...$5.00.
                                                                                           Root Metadata
distribution (such as Ubuntu, Debian, Fedora, CentOS, and                                                                       Package
openSUSE) and therefore to be in the position to launch at-                packagemetadata1          hash: c302df...f8          Metadata
tacks on clients. To mitigate this threat, many distributions              packagemetadata2          hash: 192ee0...31       packagemetadata1
use mechanisms to distribute requests to multiple mirrors                     Root Metadata signature (optional)

or provide certain information from a trusted source. We
                                                                         A Package                   A Package’s Metadata
show that some of these mechanisms actually decrease the                 foo−2.0.rpm
security of users by making it easier to target attacks.                                              name:’foo−2.0.rpm’
                                                                    package contents                  size:’56242’
                                                                                                      hash:’74b5a1...c3’
2.   BACKGROUND                                                     embedded package                  ...
                                                                    metadata
                                                                                                      Package Metadata
2.1 Package Formats                                                 Package
                                                                                                      signature (optional)
                                                                    signature (optional)
   Packages consist of an archive containing files and, in most
cases, additional embedded package metadata. For a given
package, the embedded package metadata contains informa-          Figure 1: Repository Layout. The root metadata,
tion about any other packages that must be installed in or-       package metadata, and packages may all optionally
der for it to operate (the dependencies), functionality the       have signatures depending on the support of the
package possesses (what the package provides), and various        package manager. Arrows point from a secure hash
other information about the package itself. The most popu-        to the file it references.
lar package format, RPM [23], has space for one signature.
Other popular package formats have no standard field for
                                                                    Figure 1 shows the layout of a typical repository. A pack-
signatures, although in some cases extensions exist to sup-
                                                                  age manager downloads the root metadata and uses that
port signatures [8, 11].
                                                                  to locate the files containing the package metadata. The
                                                                  package manager then downloads the package metadata.
2.2 Package Managers                                              The package metadata is used to determine package avail-
   Clients use a package manager to install packages on their     ability as well as for dependency resolution. Packages are
system. A package manager gathers information about pack-         then downloaded and installed. The root metadata, pack-
ages available on package repositories. Almost all pack-          age metadata, and packages may be signed depending on
age managers automatically download requested packages as         the security model of the package manager.
well as any additional packages that are needed to correctly
install the software. This process is called dependency res-      2.4 Mirror
olution. For example, a requested package foo may depend             It is common for a distribution to have more than one
on libc and bar. If libc is already installed, then libc is       server from which users can download packages and package
a dependency that has been resolved (no package must be           metadata. There is usually a main repository for a distribu-
installed to satisfy the dependency). If there is no installed    tion whose contents are copied by many separate mirrors.
package that provides bar, then bar is an unresolved depen-       A mirror typically contains exactly the same content as the
dency and a package that provides bar must be installed           main repository and is updated via rsync or a similar tool.
before foo may be installed. The package manager may be           A mirror differs from a main repository in that a mirror is
able to locate a package that provides bar on a repository.       not intended to have packages directly added to it or re-
   The packages that are chosen to fulfill dependencies may       moved from it by its administrators. Packages are added or
have unresolved dependencies of their own. Packages are           removed only on the main repository and the mirrors later
continually added to the list of packages to be installed until   obtain the changes when copying the main repository.
either the package manager cannot resolve a dependency               A mirror can be public (available for anyone to use) or
(and produces an error) or all dependencies are resolved.         private (restricted to a specific organization). A mirror may
                                                                  also be endorsed by a distribution for public use, typically
2.3 Repository                                                    when that the distribution is in contact with the mirror
   A package repository is usually an HTTP or FTP server          maintainers. This type of mirror is called an official mirror
from which clients can obtain packages and package meta-          (the terminology used outside of this document varies by
data. The package metadata is usually just a copy of the em-      the distribution). Official mirrors are by definition public
bedded package metadata for all packages on the repository.       because the distribution is endorsing their use to the public.
Package managers download the package metadata from a                It should be noted that some distributions do not use of-
repository so that they know which packages are available         ficial mirrors hosted by outside organizations. One example
from that repository. This also provides the package man-         is tiny distributions that can support all of their clients by
ager with dependency information needed to perform de-            a small number of repositories that the distribution directly
pendency resolution. To facilitate convenient downloading         controls. Another example is a distribution that requires
of package metadata, most repositories store all of the pack-     users to pay. These costs often are used to support a set of
age metadata in a small number of compressed files.               internally maintained mirrors for the distribution. Alterna-
   In addition to the package metadata, almost all reposito-      tively, a distribution may allow or require each organization
ries have a root metadata file. The name and location of the      using the distribution to set up their own private mirrors for
root metadata file varies for different repository formats, but   the organization’s own use.
the contents are similar. The root metadata provides the lo-         However, official mirrors hosted by outside organizations
cation and secure hashes of the files that contain the package    are the predominant mechanism for software distribution
metadata.                                                         with all but two popular distributions [9, 19] relying on offi-
cial mirrors. Official mirrors are essential for most distribu-            the attacker can exploit. The attacker can then com-
tions to reduce cost and management overhead.                              promise the client by exploiting the vulnerable pack-
                                                                           age. Note that most package managers will not down-
3.     THREAT MODEL                                                        grade an existing package, so a replay attack only works
                                                                           when the package manager installs a new package, not
   The threat model used in this paper involves an attacker
                                                                           when it updates an existing package.
that can respond to requests made by a package manager.
This can be a man-in-the-middle, an attacker that has tricked
                                                                         • Freeze Attack Similar to a replay attack, a freeze at-
a client into contacting the wrong server (e.g. through DNS
                                                                           tack works by providing metadata that is not current.
cache poisoning), or an attacker who has gained control of
                                                                           However, in a freeze attack, the attacker freezes the
an official mirror for a distribution. The threat model is as
                                                                           information a client sees at the current point in time
follows:
                                                                           to prevent the client from seeing updates, rather than
     • The attacker can serve the client arbitrary files.                  providing the client older versions than the client has
                                                                           already seen. As with replay attacks, the attacker’s
     • The attacker does not know what package the client                  goal is ultimately to compromise a client who has vul-
       will request a priori.                                              nerable versions of packages installed. A freeze attack
                                                                           may be used to prevent updates in addition to having
     • The attacker does not have a key trusted to sign pack-              an installed package be out of date.
       ages, package metadata, or the root metadata. — Note
       that mirrors do not usually possess the private key               • Extraneous Dependencies The attacker rewrites the
       used to sign files, they only copy previously signed files          package metadata to have additional packages installed
       from the main repository.                                           alongside a package the user intends to install. For
     • The attacker has access to outdated packages, out-                  example, the attacker provides metadata that incor-
       dated package metadata, and outdated root metadata                  rectly states that package foo depends on bar. This
       files. — There are many outdated mirrors on the In-                 will cause bar to be installed when it is not desired or
       ternet where an attacker can obtain these files.                    needed. If bar has a security vulnerability, this allows
                                                                           an attacker to compromise the client’s system.
     • The attacker is aware of vulnerabilities in some out-
       dated packages and is able to exploit those vulnera-              • Endless Data This attack is performed by returning
       bilities. — This is possible by looking at change logs              an endless stream of data in response to any download
       and updates to software source files or downloading an              request. This may cause the package manager to fill up
       exploit toolkit [18].                                               the disk or memory on the client and crash the client’s
                                                                           system.
     • The attacker does not know of a vulnerability in the
       latest version of any package. — Zero-day vulnerabil-
       ities are unlikely to be known by many attackers.             4. SECURITY OF PACKAGE MANAGERS
                                                                       The security of a package manager varies depending on
     • If a package manager supports signatures, signatures
                                                                     how signatures are used to protect data. This section ex-
       are used. — If a client or distribution chooses not to
                                                                     plores the security strengths and weakness of signatures on
       use signatures supported by their package manager,
                                                                     different data along with implementation pitfalls observed
       they are as vulnerable as if they used a package man-
                                                                     in package managers (and how to fix them). This section
       ager that does not support signatures.
                                                                     then classifies the security of different signatures into a list
     • Expiration times in the root metadata are used, if sup-       ordered by increasing security.
       ported, and current (un-expired) root metadata does             The discussion groups package managers with similar se-
       not contain any vulnerable versions of packages — The         curity characteristics together. The first group of package
       root metadata is a single, small file so it is feasible for   managers do not use signatures (Section 4.1). The second
       the main repository to sign it relatively frequently with     group of package managers use signatures on packages but
       short expiration times.                                       do not use signatures on package metadata or root metadata
                                                                     (Section 4.2). The third group of package managers use sig-
3.1 Attacks                                                          natures on package metadata but not on the root metadata
   Given this threat model, there are several attacks that           (Section 4.3). The final group of package managers use sig-
may be used on a client. The impact of these attacks varies,         natures on the root metadata (Section 4.4).
but all allow the attacker to either crash or control the
client’s computer (possibly via exploiting a package with a          4.1 Package Managers Without Security
known vulnerability). Each of the following attacks is effec-          There are three popular package managers that do not
tive on at least some of the package managers we studied:            provide security: Pacman, ports1 , and Slaktool. These pack-
                                                                     age managers do not sign packages, package metadata, or the
     • Arbitrary Package The attacker provides a package             root metadata file. As a result, any attacker that controls
       they created in place of a package the user wants to          a mirror can trivially launch an arbitrary package attack by
       install.                                                      responding to client requests with malicious software.
     • Replay Attack An attacker replays older versions of           1
                                                                       A version of ports used by NetBSD did support package
       correctly signed packages or metadata, causing clients        signatures at one time [7], but this has been obsoleted and
       to install an old package with security vulnerabilities       is not maintained or used.
4.2 Package Signatures                                           combination of packages (such as those that include only
   YUM and urpmi rely solely on signatures embedded in           older versions with known vulnerabilities) available on the
packages to provide security. There is no protection of pack-    mirror. Portage is vulnerable to replay attacks in the same
age metadata or the root metadata. As a result, an at-           way as freeze attacks.
tacker can launch replay or freeze attacks and have clients        In Stork, all of the package metadata hashes are in the
install vulnerable packages. An attacker can choose to in-       same signed file. This prevents an attacker from choosing
clude any vulnerable versions of signed packages they wish       package metadata that existed on the repository at different
on the repository.                                               times. Stork checks that timestamps on files are increasing
   However, neither YUM nor urpmi verify that the package        to prevent replay attacks but does not prevent freeze attacks.
metadata they initially received for dependency resolution         Fixing the package managers It is possible to add re-
matches the embedded package metadata of subsequently            play attack protection to Portage through timestamp check-
downloaded packages. This allows an attacker to forge pack-      ing and to add protection against endless data attacks to
age metadata to launch an extraneous dependencies attack.        both Portage and Stork. However, both package managers
This means that if there exists a package that the attacker      will still be vulnerable to freeze attacks.
knows how to exploit and which the user does not have in-
stalled, the attacker can cause it to be installed whenever      4.4 Root Metadata Signatures
any other package is installed by the user. The result is that      The package managers APT, APT-RPM, and YaST use
an attacker can compromise essentially every client that in-     signatures on the root metadata. All three of these package
stalls or updates a package.                                     managers optionally support package signatures as well, but
   Both YUM and urpmi are also vulnerable to endless data        this functionality is not widely used in practice.
attacks. For example, when YUM is given a repomd.xml                Package metadata is stored in compressed files and the se-
file of unlimited size, it exits without printing an error af-   cure hashes of those files are stored in the root metadata file.
ter the filesystem is full — leaving the huge file on disk.      As the root metadata is protected by a signature, the pack-
Since no information is logged or printed about the error,       age metadata is protected from tampering which prevents
this makes discovering the problem complicated (especially       extraneous dependencies attacks. In addition, the signature
if YUM runs via auto-update).                                    on the root metadata prevents a mirror from hosting ver-
   Fixing the package managers Even without chang-               sions of packages that were on the main repository during
ing the signature methods, it is possible to modify these        different time periods. The attacker must choose a time pe-
package managers to prevent the extraneous dependencies          riod of the main repository to copy and provide exactly those
attack. Doing so requires verifying that downloaded pack-        files when launching a replay attack. Unfortunately, none of
age metadata is the same as the embedded metadata in any         these package managers check the order of timestamps to
downloaded package. Furthermore, endless data attacks can        prevent replay attacks or have any mechanisms to prevent
be prevented by capping the size of downloaded data. How-        freeze attacks or endless data attacks.
ever, these changes are not adequate to protect against re-         Fixing the package managers              It is easy for these
play or freeze attacks. Given the large number of packages       package managers to protect against endless data attacks
that need to be re-signed to prevent replay or freeze attacks,   as well as to prevent replay attacks. Replay attacks can
we believe that adding root metadata signing is the most         be prevented by adding a timestamp to the root metadata
practical way to address this issue.                             and checking that any newly downloaded root metadata is
                                                                 not older than the version the client last obtained. In fact,
4.3 Package Metadata Signatures                                  all three package managers have a timestamp available and
   The Portage and Stork package managers use signatures         merely need to add this check.
on package metadata; however, they do so in different ways.         To mitigate the effectiveness of freeze attacks, package
Each package in Portage has a separate, signed package           managers could add an expiration time to the root metadata.
metadata file for each version of the package. The package       Clients would refuse to use a root metadata file if the current
metadata contains the secure hash of the package (possibly       time is greater than the expiration time. Since the root
along with hashes of related files such as patches). In con-     metadata is a single, small file, it is feasible to re-sign this
trast, Stork users create a single file that contains a times-   file often and require every mirror to be frequently updated
tamp and the secure hash of the package metadata for all of      (most distributions already require their public mirrors to
the packages that the user trusts. Users can also delegate       update at least once a day).
trust to other users and all users typically delegate trust to
a single “distribution” user. The analysis of Stork therefore    4.5 Classification
focuses on the security of the packages trusted by the dis-         The security mechanisms and vulnerabilities of the pack-
tribution user because the security of the distribution user     age managers are summarized in Figure 2. All of the pack-
affects all clients.                                             age managers studied are vulnerable to endless data attacks
   These package managers are not vulnerable to extrane-         as well as freeze attacks. Different package managers have
ous dependencies attacks because the signatures protect the      varying resistance to other attacks such as replay attacks.
package metadata. However, resistance to metadata tam-           Depending on the package manager’s security mechanisms,
pering does not imply that the package manager is resistant      the result can be any of the following, where those listed first
to all attacks. Both package managers are vulnerable to          also imply those listed after: arbitrary packages created by
endless data attacks.                                            the attacker are installed, any vulnerable package can be
   In both Portage and Stork, an attacker can launch freeze      installed alongside non-vulnerable packages a client installs
attacks. In Portage, since each package has a different file     using an extraneous dependencies attack, mismatched out-
for metadata signatures, an attacker can choose to have any      dated packages are installed (in that they existed on the
 Name           Signature    Package Installation   Metadata Abuse     software from a repository or mirror. However, it is not
 Pacman         nothing      arbitrary              arbitrary          uncommon for a user to have a stand-alone package that was
 ports          nothing      arbitrary              arbitrary
 Slaktool       nothing      arbitrary              arbitrary
                                                                       created by a party they trust and that they need to verify
 YUM            (1)          alongside              arbitrary          is free from tampering. Stand-alone packages are packages
 urpmi          (1)          alongside              arbitrary          that are not obtained through the package manager’s normal
 Portage        (2)*         mismatch               replay / freeze    channels at install time. Stand-alone packages may have
 Stork          (1)*, (2)    snapshot               freeze
                                                                       been obtained manually from unofficial sources or may even
 APT            (1)*, (3)    snapshot               replay / freeze
 YaST           (1)*, (3)    snapshot               replay / freeze    be packages a user has created. Another source provides an
 APT-RPM        (1)*, (3)*   snapshot               replay / freeze    extended discussion [29] describing why stand-alone package
                                                                       verification is an important and desirable feature.
Figure 2: Package managers, their protection mech-                        The signing of only root metadata does not allow any
anisms and vulnerabilities. The protection mecha-                      practical way to verify stand-alone packages. Package man-
nisms are numbered (1) packages, (2) package meta-                     agers that use signed root metadata could be modified to
data, (3) root metadata. ’*’ indicates that support                    keep copies of all metadata obtained from the repository
exists but is not in common use.                                       for future verification of stand-alone packages, but this only
                                                                       helps for packages a user manually downloads from the same
Signatures                Best Case               Common Case          repository that they access through their package manager.
Protecting         Package      Metadata      Package   Metadata       This also fails to satisfy one of the primary reasons given
                                Abuse                   Abuse
                                                                       for being able to verify stand-alone package signatures: ver-
No Security        arbitrary    arbitrary     arbitrary arbitrary
Package            mismatch     replay /      alongside arbitrary      ifying signatures for files when they are only available for
                                freeze                                 manual download and installation, not through a repository.
Package Meta-      mismatch /   freeze        mismatch    replay   /      Package managers that sign package metadata tend to be
data               snapshot                               freeze
Root    Meta-      current      none          snapshot    replay   /
                                                                       more able to meet the needs of stand-alone package verifi-
data                                                      freeze       cation than the package managers who only sign root meta-
                                                                       data. However, the way in which package metadata is stored
Figure 3: Classification of package manager protec-                    has a major impact on usability in this case. Similarly with
tion schemes. This demonstrates both the security                      package managers that sign only root metadata, package
that is possible to achieve using a scheme as well as                  managers would need to store old package metadata and this
what is commonly provided by existing implemen-                        would only be of use for verifying stand-alone packages that
tations.                                                               came from a repository the user normally uses. In other
                                                                       cases, the user would need to be sure to always keep the
                                                                       signed package metadata with the package for verification
main repository at different times), or installed packages will        purposes. This is far from an ideal option.
come from a collection of outdated packages that all existed              Signatures embedded in packages are thus the most practi-
at the same time on the main repository (i.e. in the same              cal option and provide the greatest ease of use when stand-
snapshot).                                                             alone packages must be verified. All that a user needs in
   Based on the observation and analysis of the security in            order to verify a package is the package itself. A drawback
existing package managers, it is possible to similarly classify        with having signatures in the package is that signatures are
the security mechanisms. As Figure 3 shows, one can obtain             constrained by the limits of the package format so multiple
an ordering of the security of the mechanisms. Clearly, hav-           signatures may not be supported.
ing no signatures allows the most attacks and is the most                 Using signatures embedded in packages for stand-alone
vulnerable. Signatures on packages provide a definite im-              package verification does have complications, though. No-
provement over no signatures, but gives the attacker the               tably, users must have the requisite public keys available in
ability to manipulate metadata arbitrarily and provides at-            order to verify package signatures. They must also ensure on
tackers the ability to populate a mirror with packages of mis-         their own that packages they are installing are not outdated
matched versions, or, if package metadata isn’t verified using         or have vulnerabilities. However, there are many scenarios
the signed packages, the ability to cause vulnerable pack-             where a user can use embedded package signatures in a way
ages to be installed alongside any non-vulnerable packages.            that increases security in their specific situation.
Signatures on package metadata prevent the attacker from
doing more than replaying or freezing the package metadata,            6. DEPLOYMENT EXPERIENCE
but if the signatures are in separate files, the attacker can
                                                                          To gain more experience with what security mechanisms
still mismatch versions of packages. By preventing replay
                                                                       work well in practice, we modified the package manager
and freeze attacks in package managers that sign the root
                                                                       Stork and added root metadata signing. We added an expi-
metadata, a package manager will only install current pack-
                                                                       ration time to the root metadata to prevent freeze attacks.
ages and is immune to metadata tampering.
                                                                       Since Stork already supported both package signatures and
                                                                       package metadata signatures, this allowed us to experiment
5.   ADDITIONAL USABILITY NEEDS                                        with all types of signatures in a single package manager.
  This section focuses on additional usability requirements               The changes to Stork for root metadata signing were tested
users have for package management. Most importantly, the               and beta-deployed and finally incorporated into the produc-
use case where a user has an uninstalled package on their              tion release. Interestingly, Stork differed from all other se-
computer they need to verify.                                          cure package managers in that there was no key already
  The standard use case of the package managers and their              trusted by clients to validate communication from the repos-
security mechanisms is where a user needs to securely install          itory. As the only signed files in Stork were the package
metadata files signed by individual users, there had never       the core distribution. Next, we examine how the mecha-
been a need for the repository to have its own key that the      nisms by which requests are distributed to different mirrors
clients trusted. This required distributing a repository key     impacts security. We then examine the use of HTTPS to
to clients in order for them to make use of the new root         try and determine its effect and applicability. The section
metadata signatures. The key was included with the ini-          concludes with a look at the discussed distributions and a
tial release of the updated version of Stork. This initial       comparison of their security characteristics.
key distribution was secure because the majority of users,
through their trusted packages files, delegate trust to the      7.1 Obtaining a Mirror
Stork team to provide them updated Stork packages. Stork’s          To evaluate the feasibility of controlling mirrors of popular
design meant that users would not be using this key for          distributions, we set up public mirrors for the CentOS, De-
trusting packages, but rather only verifying metadata files      bian, Fedora, openSUSE, and Ubuntu distributions. A fic-
downloaded from the repository.                                  titious company (Lockdown Hosting) with its own domain,
   The resulting changes were transparent to the users both      website, and fictitious administrator (Jeremy Martin) were
in terms of performance and usability. The overhead of using     used as the organization maintaining the mirrors. A server
package metadata signing along with root metadata signing        with a monthly bandwidth quota of 1500 GB was leased for
was measured and found to be negligible (between 2-5%).          $200 per month through The Planet (www.theplanet.com).
Ultimately, there were few comments about the addition of           Setting up a public mirror for each distribution involved
root metadata signatures since the existing security mech-       acquiring the packages and metadata from an existing mir-
anism (package metadata signatures) was retained without         ror and then notifying the distribution maintainers that the
modification. Though transparent to the users, they gained       mirror was online and available for public usage. The distri-
increased security through the addition of root metadata         butions varied in terms of the degree of automation in the
signatures.                                                      public mirror application and approval process as well as
   We examined packages on the Stork repository to find that     whether newly listed mirrors have traffic immediately and
user-uploaded packages did not include package signatures,       automatically directed to them. In all cases, the distribu-
indicating that researchers were not using the optional pack-    tions accepted our mirror and added it to the official mirror
age signature feature of Stork. While far from conclusive,       list for use by outside users. We saw traffic on our mirrors
this implies that package metadata signatures and package        from a variety of clients, including military and government
signatures are redundant. We reason that when multiple sig-      computers. More detail about our mirrors can be found in
nature mechanisms are provided, either package metadata          a tech report [5].
signatures or package signatures are sufficient for usability
purposes, but it is unnecessary to support both.                 7.2 Security Repository
   To conclude, since no one scheme works well from both a          Debian and Ubuntu both use an official repository that
security and a usability standpoint, we propose that pack-       serves security updates (packages that fix vulnerabilities).
age managers should use multiple security mechanisms. It is      This prevents a mirror from launching a replay attack be-
clear that root metadata signatures should be included be-       cause the package manager will use the latest version of a
cause of their security benefits. It also seems advantageous     package which will be available from the security reposi-
to have either package metadata signing or package signa-        tory. However, this protection does not extend to a man-
tures for usability. By combining root metadata signatures       in-the-middle attacker since the repositories do not support
with either signed package metadata or signed packages, a        HTTPS.
package manager can obtain a high degree of security and            Both Debian and Ubuntu use several mirrors beside the
excellent usability without significant performance impact.      security repository. In the case where the security repository
                                                                 is down, an attacker can use a mirror to serve outdated
7.   PACKAGE MANAGERS IN PRACTICE                                content that was originally from the security repository to
   This section examines additional functionality that is pro-   perform a replay attack. In addition, using multiple mirrors
vided by some of the most popular distributions (Ubuntu,         makes Ubuntu and Debian users much more vulnerable to
Debian, Red Hat Enterprise Linux, Fedora, CentOS, open-          endless data attacks.
SUSE, and SUSE Enterprise Linux) that impacts the secu-
rity of package managers in practice. It should be noted that    7.3 Mirror Selection
we did a cursory examination of other popular distributions        In many distributions, not every source of data is created
(Gentoo, Mandriva, LinuxMint, Sabayon Linux, Slackware,          equal. For example, openSUSE distributes all of the meta-
KNOPPIX, Arch Linux, and MEPIS Linux) and did not find           data from a central source and only outsources package re-
any additional security practices that significantly altered     quests to mirrors. In this section we examine the impact
the security over that provided by their package manager.        that these practices have on the security of different distri-
We did not survey any smaller distributions, but we strongly     butions.
suspect that the majority of them are at least as vulnerable       OpenSUSE uses a download redirector that sends some
as their package manager is by default.                          requests to mirrors. However, the download redirector serves
   In this section, we first examine the feasibility of an at-   package metadata and root metadata directly. This means
tacker obtaining an official mirror for a distribution. We       that the client gets the metadata from a trusted source (not a
examine this by setting up mirrors for five popular distri-      mirror). While this doesn’t protect against all of the attacks
butions that have official mirrors run by outside parties        a client may face (such as endless data attacks by mirrors
(Ubuntu, Debian, Fedora, CentOS, and openSUSE). We               or replay attacks by a man-in-the-middle attacker), it does
then examine the security pitfalls in using a security repos-    make it much more difficult to launch an attack.
itory for package updates and a set of untrusted mirrors for       On CentOS, clients contact a central service which redi-
rects the client’s requests to official mirrors. Requests for      servers using a special plug-in instead. During our discus-
all types of content (packages, package metadata, and root         sion with the Red Hat security team, they realized there
metadata) are directed to mirrors that are not controlled by       were issues in how SSL was used in the plug-in as well and
CentOS. The specific mirror a client is redirected to may be       began working on a fix for it.
different for each file requested. The result is that an aver-
age CentOS client contacts many more mirrors than would            7.6 Comparison
normally be the case. This makes it much easier for an at-           We examine the additional security mechanisms provided
tacker to conduct an endless data attack on many clients           by different distributions and comment on their effectiveness
but complicates replay or freeze attacks because the same          against attack.
repository may not be contacted for packages and metadata.
However, an attacker can still effectively launch an extrane-         • SUSE Linux Enterprise doesn’t use mirrors hosted by
ous dependencies attack.                                                outside parties and uses HTTPS so it is not vulnerable
   On Fedora, by default clients contact a central service              to the attacks described in this paper.
called MirrorManager that is very similar to the CentOS
                                                                      • OpenSUSE, by using a download redirector, provides
service. However, MirrorManager allows a mirror adminis-
                                                                        significant protection from malicious mirrors. How-
trators to specify that clients in an IP address range should
                                                                        ever, any other attacker, such as a man-in-the-middle,
use only their mirror. This allows easy targeting of attacks
                                                                        that can respond to client requests can perform replay
(to a specific country or organization) and reduces the num-
                                                                        or freeze attacks. If the download redirector fails or
ber of other parties who will consume resources on the mir-
                                                                        is unreachable (e.g. due to a denial-of-service attack),
ror. Note that the mirror need not have an IP address in
                                                                        users cannot get updates but are not at risk of replay
the IP address range that it targets.
                                                                        attacks. Despite the download redirector protecting
   Online mirror redirectors handle failures differently than
                                                                        users from replay or freeze attacks from malicious mir-
a security repository. If a mirror redirector fails, the clients
                                                                        rors, users are still vulnerable to endless data attacks
will stop trying to communicate and fail (as opposed to
                                                                        from mirrors.
“failing-open” by skipping the failed repository).
   Many distributions including Gentoo, KNOPPIX, Arch                 • Ubuntu and Debian have similar security to openSUSE
Linux, Debian, and Ubuntu also have mirror selection tools              with the additional problem that if the security repos-
that find a nearby mirror. These tools do not improve the               itory fails, they become vulnerable to replay or freeze
security of users since they are only used to select a sin-             attacks from mirrors.
gle mirror to download all content from. In fact, it should
be noted that popular mirror selection tools for Debian and           • Red Hat Enterprise Linux is not at risk from malicious
Ubuntu (such as netselect-apt or Software Sources) do not               mirrors (since it uses no mirrors hosted by outside par-
preserve the official security repository. This means that              ties) but is vulnerable to man-in-the-middle attackers
users who have used these tools to find faster or more reli-            because of flaws in their HTTPS implementation. A
able mirrors are unlikely to use the security repository for            malicious party who can act as a man-in-the-middle
updates!                                                                can launch any of the attacks that YUM is vulnerable
                                                                        to.
7.4 Verifying Mirrors
                                                                      • With a man-in-the-middle attacker, CentOS is vulner-
   Many distributions use an automated mechanism to verify              able to every attack (much like RHEL). In the case
that mirrors are staying up-to-date. While we did not at-               of a malicious mirror, their download redirector makes
tempt to deviate from the correct, up-to-date status on our             it more difficult to launch attacks that require the at-
mirrors, we posit that an attacker can determine the IP ad-             tacker to provide a snapshot of packages (like a replay
dress of the checking server and serve up-to-date content to            or freeze attack), but the potential for abuse from end-
the checking server while serving malicious content to users            less data and extraneous dependencies attacks is still
it is attacking. Note that this is trivial on distributions like        large.
Fedora that allow a mirror administrator to selectively tar-
get users, because the only references coming from outside            • Fedora has the same vulnerabilities as CentOS but
the targeted range should be the checking server. It is pub-            with the additional problem that an attacker can tar-
licly acknowledged by some distributions [13] that detecting            get attacks to a specific IP range. This also allows an
and tricking these bots is trivial and so we do not feel this           attacker to launch attacks that require the attacker to
is a viable security mechanism.                                         provide a snapshot of the repository.

7.5 HTTPS                                                             • Other distributions such as Gentoo, Mandriva, Lin-
                                                                        uxMint, Sabayon Linux, Slackware, KNOPPIX, and
  Red Hat Enterprise Linux (RHEL) and SUSE Linux En-
                                                                        MEPIS Linux do not significantly alter the protection
terprise are the only distributions that we found which widely
                                                                        provided by their package manager.
support or use HTTPS. When investigating how YUM uses
HTTPS, we discovered it does not validate SSL certificates.
This means that while the communication uses HTTPS,                8. RESULTS
there is no validation that the endpoint YUM communicates             To understand the impact of an attacker that controls
with is correct (allowing a man-in-the-middle to pose as the       a mirror, a trace of package requests was conducted on a
repository).                                                       CentOS mirror and used to estimate the number of clients
  However, the Red Hat security team informed us that              that could be compromised or crashed by an attacker. For all
on RHEL, YUM communicates with the Red Hat Network                 of the popular distributions our mirror could have launched
                                160000                                             6000                                                              3500
                                             Package Requests                                                                                                      Arbitrary




                                                                                                                    Cumulative Clients Compromised
                                140000             Unique IPs                                                                                                   Mismatched




  Cumulative Package Requests
                                                                                   5000                                                              3000         Snapshot




                                                                                          Cumulative Unique IPs
                                120000
                                                                                                                                                     2500
                                                                                   4000
                                100000
                                                                                                                                                     2000
                                80000                                              3000
                                                                                                                                                     1500
                                60000
                                                                                   2000
                                                                                                                                                     1000
                                40000
                                                                                   1000                                                               500
                                20000

                                     0                                             0                                                                    0
                                         0     50   100 150 200        250   300                                                                            0       50     100    150      200   250   300
                                                     Time (in hours)                                                                                                          Time (in hours)

Figure 4: CentOS Mirror Traffic.         This figure                                                              Figure 5: Compromised Clients CDF. This figure
presents the cumulative package downloads and re-                                                                 presents the cumulative number of clients compro-
quests from unique IP addresses for the CentOS mir-                                                               mised over a 13 day period for an attacker with 25
ror over a 13 day period.                                                                                         vulnerable versions. This figure shows the effect of
                                                                                                                  the security mechanism on the number of clients
                                                                                                                  compromised.
an endless data attack to crash all of the clients that visited
it. The number of unique IP addresses that contacted our
mirror represents an upper bound on the number of clients                                                         for those package managers that require a snapshot (pack-
that could be crashed each week.                                                                                  ages that were all on a repository during the same time
                                                                                                                  period).
8.1 Mirror Traces                                                                                                    Determining which package versions are remotely vulner-
   The CentOS mirror was chosen for this analysis because                                                         able to attack proved to be difficult and we are unaware of
it was the longest mirror experiment that was conducted,                                                          a data set that provides a good model of this. To compen-
lasting 13 days. The package access trace gathered from                                                           sate, we randomly chose a set of vulnerable versions from
the CentOS mirror is shown in Figure 4. The number of                                                             all non-current versions of packages. In practice, we believe
package requests and number of requests from unique IP                                                            that an attacker would be more likely to work to discover
addresses increase roughly linearly over this time period.                                                        vulnerabilities in old versions of popular packages as these
Assuming the CentOS user base is not growing faster than                                                          would allow the attacker to compromise more clients. This
our mirror serves clients, we would expect the number of                                                          work does not capture this effect.
unique IP addresses to flatten out over time, however our                                                            A client that uses a mirror with a package manager that
trace is not long enough to capture that effect. Since clients                                                    has missing or inadequate security (Pacman, Slaktool, ports,
are counted as unique by IP address, multiple clients behind                                                      YUM, and urpmi) is considered compromised whenever they
a NAT box or proxy are counted as a single client. There are                                                      install a package from the mirror. A client that uses a pack-
many instances where a single IP address has several orders                                                       age manager that allows an attacker to mismatch vulner-
of magnitude more package requests than the median client,                                                        able package versions (Portage) is considered compromised
often with many requests for the same package. This implies                                                       when it installs any package with a known vulnerability. For
that clients are using NAT boxes or proxies in practice.                                                          clients with snapshot vulnerability to replay and freeze at-
   The openSUSE and Fedora mirrors (not shown) had sim-                                                           tacks (Stork, APT, YaST, APT-RPM), the attacker chooses
ilar traffic effects as the CentOS mirror. The Debian and                                                         a snapshot time and can compromise clients who install a
Ubuntu mirrors (not shown) were both up for only a short                                                          package that the attacker has a vulnerable version that was
period, but did not demonstrate this effect. We suspect                                                           current at that time. We defer details of how the attacker
this is because they do not automatically distribute requests                                                     chooses the snapshot time to a tech report [5].
among the mirrors, instead requiring manual selection by a                                                           Replay attacks only impact clients that are installing new
user or the use of a tool like netselect-apt. Since our mir-                                                      packages instead of those that are updating existing pack-
rors were only listed a few days, they did not attract a large                                                    ages. From the mirror’s request log, it isn’t possible to de-
number of Debian or Ubuntu users.                                                                                 termine whether a client is installing or updating a package.
                                                                                                                  This means the results for Portage, Stork, APT, APT-RPM,
8.2 Package Versions and Vulnerability                                                                            and YaST represent an upper bound. The package managers
  To perform our analysis, we needed to know the distri-                                                          without security (Slaktool, ports, Pacman) are not impacted
bution of package versions over time on the mirror, as well                                                       because an attacker can create arbitrary packages. Similarly,
as which of those versions are vulnerable to attack. Infor-                                                       the results for YUM and urpmi are accurate because as long
mation about the 58165 versions of the 3020 RPM CentOS                                                            as there exists a vulnerable package the client hasn’t updated
packages used in the last year was captured. The update                                                           to a newer version, an extraneous dependencies attack can
times were captured and used in the data set to determine if                                                      compromise the client.
different versions existed on the main repository at the same
time. This information was used to estimate compromises                                                           8.3 Number of Compromised Clients
   Using the CentOS trace and version information, the num-                                       700
ber of clients compromised by a malicious mirror was esti-                                                  Mismatch
                                                                                                            Snapshot




                                                                  Clients Compromised per Week
mated (Figure 5). Note that this estimation doesn’t take                                          600
into account the security practices of the distribution. As
                                                                                                  500
described in Section 7.6, users from distributions with ad-
ditional security practices may be safer than what is de-                                         400
scribed here. We provide information about the security
of the package manager because many of the smaller dis-                                           300
tributions reuse package managers for more popular distri-
butions and use only the security mechanisms provided by                                          200
their package manager.
                                                                                                  100
   As the true number of vulnerable packages is not known,
we use a value of 25 vulnerable packages (the effect of vary-                                       0
ing the number of vulnerable packages is shown in Figure 6).                                            5    10   15    20 25 30 35 40       45   50
These plots show that the security model of the package                                                                Vulnerable Packages
manager has a great impact on the number of clients that
can be compromised. A client that restricts attackers to mis-   Figure 6: Clients Compromised Per Week. This fig-
matched vulnerable package versions reduces the maximum         ure presents the estimated number of clients com-
estimated number of compromised clients by about a factor       promised per week by an attacker that controls a
of 4 to around 900 over the 13 day period. Package man-         mirror. The error bars show one standard devia-
agers whose security mechanisms require a snapshot reduce       tion. The plots are offset slightly for readability.
the maximum estimated number of clients compromised to
under 500. A package manager with signatures on the root
metadata and protection against replay and freeze attacks                                        rectly checks certificates, the distribution can set up
(for example, modified Stork) will not have any compromises                                      repositories or mirrors that support HTTPS transfers.
from an attacker that controls a mirror.                                                         This will not protect against a malicious mirror, but
   The package managers that allow the attacker to mis-                                          will prevent a man-in-the-middle attack from an out-
match packages or that require a snapshot vary based upon                                        sider.
the number of vulnerable packages. The effect of varying
                                                                             • The distribution can review their mirror policy care-
the number of vulnerabilities is shown in Figure 6. The
                                                                               fully and validate administrator credentials before putting
number of packages with vulnerabilities that the attacker
                                                                               a mirror on the official mirror list. The distribution
can exploit is on the x-axis. The plots show that package
                                                                               should also review their policies for deciding how mir-
managers that allow an attacker to choose different versions
                                                                               rors are allocated traffic to prevent a mirror from being
of packages that existed on the root repository at different
                                                                               able to target attacks (perhaps indirectly by advertis-
times (mismatch) are more vulnerable than package man-
                                                                               ing high bandwidth).
agers that require a snapshot.
   This figure clearly shows that a package manager that                     • The distribution’s mirrors should use a secure connec-
requires an attacker to present a consistent set of packages                   tion (e.g. SSH) to synchronize with the main repos-
provides better security than one that allows an attacker to                   itory to prevent an attacker from impersonating the
mismatch packages. Somewhat disturbing is the significant                      main repository.
number of clients that can be compromised if there are only
5 vulnerable package versions.                                               • Users should check that the versions of the packages
   Our leased server was bandwidth-limited and mirrored                        their package manager recommends for installation are
multiple distributions simultaneously for cost reasons. An                     recent through multiple sources.
attacker would likely expend more bandwidth or set up mul-
tiple mirrors to capture additional traffic, thus leading to    10. RELATED WORK
more compromises.
                                                                   Many package managers have GUI front-ends [15, 27].
                                                                These GUI-based tools are usually just a different interface
9.    DISCLOSURE AND DISCUSSION                                 to the functionality provided by a command line package
   After disclosing these vulnerabilities to package manager    manager and so are identical from a security standpoint.
developers via Ryan Giobbi at CERT [12], we were contacted         There are many techniques that help to support software
by proactive developers at Gentoo and openSUSE who con-         security such as systems that ensure the authenticity and
firmed our findings and began working on fixes [14]. We         integrity of software (including SFS-RO [16], SUNDR [17],
were also able to get in touch with developers at CentOS,       Deployme [20], and Self-Signed Executables [30]), and code
Red Hat, Fedora, Ubuntu, and Debian and had confirma-           signing certificates [6]. These are complimentary to the so-
tion and discussion about how we recommend they fix the         lutions presented in this paper.
issues we uncovered [10]. We believe that most of the other        There are several systems that access multiple mirrors
distributions will follow suit and either patch the vulnera-    to improve download performance and avoid DoS attacks.
bilities themselves or ask their package manager developer      Byers et al. [4] describe using Tornado codes to improve
to do so. In the interim, we provide some suggestions about     performance by downloading from several mirrors simulta-
how distributions and users can increase their security:        neously. This has the side-effect of allowing the client to
                                                                make progress even if one of the mirror misbehaves. Sharma
     • If the package manager supports HTTPS and it cor-        et al. [24] describe having the client “hop” between mirrors
while downloading a file. This prevents an attacker from              fedora-infrastructure-list/2008-July/m%
launching a DoS attack on the client because the attacker             sg00114.html.
does not know which mirror the client will use next. We          [11] man dpkg-sig.
hope that raised awareness will result in organizations try-          http://pwet.fr/man/linux/commandes/dpkg_sig.
ing these techniques in practice.                                [12] R. Giobbi. Vulnerability Analysis Blog: Safely Using
                                                                      Package Managers. http://www.cert.org/blogs/
11. CONCLUSION                                                        vuls/2008/07/using_package_managers.html.
   This work identifies security issues in ten popular pack-     [13] J. Hughes. HughesJR.com – Attacks on Package
age managers in use today. Furthermore, we demonstrate                Managers – ummm...
that while some security mechanisms used by distributions             http://www.hughesjr.com/content/view/22/1/.
may help to prevent attacks, others may actually decrease        [14] R. H. Johnson. [gentoo] Index of
security. We estimate that when ignoring any additional se-           /users/robbat2/tree-signing-gleps.
curity mechanisms, an attacker with a mirror that costs $50           http://viewcvs.gentoo.org/viewcvs.py/gentoo/
per week can compromise between 150 and 1500 clients each             users/robbat2/tree-signing%-gleps/.
week. These security issues have been disclosed to distribu-     [15] The KPackage Handbook. http://docs.kde.org/
tions that are currently working to fix these problems.               development/en/kdeadmin/kpackage/.
                                                                 [16] D. Mazières, M. Kaminsky, M. F. Kaashoek, and
12. ACKNOWLEDGMENTS                                                   E. Witchel. Separating key management from file
                                                                      system security. In Proc. 17th SOSP, pages 124–139,
  We would like to thank our shepherd Steven Murdoch
                                                                      Kiawah Island Resort, SC, Dec 1999.
and the anonymous reviewers for their insightful comments
and feedback. We would like to thank Ryan Giobbi at the          [17] D. Mazières and D. Shasha. Building secure file
CERT for helping us to responsibly disclose these issues to           systems out of Byzantine storage. In PODC ’02:
the package manager maintainers. We would like to thank               Proceedings of the twenty-first annual symposium on
Jake Edge, Dag Wieers, Kees Cook, Jan iankko Lieskovsky,              Principles of distributed computing, pages 108–117,
Robin Johnson, Ludwig Nussel, Peter Poeml, Marcus Meiss-              New York, NY, USA, 2002. ACM.
ner, Josh Bressers, Tomas Hoger, Jason Gunthorpe, Kyri-          [18] milw0rm - exploits : vulnerabilities : videos : papers :
cos Pavlou, Mike Piatek, Chris Gniady, Wenjun Hu, and                 shellcode. http://www.milw0rm.com.
Tadayoshi Kohno for their comments on our research. We           [19] Netcraft: Strong growth for Debian.
would also like to thank the members of the Stork project for         http://news.netcraft.com/archives/2005/12/05/
their help in creating Stork and their input on this research.        strong_growth_for_debian.%html.
This work was funded in part by the PlanetLab Consortium.        [20] K. Oppenheim and P. McCormick. Deployme:
                                                                      Tellme’s Package Management and Deployment
13. REFERENCES                                                        System. In Proc. 14th Systems Administration
                                                                      Conference (LISA ’00), pages 187–196, New Orleans,
 [1] Debian APT tool ported to Red Hat Linux.                         LA, Dec 2000.
     http://www.apt-get.org/.
                                                                 [21] Gentoo-Portage. http://gentoo-portage.com/.
 [2] APT-RPM. http://apt-rpm.org/.                               [22] Installing Applications: Packages and Ports.
 [3] Arch Linux (Don’t Panic) Installation Guide.                     http://www.freebsd.org/doc/en_US.ISO8859-1/
     http://www.archlinux.org/static/docs/                            books/handbook/ports.html.
     arch-install-guide.txt.                                     [23] RPM Package Manager. http://www.rpm.org/.
 [4] J. Byers, M. Luby, and M. Mitzenmacher. Accessing           [24] P. Sharma, P. Shah, and S. Bhattacharya. Mirror
     multiple mirror sites in parallel: using Tornado codes
                                                                      hopping approach for selective denial of service
     tospeed up downloads. INFOCOM’99. Eighteenth
                                                                      prevention. Object-Oriented Real-Time Dependable
     Annual Joint Conference of the IEEE Computer and                 Systems, 2003.(WORDS 2003). Proceedings of the
     Communications Societies. IEEE, 1, 1999.                         Eighth International Workshop on, pages 200–208,
 [5] J. Cappos, J. Samuel, S. Baker, and J. Hartman. A                2003.
     Look In the Mirror: Attacks on Package Managers.
                                                                 [25] Slackware Package Management. http:
     Technical Report TR–08–06, Department of Computer                //www.slacksite.com/slackware/packages.html.
     Science, University of Arizona, Jul 2008.
                                                                 [26] Stork. http://www.cs.arizona.edu/stork.
 [6] Introduction to Code Signing. http://msdn2.
                                                                 [27] Synaptic Package Manager - Home.
     microsoft.com/en-us/library/ms537361.aspx.
                                                                      http://www.nongnu.org/synaptic/.
 [7] A. Crooks. The netbsd update system. In ATEC ’04:
                                                                 [28] URPMI. http://www.urpmi.org/.
     Proceedings of the USENIX Annual Technical
     Conference, pages 17–17, Berkeley, CA, USA, 2004.           [29] dkpg-sig support wanted?
     USENIX Association.                                              http://nixforums.org/about101637-asc-15.html.
 [8] debsigs - What is debsigs. http:                            [30] G. Wurster and P. van Oorschot. Self-Signed
     //linux.about.com/cs/linux101/g/debsigs.htm.                     Executables: Restricting Replacement of Program
                                                                      Binaries by Malware. In 2nd USENIX Workshop on
 [9] DistroWatch.com: Editorial: How Popular is a
                                                                      Hot Topics in Security, Boston, MA, Aug 2007.
     Distribution? http://distrowatch.com/weekly.php?
     issue=20070827#feature.                                     [31] YaST - openSuSE. http://en.opensuse.org/YaST.
[10] M. Domsch. Re: YUM security issues...                       [32] Yum: Yellow Dog Updater Modified.
     https://www.redhat.com/archives/                                 http://linux.duke.edu/projects/yum/.
