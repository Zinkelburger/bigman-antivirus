---
url: "https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf"
title: "Survivable Key Compromise in Software Update Systems"
captured: "2026-11-02T18:47:41Z"
via: "live"
sha256: "63fac2cb50b1da504079707f34b5a7f903273e241352e3f033659ad98ee75c44"
wayback: "https://web.archive.org/web/20240417232141/https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf"
cited_in:
  - "docs/research/linux-av-landscape-and-papers.md"
  - "docs/threats/03-distro-packaging-and-install-channels.md"
---

# Survivable Key Compromise in Software Update Systems

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf>
> Snapshot: <https://web.archive.org/web/20240417232141/https://uptane.org/assets/files/samuel_ccs_2010-2e4e7a69695e936b1ad3b0b7010c550a.pdf>

  Survivable Key Compromise in Software Update Systems

                                               ∗
                     Justin Samuel                                     Nick Mathewson                         Justin Cappos
                      UC Berkeley                                         The Tor Project                University of Washington
                Berkeley, California, USA                         nickm@alum.mit.edu                     Seattle, Washington, USA
               jsamuel@berkeley.edu                                                                   justinc@cs.washington.edu
                                                                      Roger Dingledine
                                                                          The Tor Project
                                                                        arma@mit.edu

ABSTRACT                                                                             1.   INTRODUCTION
Today’s software update systems have little or no defense                               Software update systems span a wide range of uses and
against key compromise. As a result, key compromises have                            designs. This includes their use as package managers [4,
put millions of software update clients at risk. Here we iden-                       59, 60], library managers [16, 20, 44, 48], and application up-
tify three classes of information whose authenticity and in-                         daters [34, 36]. All of these software update systems play
tegrity are critical for secure software updates. Analyzing                          a critical role in computer security. They must discover,
existing software update systems with our framework, we                              download, verify, and apply updates when security flaws are
find their ability to communicate this information securely                          discovered in installed software. Crucial to the security of
in the event of a key compromise to be weak or nonexistent.                          this process is that all downloaded files are ensured to be
We also find that the security problems in current software                          authentic. Though some systems do not authenticate up-
update systems are compounded by inadequate trust revo-                              dates [2,7,11], many do. In practice, authentication in these
cation mechanisms. We identify core security principles that                         systems requires cryptographic signatures. The security of
allow software update systems to survive key compromise.                             software update systems therefore relies heavily on the in-
Using these ideas, we design and implement TUF, a software                           ability of attackers to obtain private keys that are trusted
update framework that increases resilience to key compro-                            to provide updates.
mise.                                                                                   Historically, software update systems have been designed
                                                                                     such that the compromise of a single trusted key is fatal.
Categories and Subject Descriptors                                                   With a compromised key, attackers who can respond to
                                                                                     client requests can cause clients to install malicious soft-
K.6.5 [Management of Computing and Information                                       ware. In many systems, even if the developers learn of a
Systems]: Security and Protection; D.2.0 [Software En-                               key compromise, there is no secure means of key revoca-
gineering]: Protection Mechanisms                                                    tion. This risk is not theoretical: popular Linux distribu-
                                                                                     tions with tens of millions of users have suﬀered panics due
General Terms                                                                        to key compromise [23, 46], weak key generation is a con-
                                                                                     tinuing threat [12, 28, 58], and there are many known PKI
Security, Design
                                                                                     vulnerabilities [30, 53]. Recently, attention has focused on
                                                                                     the possibility of government-compelled certificate issuance,
Keywords                                                                             a tool that governments may use to tamp down on political
Software updates, authentication, delegation, key manage-                            dissidents [13, 52]. While that work highlighted the risk of
ment, key compromise, revocation, threshold signatures                               such attacks to secure web browsing, their threat extends
                                                                                     into software update systems, which commonly rely on PKI
                                                                                     for transport layer security as well as code signing. Having
                                                                                     compromised a key by any of these means, attackers can
                                                                                     impersonate update servers using DNS cache poisoning [19],
                                                                                     BGP prefix hijacking [42], posing as (or compromising) a
                                                                                     legitimate content mirror [11], or any other method of inter-
∗Work done while at the University of Washington.                                    cepting client requests.
                                                                                        Ideally, software update systems would remain safe even
                                                                                     when some of their keys are under the control of an at-
                                                                                     tacker. This property of survivability—the ability for a
Permission to make digital or hard copies of all or part of this work for            system to function correctly while under attack or partial
personal or classroom use is granted without fee provided that copies are            compromise—is important for building reliable systems. We
not made or distributed for profit or commercial advantage and that copies           believe a software update system cannot remain secure if
bear this notice and the full citation on the first page. To copy otherwise, to      an attacker who can respond to client requests has compro-
republish, to post on servers or to redistribute to lists, requires prior specific   mised all its keys. However, with the proper understanding
permission and/or a fee.
CCS’10, October 4–8, 2010, Chicago, Illinois, USA.
                                                                                     of software update system vulnerabilities, we find updaters
Copyright 2010 ACM 978-1-4503-0244-9/10/10 ...$10.00.                                can be designed such that specific attacks requiring partial
key compromise are less likely to succeed. Additionally, the     The security properties of current software update systems
severity of many successful attacks can be decreased.            are examined in Section 5. Design principles for securing
   We begin by identifying the information that software         software update systems are identified in Section 6. We
update systems must authenticate in order to perform se-         describe the design of our framework and analyze its key
cure updates. This information is the content of updates,        compromise survivability in Section 7. We discuss our ex-
the availability (timeliness) of updates, and the consistency    periences and make recommendations for balancing security
of information describing updates. Unfortunately, the keys       with usability in Section 8. We look at related work and
that software update systems use to protect this information     conclude in Sections 9 and 10.
are vulnerable to compromise in many ways. We examine
popular software update systems, all of which authenticate       2.     THREAT MODEL
some data, and find no principled approach to securing this
                                                                      Our threat model assumes the following:
information in the event of key compromise.
   We then define a set of design principles that can improve          • Attackers can compromise at least one of a software
the resilience of software update systems to key compromise.             update system’s trusted keys.
We have applied these principles in our design of TUF, an              • Attackers compromising multiple keys may do so at
open source software update framework. TUF is the next                   once or over a period of time.
generation of the Thandy [33] design originally developed              • Attackers can respond to client requests.
for secure updates of Tor [54]. The design of TUF uses re-
sponsibility separation and delegation, multi-signature trust       We consider the attacker as successful if they can con-
with threshold signatures and multiple roles, and both im-       vince the updater to install (or leave installed) something
plicit and explicit trust revocation. TUF can be integrated      other than the most up-to-date version of the software it is
with both new and existing software update systems. We           updating. If the attacker is preventing the installation of up-
discuss our experiences integrating TUF with two dissimilar      dates, they want the updater to not realize there is anything
software update systems: a traditional application updater       wrong.
and Python’s library management system. Based on our                We deem the ability for an attacker to have arbitrary files
experiences, we discuss practical implications of our design     signed by a key to be functionally equivalent to a key com-
and make recommendations for key threshold sizes, meta-          promise. To a software update system, the ability to sign
data expiration times, and appropriate use of automated          arbitrary files is indistinguishable from possession of the key.
signing.                                                            We assume that the internal processes used by project
                                                                 administrators to manage their codebase and software are
Main contributions:
                                                                 secure. Though important for overall client security, these
   • We draw attention to the danger of relying on sin-          processes are out-of-scope for software update systems.
     gle signing keys and PKI1 in software update systems.
     PKI is exceptionally high-risk in update systems as the     3.     SOFTWARE UPDATE SYSTEMS
     software provider does not control the trust chain. We         Software update systems find, download, and install the
     look at a variety of update systems and find partial or     latest versions of software. They usually begin by making
     complete reliance on PKI in most of these systems.          requests to servers that host the updates. The server re-
   • We identify three fundamental classes of information        sponds with information describing available updates and
     that software update systems must authenticate to en-       the update system then decides which updates to download
     sure correct and timely updates. This information is        and install. Often, the servers from which updates are down-
     the content of updates, the availability of updates, and    loaded are not owned by the same organization that created
     the correct combination of updates.                         the updates. These third-party repository mirrors may be
   • We show how role separation can be used in software         provided by volunteers or may be owned by content delivery
     update systems to reduce the impact of key compro-          networks (CDNs).
     mise. Central to this separation is the observation that       Software update systems span a range of purposes. Pack-
     an individual key’s manner of use and storage aﬀects        age managers such as YUM [60] and APT [4] are responsible
     its likelihood of compromise. We demonstrate how re-        for most of the software installed on an operating system.
     sponsibilities can be divided between roles whose keys      Library managers such as Python’s easy install [20] are com-
     have diﬀerent likelihoods of compromise in order to         monly provided with programming environments to install
     reduce the overall risk to clients.                         and update optional libraries. Application updaters are in-
   • We design and implement a software update system            cluded with individual applications such as Firefox so that
     that uses multiple-signature trust and role separation      the application can update itself without needing, for ex-
     without requiring SSL or other protocols that store         ample, a package manager, or manual intervention by the
     keys on public-facing servers.                              user.

   Paper organization: A threat model is given in Sec-           3.1      Authentication
tion 2. Software updates systems are described along with           Software update systems must verify the authenticity of
their vulnerabilities and information responsibilities in Sec-   files they receive. These files may be the actual software up-
tion 3. Threats to key security are discussed in Section 4.      dates, may describe available updates, or may provide other
1
  We use the term PKI to refer to contemporary PKI based         information that the updater uses to make download and
on third-party CAs. Our design includes what is, in fact, a      trust decisions. Software update systems that authenticate
public key infrastructure. To avoid ambiguity, we will not       none of this information are insecure and exploiting them
refer to it as a PKI.                                            has been well studied [2, 7].
      Information               Vulnerability                      3.2.2    Timeliness of Updates
      Content of updates        Malicious software                    If an attacker can prevent a software update system from
      Timeliness of updates     Freeze attack                      learning that updates are available, the attacker can also
      Repository state          Metadata inconsistency             prevent the installation of security updates. As a result, at-
                                                                   tackers gain time to exploit known vulnerabilities in software
Table 1: Information responsibilities in software up-              installed on client systems. We call this a freeze attack.
date systems and the corresponding vulnerabilities                    To prevent freeze attacks, clients must know when updates
that exist when these responsibilities are compro-                 are available. Though a client cannot stop an attacker from
mised.                                                             preventing updates altogether, a client should be able to
                                                                   detect when an attacker is replaying responses to the client’s
                                                                   previous queries for updates. Thus, clients must be able to
   In this work, we are not concerned with systems that oﬀer       authenticate information that indicates the availability of
no security. Instead, we focus on systems designed for secu-       updates.
rity whose architectures are nevertheless dangerously fragile
with respect to key compromise. Among the systems that             3.2.3    Repository State
do authenticate the information they receive, the two pri-           In a metadata inconsistency attack, a software update sys-
mary methods of authentication are transport layer security        tem is tricked into using a combination of metadata and up-
and cryptographically signed files.                                date files that never existed together on the repository at
   Many software update systems rely on transport layer se-        the same time. Depending on the design of the software
curity [34, 36]. These systems verify that they are retrieving     update system, the result of this attack varies. For exam-
trusted files by using an authenticated transport protocol         ple, a metadata inconsistency attack may allow an insecure
(such as SSL/TLS), and verifying the public key certificate        version of a dependency to be installed [11].
of the server from which they retrieve updates. The secu-            We call the set of files available from a repository at a
rity of these systems is heavily based on the security of an       given time the repository state. To remain secure against
underlying PKI. Trust is bootstrapped by the client having         metadata inconsistency attacks, software update systems
a set of trusted root CA certificates.                             need access to authenticated information that indicates
   Other software update systems use signed data [4, 59, 60].      whether any subset of downloaded files is from the same
In these systems, clients know one or more keys that they          repository state.
trust to provide updates. When files are downloaded, the
signatures on these files are checked.
   A few systems use both transport layer security and signed      4.    THREATS TO KEY SECURITY
files [59, 60]. Of these, some rely on PKI-based file signing.
                                                                      There are many ways signing keys and transport keys can
Thus means that despite the use of code signing, clients are
                                                                   be compromised or bypassed. Some of these categories of
still vulnerable to compromise as a result of PKI problems
                                                                   risk have been considered before in the context of key com-
such as those we will discuss in Section 4.
                                                                   promise [27].
   It is common for software update systems to directly au-
thenticate only a single metadata file through either a sig-
nature on the file or transport layer security. This metadata      4.1     PKI Vulnerabilities
file includes cryptographic hashes of all other available meta-       Trust based on a chain of single keys is compromised if any
data and update files. By comparing the hashes of the files        key in the chain is compromised. Thus, the use of traditional
downloaded over insecure channels with those listed in the         PKI for trusting a key within an update system, whether
initial signed or SSL-transferred metadata, updaters ensure        that key is for signing files or for transport layer security,
the authenticity of these other files. This technique is espe-     is problematic. For example, a single trusted CA certificate
cially useful to larger projects that rely on SSL, as SSL has a    held by a malicious party puts all clients at risk.
significant performance impact on servers providing updates           Recently, the risk of rogue CA certificates was shown to be
for many clients.                                                  more than theoretical [53]. As many clients rely on certifi-
                                                                   cates to indicate the location of a revocation server (OCSP
3.2 Information Responsibilities                                   responders or CRLs), rogue CA certificates may be diﬃcult
   To address key compromise in software update systems,           to revoke. This risk of rogue CA certificates is in addition
we must consider attacks specifically aimed at them. Vari-         to the inherent danger of compromise of any CA’s private
ous attacks on software update systems are described in [11].      key.
Here we classify the underlying vulnerabilities and identify          With PKI, there is also the risk that one of the certificate
the information that software update systems must authen-          authorities improperly issues certificates. A notable case of
ticate in order to remain secure. These information respon-        a fraudulently issued certificate is the issuing by VeriSign of
sibilities and the related vulnerabilities are listed in Table 1   a certificate in the name of Microsoft Corporation to an at-
and described below.                                               tacker claiming to be a Microsoft employee [15]. Incorrectly
                                                                   issued certificates are also fundamental to the null prefix at-
3.2.1 Content of Updates                                           tack discovered in 2009 [31]. This attack uses a certificate
  For an attacker to compromise clients through the update         whose common name is formatted as the domain to imper-
process, the easiest approach is to provide malicious soft-        sonate, followed by a null byte, followed by a domain owned
ware to a client downloading updates. To protect against           by the attacker. Many CAs would sign such a certificate and
installation of malicious updates, software update systems         many SSL implementations would consider only the portion
must be able to verify that all updates are authentic.             of the common name before the null byte. As a result, an
attacker could obtain a certificate that would be trusted for        implementers of software update systems can choose widely
any domain.                                                          trusted cryptographic algorithms to lessen the likelihood of
  Fraudulently issued certificates can also be the result of         algorithmic weaknesses. Similarly, the ability for an attacker
compelled certificate creation. In such an attack, a govern-         to derive a private key through brute force methods can be
ment agency compels a CA to issue a certificate in order to          hampered by using large key sizes.
enable the agency to intercept secure communication [52].              However, there are areas of cryptographic weaknesses
  Despite the risks of PKI-based authentication, many soft-          that these systems have much less control over. One major
ware update systems rely solely on PKI for authenticating            problem is that implementation flaws in crypto libraries
updates. Examples of such systems include Firefox’s built-           can render cryptographic algorithms insecure. Such flaws
in updater [22] as well as Google’s Windows and Mac up-              even include the vulnerability of SSL keys to side-channel
daters [24, 25].                                                     attacks [9]. Additionally, the discovery of implementation
                                                                     problems resulting in weak key generation has occurred
4.2 Key Theft                                                        multiple times and has aﬀected thousands of actively used
   Private keys can be stolen either by an outsider gaining          keys [12, 28, 58]. In these situations, it is often the case that
unauthorized access to a system storing the key or by an             the private key can be quickly recovered.
insider with access to the key.
   The more easily accessible a key is, the easier it is to steal.
In the case of transport keys, the private key will often ex-        5.   ANALYSIS OF CURRENT SYSTEMS
ist unencrypted on disk and will always exist in memory                 In this section, we look at how software update systems
unencrypted. If the server software is compromised, a pri-           in use today secure the classes of information identified in
vate key in the process’s memory will not be safe even when          Section 3.2. There is a large amount of similarity in the
access control mechanisms such as SELinux are used. Even             security properties of these systems, which we summarize in
without obtaining the private key, an attacker who has com-          Table 2. Therefore, we will focus on the general properties
promised such a system can impersonate the repository and            and discuss in more detail the notable diﬀerences.
reply to requests from software update clients. The com-                Current systems authenticate the content of updates in
promise of web servers is not uncommon. There have been              three primary ways: using SSL, signed files, and with Au-
many remotely exploitable vulnerabilities in web server soft-        thenticode. Though Authenticode, Microsoft’s form of code
ware [3, 26] as well as in SSL communication libraries used          signing, uses signed files, we make a distinction because Au-
by web servers [35, 43].                                             thenticode has the security problems of PKI. Whether the
   Even though signing keys are generally maintained more            key that signs files is known directly to the client or authen-
securely than transport keys, they can still be compromised          ticated via a PKI, all of these systems use a single key’s
if an attacker gains access to an internal server through            signature on signed files. From the standpoint of immediate
vulnerabilities or compromised accounts. Two recent high-            danger due to key compromise, Authenticode and SSL are
profile compromises of software signing systems are those of         strictly worse than direct key trust because, as discussed in
the popular Linux distributions Fedora and Red Hat.                  Section 4, there are more keys that can be compromised as
   In August 2008, the server that was used to sign oﬃcial           well as other equivalent attacks.
Fedora software was compromised [23]. This server had on                Update availability information is generally either authen-
it a passphrase-protected key. Fedora staﬀ would type the            ticated using SSL or not at all. No update systems provide
passphrase on the system when software needed to be signed.          or encourage methods of ensuring reasonable timeliness of
If the attacker was able to obtain the key’s passphrase either       update information using signed files.
directly (for example, though keystroke logging) or through             The correctness of the repository state seen by clients af-
brute force, they would have been able to gain control of            fects package managers and library managers more than ap-
any Fedora client that performed an update or new software           plication updaters. Repositories for application updaters
installation. Fedora distributed a new key to clients through        often contain only a small number of files (and sometimes a
an insecure means (using the potentially compromised key             single metadata file), whereas package and library managers
to sign a package to replace the key). Fedora’s website cur-         typically need to handle hundreds or thousands of packages
rently states that they intend to consider better methods for        at a time. Some package managers do provide repository
key migration in the future [21].                                    state information through SSL or signed metadata. How-
   Around the same time as the Fedora signing key compro-            ever, many package managers do not. Also unfortunate is
mise, Red Hat had one of its systems used for package sign-          that none of the programming language library managers we
ing compromised. The attacker was able to sign multiple              looked at [16, 20, 44, 48] authenticate repository state infor-
malicious versions of the OpenSSH software with the Red              mation or any other aspects of their updates. Some library
Hat signing key [46]. Red Hat believes the key itself was not        managers, however, do have incomplete proposed schemes
compromised because they use custom hardware requiring               for signed packages [49].
physical access to the machine in order to obtain the private           Google’s update system for Windows software, Omaha, of-
key. However, once the malicious software was signed, there          fers an alternative to SSL for providing timely update avail-
was no secure way to revoke the key or the client’s trust of         ability information. This alternative cryptographic protocol
the malicious, signed packages.                                      is the Client Update Protocol (CUP) [14]. CUP provides
                                                                     authenticity and freshness for software updates with a sin-
4.3 Crypto and Implementation Weaknesses                             gle request and response over an insecure connection. CUP
  In addition to theft of keys, cryptographic weaknesses also        is not a replacement for SSL as it does not provide pri-
pose a danger to software update systems. There can be a             vacy, client authentication, or prevention of replay of client
degree of control over some of these threats. For example,           requests (it does prevent replay of server responses). The
                                                 Update Content        Update Timeliness     Repository State
                     yum (Fedora 10)             signed                unprotected           unprotected
                     yum (Fedora 11)             SSL + signed          SSL                   SSL
                     yum (CentOS)                signed                unprotected           unprotected
                     yum (Red Hat)               SSL + signed          SSL                   SSL + signed
                     APT (Ubuntu)                signed                unprotected           signed
                     YaST (OpenSUSE)             SSL + signed          SSL                   SSL
                     YaST (SUSE Ent.)            SSL + signed          SSL                   SSL
                     slackpkg (Slackware)        signed                unprotected           unprotected
                     Sparkle                     SSL and/or signed     SSL or none           SSL and/or signed
                     Update Engine (Google)      SSL                   SSL                   SSL
                     Omaha (Google Update)       SSL + Authenticode    SSL                   SSL
                     Omaha with CUP              CUP + Authenticode    CUP                   CUP
                     Adobe AIR applications      SSL + signed          SSL                   SSL + signed
                     Firefox (Windows)           SSL + Authenticode    SSL                   SSL
                     Firefox (Mac/Linux)         SSL                   SSL                   SSL
                     Firefox extensions          SSL or signed         SSL or none           SSL or none


               Table 2: Software update systems and the authenticated information they provide.


CUP specification [14] does not address revocation of the           trust can be achieved in two ways: threshold signatures and
public key that clients trust.                                      responsibilities shared between roles with separate keys.
                                                                       A single role can have its resilience to key compromise
                                                                    increased by using a (t, n) threshold signature scheme. That
6. DESIGN PRINCIPLES                                                is, the signatures of at least t signers are required out of a set
   The software update systems discussed in Section 5 share         of n potential signers. An attacker who compromises t − 1
many of the same weaknesses. These weaknesses stem from             keys cannot successfully attack any clients.
common design decisions that largely ignore the potential              Distinct from threshold signatures, which use multiple
for key compromise. In this section, we describe methods            keys within the same role, another approach is to require
by which software update systems can resist and recover             the signatures of multiple roles. The roles involved in multi-
from key compromise. These concepts are not new but their           role trust share a responsibility between them. Sharing a
importance and applicability to software update systems has         responsibility is diﬀerent from delegating a responsibility.
not been generally understood. They will guide the design           When one role delegates a responsibility to another, there is
of our framework in Section 7.                                      no multi-role trust that results, as either role is trusted for
                                                                    the responsibility without the other role’s signature.
6.1 Responsibility Separation                                          These two multiple-signature approaches, threshold sig-
   A software update system can have diﬀerent roles that are        nature schemes and responsibilities shared between roles,
trusted for diﬀerent responsibilities. These responsibilities       can be used together. For example, if role A uses thresh-
may or may not overlap. By carefully dividing these respon-         old (tA , nA ), role B uses threshold (tB , nB ), and there are
sibilities, we can achieve a higher level of security by limiting   no keys in common between nA and nB , then an attacker
which attacks are enabled when the key for a single role is         would need to compromise at least (tA , nA ) and (tB , nB )
compromised. A simple example of responsibility separation          keys in order to compromise any responsibilities that these
can be seen in the two-role designs that use SSL in conjunc-        roles share.
tion with a single signing key. In these designs, the signed
data does not provide timeliness information; that is solely
the responsibility of SSL.                                          6.3    Explicit and Implicit Revocation
   In addition to clients trusting completely unrelated roles          There are multiple reasons for revoking keys in software
for diﬀerent responsibilities, another way to achieve sepa-         update systems. These range from lost keys, suspected or
ration of responsibilities is through delegation. That is, a        known weak keys, suspected or known compromised keys,
role can delegate some of its responsibilities to other roles.      and rotation of project members. We divide revocation into
Delegation can be performed by providing the client with a          two types: implicit revocation and explicit revocation.
certificate signed by the delegating role that describes which         Implicit revocation occurs when trust is revoked automat-
responsibilities are delegated and to whom.                         ically when some criteria is met. For example, the signatures
   The division of responsibilities between roles does not by       on a file may be considered to be expired after a specific date
itself improve key compromise survivability. For example,           or a key may only be trusted to sign information a set num-
if the same keys are used for all roles, there is no benefit.       ber of times before clients stop trusting the key.
However, this separation is valuable when diﬀerent keys are            Explicit revocation, on the other hand, requires clients to
used as well as when combined with other design principles.         be told that they should stop trusting keys they currently
                                                                    trust. Explicit revocation can be performed by having the
6.2 Multi-signature Trust                                           trust-delegating role sign a message indicating that trust
  One of the most obvious ways to prevent software up-              should be removed. Explicit revocation is always useful to
date systems from being at risk due to key compromise is            indicate that specific keys should no longer be trusted. It
to require the signatures of multiple keys. Multi-signature         is important to ensure that explicit revocation mechanisms
                                                                   framework based on whether updates are installed after be-
                                                                   ing retrieved. TUF, being a framework, performs the secure
                                                                   retrieval of updates but leaves installation to the software
                                                                   update system it is integrated with.
                                                                      For space reasons, some details of TUF’s design have been
                                                                   omitted. TUF is the second generation design of Thandy,
                                                                   the updater originally developed for Tor [54]. Detailed de-
                                                                   scriptions of both are available in the Thandy spec [32] and
                                                                   the TUF spec [56].

                                                                   7.1     Design Overview
                                                                      We use the following terminology to describe the design
                                                                   of TUF:
                                                                      Target files. Target files are the files (updates) that a
                                                                   software update system ultimately wants to download and
                                                                   install.
                                                                      Metadata files. Metadata files are signed files that de-
                                                                   scribe roles, other metadata files, and target files.
                                                                      Repositories and mirrors. A repository is a conceptual
Figure 1: Overview of roles and files in TUF when                  source of named metadata and target files. Each repository
used with a software update system that does not                   has one or more mirrors. These mirrors are the actual hosts
perform targets delegation. The value of the release               providing the repository’s content.
role increases when there are delegated targets roles                 Roles. There is one root role per repository. There are
and thus more metadata on the repository.                          multiple roles whose responsibilities are delegated to them
                                                                   directly or indirectly by the root role. The term top-level
                                                                   role refers to the root role and any role delegated by the
are not vulnerable to freeze attacks that may prevent clients      root role. Each role has a single metadata file that it is
from being aware of the revocation.                                trusted to provide.
   By combining multiple signatures with reliable revocation       7.1.1    Repository Contents
and replacement of keys, the result is a system that is proac-
tive. Such a design oﬀers resistance to long-term attacks                metabase/
where diﬀerent keys are compromised over time.                               root.txt
                                                                             timestamp.txt
6.4 Minimizing Risk                                                          release.txt
   Key risk is the product of the probability of the key’s com-              targets.txt
promise and the impact of its compromise. When threshold                     targets/foo.txt               (optional)
schemes are used, risk can also be considered in terms of role               targets/foo/bar.txt           (optional)
risk, the product of the probability of at least (t, n) keys be-         targetbase/
ing compromised and the impact of the role’s compromise.                     a.rpm                         (example target)
The same concept of risk extends to responsibilities shared                  x/y.dll                       (example target)
by multiple roles.
   Unfortunately, risk in this context cannot be clearly quan-
tified. The probability of a key’s compromise is diﬃcult to                    Table 3: Layout of a repository.
determine and it is similarly diﬃcult to quantify the im-
pact of various responsibilities under the control of an at-         The repository contents consist of metadata files and tar-
tacker. What is clear is that we should decrease overall risk      get files. Table 3 shows a sample repository layout. The
as much as possible. When a responsibility would be very           names of the directories “metabase” and “targetbase” can
dangerous in the hands of an attacker, the relevant keys           vary on each mirror. The content and layout of files under
should be used and stored in a way that reduces the chance         those directories, however, is the same on all mirrors of a
of their compromise. Keys that are stored on systems that          given repository.
are not network-connected are generally much safer than              The required metadata files are:
those stored on network-connected systems. Similarly, keys
on network-connected systems that are not public-facing              − root.txt: Specifies keys of top-level roles.
are generally much safer than those stored on public-facing          − timestamp.txt: Specifies the latest release.txt.
servers.                                                             − release.txt: Specifies the latest versions of all meta-
                                                                       data files other than timestamp.txt.
7. DESIGN AND ANALYSIS                                               − targets.txt: Specifies available target files.
  The security concepts discussed in Section 6 provide ways        These files are discussed in more detail in Section 7.3.
to understand and increase key compromise survivability in
updaters. This section describes our application of these          7.1.2    Client Workflow
concepts in The Update Framework (TUF). We distinguish               Here we outline the workflow of a software update system
between a full software update system and a software update        using TUF to check for and obtain updates. This workflow
shows the steps involved when only the required metadata          7.3    Roles and Responsibilities
files are in use. Whenever an unresolvable problem is en-            We use a root role that is responsible for delegating to
countered, the software update system using the framework         other roles their responsibility for each type of information.
must decide how to proceed. The updater may want to show          These other top-level roles are the targets role, release role,
information about the problem in a GUI, log the problem,          and timestamp role as shown in Figure 1.
or email an administrator.                                           Root role. The root role is the root of trust for the en-
   Note that all metadata is verified by ensuring the required    tire repository. The root role signs the root.txt metadata
threshold of trusted signatures, a valid creation time, a fu-     file (Table 4). This file indicates which keys are authorized
ture expiration time, and that the file is not older than the     for each of the top-level roles, including for the root role it-
last seen version of the same metadata file. With all files       self. The roles “root”, “release”, “timestamp”, and “targets”
except timestamp.txt, the client knows and verifies the ex-       must be specified and each has a list of KEYIDs. The sig-
pected hashes and length of each file.                            natures of these keys are trusted to sign on behalf of the
                                                                  corresponding role. The signature threshold for each role is
  1. The software update system instructs TUF to check            the value THRESHOLD. The corresponding public key for
     for updates.                                                 each KEYID is specified in the “keys” object.
  2. TUF downloads and verifies timestamp.txt.                       The keys belonging to the root role are intended to be
  3. If timestamp.txt indicates that release.txt has              very well protected and used with the least frequency of any
     changed, TUF downloads and verifies release.txt.             keys in the framework.
  4. TUF determines which metadata files listed in re-               Targets role. Through the targets.txt file, the targets
     lease.txt diﬀer from those described in the last re-         role is responsible for indicating which target files are avail-
     lease.txt that TUF has seen. In the case where no            able from the repository. More precisely, the targets role
     optional metadata files are in use, only root.txt and        shares the responsibility of providing information about the
     targets.txt are listed. If root.txt has changed, the         content of updates. The two roles it shares this responsibil-
     update process starts over using the new root.txt.           ity with are the release role and the timestamp role. This
  5. TUF provides the software update system with a list          sharing is a result of the release and timestamp roles need-
     of available files according to targets.txt.                 ing to indicate the latest version of targets.txt in their
  6. The software update system instructs TUF to down-            metadata files (indirectly in the case of the timestamp role),
     load a specific target file.                                 as shown in Figure 1. Depending on whether the release
  7. TUF downloads and verifies the file and then makes           and timestamp roles are used in an automated fashion, the
     the file available to the software update system.            targets role may have essentially full responsibility for the
  8. The software update system installs the update.              content of updates. The automated use of roles will be dis-
                                                                  cussed in Section 8.
7.2 Signed Metadata Format                                           In the format of targets.txt shown in Table 4, each key
 All metadata files in TUF use canonical JSON [10] for-           of the TARGETS object is a TARGETPATH. A TARGET-
mat2 and are enclosed in the signature structure:                 PATH is a path that is relative to a mirror’s location of
                                                                  target files. Each element of HASHES is the name of a hash
        { "signed" : X,                                           algorithm and the corresponding value is the hex encoded
          "signatures" : [                                        digest of the file’s contents. LENGTH is the size, in bytes,
             { "keyid" : K,                                       of the target file. If defined, the information in “custom”
               "method" : M,                                      will be made available to the code using the framework; this
               "sig" : S }                                        optional data can be used to indicate additional information
             , ... ]                                              such as a file’s version number.
        }                                                            The targets role may delegate to other roles the responsi-
                                                                  bility for providing some or all target files. These delegated
where X is the signed object, K is the identifier of the key      target roles are specified in the “delegations” object. The
that created the signature, M is the method used to make          target paths that a delegated role is given responsibility to
the signature, and S is a signature of the canonical encoding     provide are specified.
of X. An example key format is:                                      Delegated targets roles (optional). If the top-level targets
                                                                  role performs delegation, the resulting delegated roles can
       { "keytype" : "rsa",
                                                                  then provide their own metadata files. If a delegated role is
         "keyval" : { "e" : E,
                                                                  named foo, then foo’s metadata file is available on the repos-
                      "n" : N }
                                                                  itory as targets/foo.txt. The format of the metadata files
       }
                                                                  provided by delegated targets roles is the same as that of
  The signed data of each file contains a creation timestamp      targets.txt. Delegated targets roles may also further dele-
and an expiration date. These are the CREATIONTIME                gate. As with targets.txt, the latest versions of metadata
and EXPIRES values shown in Table 4.                              files belonging to delegated targets roles are described in the
                                                                  release role’s metadata.
                                                                     Release role. The release role is responsible for ensur-
                                                                  ing that clients see a consistent repository state. It provides
2
                                                                  repository state information by indicating the latest versions
  The specific file format used is not itself important as long   of all metadata files on the repository in release.txt (Ta-
as the format is suﬃciently expressive and the data can be
represented in a canonical form for signing and signature         ble 4), which it signs. The only metadata file not listed in
verification purposes.
  {"_type" : "Root",                         {"_type" : "Targets",                   {"_type" : "Timestamp|Release",
   "ts" : CREATIONTIME,                       "ts" : CREATIONTIME,                    "ts" : CREATIONTIME,
   "expires" : EXPIRES,                       "expires" : EXPIRES,                    "expires" : EXPIRES,
   "keys" : {                                 "targets" : {                           "meta" : {
     KEYID : KEY                                TARGETPATH : {                          METAPATH : {
     , ... },                                     "length" : LENGTH,                      "length" : LENGTH,
   "roles" : {                                    "hashes" : HASHES,                      "hashes" : HASHES }
     ROLE : {                                     ("custom" : { ... }) }                , ... }}
       "keyids" : [ KEYID, ... ] ,              , ... },
       "threshold" : THRESHOLD }              ("delegations" : {
     , ... }}                                   "keys" : {
                                                   KEYID : KEY,
                                                   ... },
                                                "roles" : {
                                                   ROLE : {
                                                     "keyids" : [ KEYID, ... ] ,
                                                     "threshold" : THRESHOLD,
                                                     "paths" : [ PATHPATTERN, ... ] }
                                                   , ... }})}


                root.txt                                  targets.txt                         timestamp.txt / release.txt



      Table 4: Metadata formats. Each is wrapped in the signature structure described in Section 7.2.


release.txt is timestamp.txt, discussed below, which is              Targets role compromise. If a threshold of targets
always created after release.txt.                                 role keys are compromised, there is no immediate threat to
  Timestamp role. The timestamp role is responsible for           clients. The attacker cannot cause clients to install mali-
providing information about the timeliness of available           cious software unless the timestamp and release roles are
updates. Timeliness information is made available by fre-         also compromised. That is, due to the shared responsibility
quently signing a new timestamp.txt file that has a short         between these roles for providing update content, all three
expiration time. This file indicates the latest version of        roles need to be compromised.
release.txt. The format of timestamp.txt is the same as              If less than a threshold of targets role keys are compro-
the format of release.txt. As the timestamp role needs            mised and the compromise is noticed, these keys can be se-
to frequently sign a file, it lends itself to automated usage     curely and reliably revoked by having the root role sign a
with a single key that is potentially kept on a public-facing     new root.txt file that does not list the compromised keys.
server. By placing only this one responsibility in a role         The keys of the targets role can also be regularly changed
that has a high likelihood of compromise, we minimize the         using this same method to proactively defend against un-
resulting risk.                                                   known key compromises.
                                                                     Delegated targets role compromise. If delegated tar-
7.4 Key Compromise Analysis                                       gets roles are used, they have essentially the same risk prop-
   We now consider the ability of this design to survive the      erties as the top-level targets role. The diﬀerences are that:
compromise of one or more roles. The summary of this anal-              • It may be the case that a compromised delegated tar-
ysis is shown in Table 5. Based on this analysis, we will                 gets role is only responsible for a subset of the targets
discuss recommendations for signature thresholds for these                available from the repository. Thus, the role’s com-
roles in Section 8.                                                       promise may result in only a subset of clients (e.g.
   In the following analysis of key compromise, one can often             those on a specific operating system) being at risk if
consider the timestamp role and possibly the release role as              the timestamp and release roles are also compromised.
not being major obstacles for attackers. That is, they do               • In addition to revocation by the delegating role, the
present obstacles to attackers, but some projects may choose              compromised keys can also be indirectly revoked by
to use these roles in an automated fashion and store their                any other role in the delegation chain. Indirect revo-
keys less securely.                                                       cation occurs when a delegating role has itself been
   Root role compromise. If less than the required thresh-                revoked. As a result, all delegations it performed are
old of root keys is compromised, clients are not at any risk.             revoked.
The compromised root keys can be replaced through a new
root.txt file.                                                      Release role compromise. If a threshold of release role
   If more than the required threshold of keys belonging to       keys is compromised, there is no immediate risk of com-
the root role is compromised, an attacker who can respond         promise or even of attacks on the consistency of repository
to client requests can compromise clients. An attacker would      contents. An attacker must additionally compromise the
sign a new root.txt that lists their own keys as the keys         timestamp role in order to perform metadata inconsistency
belonging to the other roles and provide all of their own         attacks. Revocation of compromised keys is done through
signed metadata to clients.                                       the root.txt file.
   Role           Malicious      Freeze          Metadata          to a single repository, PyPI, and end-users run a library
   Compromise     Updates        Attack          Inconsistency
                                                 Attack
                                                                   management and update utility, easy install, to download
                                                                   and install new and updated libraries from PyPI. There is
   Timestamp      no             limited by      no
                                 Rel., Targ.,                      currently no security in the update discovery and installation
                                 and Root                          process.
   Release        no             no              no                   Our prototype integration [55] with PyPI and easy install
   Timestamp      no             limited by      yes               makes heavy use of targets delegation and clearly benefits
   + Release                     Targ.    and                      from the existence of the release role. In this system, PyPI
                                 Root                              owns the root keys and keeps the keys for all top-level roles.
   Targets        no             no              no                PyPI delegates to each developer a targets role that is only
   Timestamp      no             limited         no
   + Targets                     by      Rel.,
                                                                   trusted to provide the individual libraries maintained by
                                 Targ.*, and                       that developer. The developer can provide any version of
                                 Root                              these libraries, but is not trusted to provide other libraries.
   Release        no             no              no                There are over 8000 libraries on PyPI. Our integration uses a
   + Targets                                                       separate delegated role for each of these libraries. As a result
   Timestamp      yes            limited   by    yes               of the large number of delegated targets roles, the release file
   + Release                     Root                              is approximately 1.5MB in size due to listing the hashes and
   + Targets
   Root           yes            yes             yes
                                                                   lengths of each metadata file. The release file changes every
                                                                   time a developer adds a new version of a library. As a result,
                                                                   it will be frequently downloaded by clients. We implemented
Table 5: Vulnerabilities when roles are compromised                gzip compression of the release file, which brought its size
in TUF. The duration of freeze attacks is bounded                  down to 460KB. This is similar to the size of the initial meta-
by the expiration times of metadata signed by un-                  data that the insecure easy install downloads on each run,
compromised roles. *Without the release role com-                  which is 446KB. Another option would be to add support
promised, the attacker cannot provide their own tar-               for retrieving deltas (diﬀs) of large metadata files.
gets role metadata.                                                   Developers create and sign metadata on their own systems
                                                                   before uploading this metadata and the packaged libraries
                                                                   to PyPI. Developers have the option of using thresholds and
  Timestamp role compromise. The compromise of the                 delegating further. With respect to PyPI’s developer key
timestamp role allows freeze attacks on clients. Once the          management, we envision a system where developers use a
compromise is detected, the duration of freeze attacks is          secure web interface to upload their public keys to PyPI.
bounded by the earliest expiration time of any of the meta-        PyPI may choose to enforce additional security measures
data files. As with the other top-level roles, revocation of       to ensure requested key changes are legitimate and not the
timestamp keys is done through the root.txt file.                  result of compromised account credentials.

8. EXPERIENCE AND DISCUSSION                                       8.2    Recommendations
   In this section, we discuss our preliminary experiences            Number of keys. Given the existence of the root role
with TUF. We also discuss recommendations and best prac-           and three other required top-level roles in our design, many
tices that balance the security benefits of multiple roles and     keys may need to be managed. It is important for keys to
keys with the practical needs of organizations. Specifically,      be stored independently—that is, on separate systems and
we consider organizations with limited ability to store many       encrypted with diﬀerent passwords. If too many keys need to
keys securely and independently.                                   be managed by a small organization, it is likely that many
                                                                   keys would be stored together and thus have the security
8.1 Integration Experience                                         properties of a single key.
   We have done prototype integrations of TUF with two                The first consideration is that the timestamp role poses
very diﬀerent software update systems. The first is the            little immediate risk if compromised. It will also often be
application updater for Seattle [50], an application run-          used in an automated fashion and thus highly vulnerable to
ning on end-user machines that allows the safe execution           exposure if the system it resides on is compromised. Given
of untrusted code. The second is with PyPI [45] and                the low risk of just this role’s compromise, the timestamp
easy install [20], the community repository and library            role can use a single key. The release role is similar to the
management system for Python.                                      timestamp role in that the immediate risk that results from
   Seattle’s application updater is quite simple. It needs to      compromise is low. Some situations may be diﬀerent, but
identify all files for which new versions exist as well as files   in general, it is likely that the release role gains little from
that did not previously exist, download these files, save them     threshold signatures and thus can use a single key.
to the application’s installation directory, and restart any          The roles that benefit the most from threshold signatures
processes that are aﬀected by the changed files. TUF was           are the root role and the targets role. The root role is
integrated such that once all available files were successfully    very important because its compromise immediately puts all
downloaded and verified, the existing application updater is       clients at risk. Additionally, once the root role has been com-
then given access to these files. The rest of the application      promised, its keys cannot be reliably revoked and replaced
updater’s functionality did not need to be modified.               without a trusted communication channel. The targets role
   Python’s library management system provides a way for           should also use threshold signatures for similar but slightly
many mutually distrustful library developers to make their         diﬀerent reasons. Even though the immediate risk due to
libraries available to users. Developers upload their libraries    compromise of just the targets role is much less than that of
the root role, the keys for the targets role will be used more   setup requirements or assumptions [6]. However, it is trivial
frequently. As a result, the targets role’s keys will be more    to construct a multi-signature scheme by having the multi-
susceptible to compromise. Even for small projects, the bur-     signature of a message just be a list of signatures [6]. This
den of threshold signatures for these roles does not need to     scheme is what we use in our design. The advantages of
be high. The threshold can be as little as (t, n) = (2, 2) to    this approach include the simplicity of implementation and
gain some level of resilience to key compromise.                 the safety from rogue-key attacks, which are generally the
   Diversity of keys. Due to the threats of weak key gener-      result of the key setup process of proposed multi-signature
ation and algorithmic weaknesses discussed in Section 4, the     schemes. One drawback of this simple scheme is that the
types of keys as well as the libraries used to generate them     signature size grows linearly with the number of signatures.
should vary within a software update system. Our current            Like threshold schemes, proxy signature schemes for del-
implementation of TUF only supports RSA keys and only            egating signing abilities have a long history [41, 57] and the
provides one library with which to generate keys. However,       use of rights delegation in computer systems has of course
one can generate RSA keys using other crypto libraries and       existed much longer. Our design uses delegation by war-
use them in TUF.                                                 rant in which signed certificates indicate the public keys to
   Automated signing. The timestamp role will always             which specific rights are delegated. This system lends it-
be used in an automated fashion to frequently resign the         self to the use of ordinary signature schemes rather than
timestamp file. Depending on the project, the release file       special-purpose proxy signature schemes [29].
may also be signed in an automated fashion. In order to             The signature scheme we use is forward-secure and has
reduce the likelihood of compromise of keys used for auto-       the advantage of being proactive. Forward-secure signature
mated signing, the signing should take place on a system         schemes, which are key-evolving, need to be able to change
that is not public-facing. Once signed, the metadata can         the secret key without having to change the public key [1].
then be pushed to a public repository.                           We avoid this complication by using a certificate-based sys-
   Metadata expiration times. All of the metadata in             tem.
TUF includes expiration times. The appropriate lengths of           Existing approaches to certificate revocation include the
these expiration times depend on the type of metadata file.      use of semi-trusted mediators (SEMs) [8], certificate revo-
For the timestamp file, the expiration time should be short.     cation lists (CRLs), the Online Certificate Status Protocol
How short depends on the specific system. For example, if        (OCSP) [39], certification revocation trees (CRTs) [17], and
repository mirrors synchronize once a day, the timestamp         short-term certificates [40] that allow clients to set their own
file will not be able to have an expiration time of less than    recency requirements [47]. In our design, we make use of
one to two days, depending on when the synchronizations          multiple revocation approaches, including short-term certifi-
are done. The release file should have an expiration time        cates, to improve revocation against various types of adver-
not much longer than the period between expected changes         saries.
to the targets role’s metadata. Similarly, the targets role’s       Our work is not concerned with detecting key compro-
metadata should use expiration times on the order of the         mise but rather only with maintaining a secure state both
expected time until new target files will be made available.     before and after keys are known or suspected to be compro-
The root role’s metadata, on the other hand, should gener-       mised. There are methods of detecting key compromise [27],
ally have a much longer expiration time than any of the other    but these generally require signature verification to be done
metadata files. If the root role’s metadata expires too soon,    online.
then clients who had not updated for an extended period of          Our previous work identified major security flaws in Linux
time will not know whether an attacker is replaying the root     package managers [11]. That work highlighted the ease with
metadata. Another reason to use long expiration times on         which attackers can become mirrors for popular Linux dis-
the root metadata is that frequent usage of the root keys ex-    tributions and in some cases even force specific clients to
poses them to increased potential for compromise. Thus, it       use the attacker’s mirror [37]. Previous work by Bellissimo
is reasonable for some projects to use a multi-year expiration   looked at the limited use of authenticated data by software
time with the root metadata.                                     update systems [7]. That work recommended the develop-
   It is worth noting that systems which are vulnerable to at-   ment of a standard for secure updates.
tacks that modify the system time, such as may result from          We have focused on designing a security framework that
insecure communication with a time server, may be vulner-        is applicable to software update systems on any operating
able to freeze attacks due to inability to detect metadata       system and for any application. However, some applications
expiration.                                                      and operating systems can also apply the principle of least
   Use of SSL. Our design avoided reliance on SSL in order       privilege to installed updates. That is, in addition to de-
to ensure the design was not dependent on the security of        creasing the likelihood of successful attacks on clients by
public-facing servers. However, the use of SSL in conjunc-       being resilient to key compromise, these systems can also
tion with our design oﬀers further defense against freeze at-    isolate compromises by not giving updates and programs
tacks as well as an additional layer of key security when the    run from updates more privilege than required. For exam-
server and PKI are not compromised. One way to gain the          ple, web browsers that install unprivileged extensions [5, 38]
benefits of SSL with minimal overhead is to only transmit        can keep the rest of the client’s system secure even if an at-
the timestamp file over SSL.                                     tacker causes a malicious extension update. If the installed
                                                                 software is isolated from the software update system suﬃ-
9. RELATED WORK                                                  ciently that the attacker cannot interfere with future up-
                                                                 dates, a future update could return the system to a secure
  Since the introduction of the first threshold cryptosys-
                                                                 state. Operating system security mechanisms such as privi-
tem in 1987 [18], many new threshold systems have been
                                                                 leged users installing updates for unprivileged users as well
developed [51]. Often, these schemes have impractical key
as fine-grained mechanisms such as SELinux can be used for       [10] Canonical JSON - OLPC.
this purpose.                                                         http://wiki.laptop.org/go/Canonical_JSON.
                                                                 [11] Justin Cappos, Justin Samuel, Scott Baker, and John
10. CONCLUSION                                                        Hartman. A look in the mirror: Attacks on package
   Software update systems that do not authenticate updates           managers. In Proc. 15th ACM Conference on
have received increased scrutiny in recent years. Due to this         Computer and Communications Security, pages
attention, many of these systems have implemented simple              565–574, New York, NY, USA, 2008. ACM.
authentication mechanisms that cannot survive key compro-        [12] CERT/CC. CERT advisory CA-2000-09 flaw in PGP
mise. We feel that software update systems must move to               5.0 key generation, May 2000.
using the approaches we advocate in order to be resilient             http://www.cert.org/advisories/CA-2000-09.html.
to the many threats to key security. Our open source soft-       [13] Bug 476766 - add China Internet Network Information
ware update framework, TUF, allows both new and existing              Center (CNNIC) CA root certificate. https:
systems to benefit from a design that leverages responsibil-          //bugzilla.mozilla.org/show_bug.cgi?id=476766.
ity separation, multi-signature trust, trust revocation, and     [14] Open client update protocol.
low-risk roles.                                                       http://omaha.googlecode.com/svn/wiki/cup.html.
                                                                 [15] Microsoft Corporation. Microsoft security bulletin
Acknowledgments                                                       MS01-017, Mar 2001. http://www.microsoft.com/
We thank those who contributed to Thandy, including Se-               technet/security/bulletin/MS01-017.mspx.
bastian Hahn and Martin Peck. We are grateful to Geremy          [16] CPAN. http://www.cpan.org/.
Condra, Tadayoshi Kohno, Wenjun Hu, and the anonymous            [17] Internet x.509 public key infrastructure certificate and
reviewers for their valuable comments.                                certificate revocation list (CRL) profile.
   This material is based upon work supported by the Na-              http://tools.ietf.org/html/rfc5280.
tional Science Foundation under Grants No. CNS-0737890           [18] Y. Desmedt. Society and group oriented cryptography:
and CNS-0959138, and by Google. Any opinions, findings,               A new concept. In Advances in Cryptology - Crypto
and conclusions or recommendations expressed in this mate-            1987, pages 120–127. Springer, 1987.
rial are those of the author(s) and do not necessarily reflect   [19] Vulnerability note VU#800113 multiple DNS
the views of BBN Technologies, Corp., the GENI Project                implementations vulnerable to cache poisoning.
Oﬃce, or the National Science Foundation.                             http://www.kb.cert.org/vuls/id/800113.
                                                                 [20] EasyInstall - the PEAK developers’ center. http:
11. REFERENCES                                                        //peak.telecommunity.com/DevCenter/EasyInstall.
 [1] M. Abdalla and L. Reyzin. A new forward-secure              [21] New signing key.
     digital signature scheme. Advances in Cryptology -               https://fedoraproject.org/wiki/New_signing_key.
     ASIACRYPT 2000, pages 116–129, 2000.                        [22] Firefox update.
 [2] Francisco Amato. ISR-evilgrade.                                  http://www.mozilla.com/en-US/firefox/update/.
     http://www.infobyte.com.ar/down/isr-evilgrade-              [23] Paul W. Frields. Infrastructure report, 2008-08-22
     Readme.txt.                                                      UTC 1200, Aug 2008.
 [3] Vulnerability note VU#944335 Apache web servers                  https://www.redhat.com/archives/fedora-
     fail to handle chunks with a negative size, Jun 2002.            announce-list/2008-August/msg00012.html.
     http://www.kb.cert.org/vuls/id/944335.
                                                                 [24] Omaha (google update).
 [4] APT HOWTO.                                                       http://code.google.com/p/omaha/.
     http://www.debian.org/doc/manuals/apt-howto/.
                                                                 [25] Update Engine and security: How to use Update
 [5] A. Barth, A.P. Felt, P. Saxena, and A. Boodman.                  Engine in a secure manner.
     Protecting browsers from extension vulnerabilities. In           http://code.google.com/p/update-
     Proc. of the 17th Network and Distributed System                 engine/wiki/UpdateEngineAndSecurity.
     Security Symposium (NDSS 2010), 2010.
                                                                 [26] Microsoft security bulletin MS08-006 - important
 [6] Mihir Bellare and Gregory Neven. Multi-signatures in             vulnerability in internet information services could
     the plain public-key model and a general forking                 allow remote code execution (942830), Feb 2008.
     lemma. In CCS ’06: Proceedings of the 13th ACM                   http://www.microsoft.com/technet/security/
     conference on Computer and communications security,              bulletin/ms08-006.mspx.
     pages 390–399, New York, NY, USA, 2006. ACM.
                                                                 [27] M. Just and P.C. van Oorschot. Addressing the
 [7] Anthony Bellissimo, John Burgess, and Kevin Fu.                  problem of undetected signature key compromise. In
     Secure software updates: Disappointments and new                 Proceedings of the Network and Distributed System
     challenges. In 1st USENIX Workshop on Hot Topics in              Security Symposium, NDSS. Citeseer, 1999.
     Security, pages 37–43, Vancouver, Canada, Jul 2006.         [28] Werner Koch. [Announce] GnuPG’s ElGamal signing
 [8] D. Boneh, X. Ding, G. Tsudik, and C.M. Wong. A                   keys compromised, Nov 2003.
     method for fast revocation of public key certificates            http://lists.gnupg.org/pipermail/gnupg-
     and security capabilities. In Proceedings of the 10th            announce/2003q4/000160.html.
     conference on USENIX Security Symposium-Volume              [29] M. Mambo, K. Usuda, and E. Okamoto. Proxy
     10, page 22. USENIX Association, 2001.                           signatures: Delegation of the power to sign messages.
 [9] Dan Boneh and David Brumley. Remote timing                       IEICE Transactions on Fundamentals of Electronics,
     attacks are practical. In Proc. 12th USENIX Security
     Symposium, Washington, DC, Aug 2003.
     Communications and Computer Sciences,                          contain a buﬀer overflow during the SSL2 handshake
     79(9):1338–1354, 1996.                                         process, Aug 2002.
[30] Moxie Marlinspike. Defeating OCSP with the number              http://www.kb.cert.org/vuls/id/102795.
     3, 2009. http:                                            [44] PEAR - PHP Extension and Application Repository.
     //www.thoughtcrime.org/papers/ocsp-attack.pdf.                 http://pear.php.net/.
[31] Moxie Marlinspike. Null-prefix attacks against SSL        [45] Python package index : Pypi.
     certificates, 2009.                                            http://pypi.python.org/pypi.
     http://www.thoughtcrime.org/papers/null-                  [46] Critical: openssh security update, Aug 2008. http:
     prefix-attacks.pdf.                                            //rhn.redhat.com/errata/RHSA-2008-0855.html.
[32] Nick Mathewson. Thandy: Automatic updates for Tor         [47] R. Rivest. Can we eliminate certificate revocation
     bundles. https://git.torproject.org/checkout/                  lists? In Financial Cryptography, pages 178–183.
     thandy/specs/thandy-spec.txt.                                  Springer, 1998.
[33] Nick Mathewson. Thandy: Secure update for Tor -           [48] RubyGems manuals. http://docs.rubygems.org/.
     Google open source blog.                                  [49] Signing your gems - RubyGems user guide.
     http://google-opensource.blogspot.com/2009/03/                 http://docs.rubygems.org/read/chapter/21.
     thandy-secure-update-for-tor.html.                        [50] Seattle: Open peer-to-peer computing.
[34] Extension versioning, update and compatibility:                http://seattle.cs.washington.edu/.
     Securing updates. https://developer.mozilla.org/          [51] V. Shoup. Practical threshold signatures. In Advances
     en/Extension_Versioning%2c_Update_and_                         in Cryptology - EUROCRYPT 2000, pages 207–220.
     Compatibility#Securing_Updates.                                Springer, 2000.
[35] Microsoft SSL library remote compromise                   [52] Christopher Soghoian and Sid Stamm. Certified lies:
     vulnerability (ms04-011, exploit), Apr 2004.                   Detecting and defeating government interception
     http://www.securiteam.com/windowsntfocus/                      attacks against SSL. Technical Report 684, Indiana
     5CP0L0KCKO.html.                                               University Computer Science Department, April 2010.
[36] Greg Miller. Revving software with Update Engine,         [53] Alexander Sotirov, Marc Stevens, Jacob Appelbaum,
     Sep 2008.                                                      Arjen Lenstra, David Molnar, Dag Arne Osvik, and
     http://googlemac.blogspot.com/2008/09/revving-                 Benne de Weger. MD5 considered harmful today:
     software-with-update-engine.html.                              Creating a rogue CA certificate, Dec 2008.
[37] Mirror manager security risks.                                 http://www.win.tue.nl/hashclash/rogue-ca/.
     http://fedoraproject.org/wiki/Mirror_manager_             [54] Tor: anonymity online. http://www.torproject.org/.
     security_risks.                                           [55] Securing python package management - tuf: The
[38] Mozilla Labs Jetpack | Exploring new ways to extend            update framework. http://www.updateframework.
     and personalize the Web.                                       com/wiki/SecuringPythonPackageManagement.
     https://jetpack.mozillalabs.com/.                         [56] /specs/tuf-spec.txt - TUF: The Update Framework.
[39] M. Myers, R. Ankney, A. Malpani, S. Galperin, and              https://www.updateframework.com/browser/specs/
     C. Adams. RFC2560: X. 509 Internet public key                  tuf-spec.txt.
     infrastructure online certificate status protocol-OCSP.   [57] V. Varadharajan, P. Allen, and S. Black. An analysis
     Internet RFCs, 1999.                                           of the proxy problem in distributed systems. In 1991
[40] M. Naor and K. Nissim. Certificate revocation and              IEEE Computer Society Symposium on Research in
     certificate update. In in Proceedings of the 7th               Security and Privacy, 1991. Proceedings., pages
     USENIX Security Symposium, 1998.                               255–275, 1991.
[41] B.C. Neuman et al. Proxy-based authorization and          [58] Florian Weimer. [security] [DSA 1571-1] new openssl
     accounting for distributed systems. In International           packages fix predictable random number generator,
     Conference on Distributed Computing Systems,                   May 2008. http://lists.debian.org/debian-
     volume 13, pages 283–283. Citeseer, 1993.                      security-announce/2008/msg00152.html.
[42] O. Nordstrom and C. Dovrolis. Beware of BGP               [59] YaST - openSuSE. http://en.opensuse.org/YaST.
     attacks. SIGCOMM Comput. Commun. Rev,                     [60] Yum: Yellow Dog Updater Modified.
     34(2):1–8, 2004.                                               http://linux.duke.edu/projects/yum/.
[43] Vulnerability note VU#102795 OpenSSL servers
