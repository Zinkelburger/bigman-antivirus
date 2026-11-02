---
url: "https://terrapin-attack.com/TerrapinAttack.pdf"
title: "Terrapin Attack: Breaking SSH Channel Integrity"
captured: "2026-11-02T18:46:31Z"
via: "live"
sha256: "c375ba257384e256a0a5b44384064ce1dda587bb645107d3afd446b7dc636333"
cited_in:
  - "docs/threats/07-network-exposure-and-remote-access.md"
---

# Terrapin Attack: Breaking SSH Channel Integrity

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://terrapin-attack.com/TerrapinAttack.pdf>

                                                                 Terrapin Attack: Breaking SSH Channel Integrity
                                                                       By Sequence Number Manipulation

                                                       Fabian Bäumer                         Marcus Brinkmann                               Jörg Schwenk
                                                   Ruhr University Bochum                  Ruhr University Bochum                      Ruhr University Bochum




arXiv:2312.12422v2 [cs.CR] 7 May 2024
                                                                 Abstract                                      1    Introduction

                                           The SSH protocol provides secure access to network ser-             Secure Shell (SSH). While TLS is commonly used to se-
                                        vices, particularly remote terminal login and file transfer            cure user-facing protocols such as web, email, or FTP, SSH is
                                        within organizational networks and to over 15 million servers          used by administrators to deploy and maintain these servers,
                                        on the open internet. SSH uses an authenticated key exchange           often with high privilege (root) access and a large attack sur-
                                        to establish a secure channel between a client and a server,           face for lateral movement within an organization’s infrastruc-
                                        which protects the confidentiality and integrity of messages           ture. SSH was developed by Tatu Ylonen in 1995 as a secure
                                        sent in either direction. The secure channel prevents message          alternative to telnet and rlogin/rcp and has since become a
                                        manipulation, replay, insertion, deletion, and reordering. At          critical component of internet security.
                                        the network level, SSH uses the Binary Packet Protocol over               In 1996, SSHv2 was developed to fix severe vulnerabilities
                                        TCP.                                                                   in the original version. In February 1997, the IETF formed
                                                                                                               the SECSH working group to standardize SSHv2. After a
                                           In this paper, we show that as new encryption algorithms
                                                                                                               decade, it published five core RFCs [29–33]. SSHv2 provides
                                        and mitigations were added to SSH, the SSH Binary Packet
                                                                                                               cryptographic agility and protocol agility without breaking
                                        Protocol is no longer a secure channel: SSH channel integrity
                                                                                                               backward compatibility. Since its original release, dozens
                                        (INT-PST, aINT-PTXT, and INT-sfCTF) is broken for three
                                                                                                               of standardized and informal updates to the protocol have
                                        widely used encryption modes. This allows prefix truncation
                                                                                                               been published. Because of this, SSHv2 remains relevant
                                        attacks where encrypted packets at the beginning of the SSH
                                                                                                               after 25 years without major redesign, but it has also become
                                        channel can be deleted without the client or server noticing it.
                                                                                                               difficult to analyze. There is a significant risk that these
                                        We demonstrate several real-world applications of this attack.
                                                                                                               extensions of SSH interact to undermine its security goals.
                                        We show that we can fully break SSH extension negotiation
                                        (RFC 8308), such that an attacker can downgrade the pub-
                                        lic key algorithms for user authentication or turn off a new           SSH Connections. An SSH connection between a client
                                        countermeasure against keystroke timing attacks introduced             and a server begins with the Transport Layer Protocol [33],
                                        in OpenSSH 9.5. Further, we identify an implementation flaw            which defines the handshake messages for key exchange and
                                        in AsyncSSH that, together with prefix truncation, allows an           server authentication and how messages are exchanged over
                                        attacker to redirect the victim’s login into a shell controlled        TCP using the Binary Packet Protocol (BPP). After the hand-
                                        by the attacker.                                                       shake, SSH provides a secure channel1 for application data.
                                           We also performed an internet-wide scan for affected en-            At the application level, the client chooses a sequence of ser-
                                        cryption modes and support for extension negotiation. We               vices to run. In practice, the client will run precisely two ser-
                                        find that 71.6% of SSH servers support a vulnerable encryp-            vices: the Authentication Protocol [30] for user authentication
                                        tion mode, while 63.2% even list it as their preferred choice.         with a password or public key, followed by the Connection
                                           We identify two root causes that enable these attacks: First,       Protocol [31] for the bulk of SSH’s features like terminal
                                        the SSH handshake supports optional messages that are not              sessions, port forwarding, and file transfer.
                                        authenticated. Second, SSH does not reset message sequence                1We note that by channel, we refer to the integrity-protected, encrypted
                                        numbers when activating encryption keys. Based on this anal-           byte stream at the transport level, not the SSH data channels from the con-
                                        ysis, we propose effective and backward-compatible changes             nection protocol. Multiple SSH data channels can be multiplexed over the
                                        to SSH that mitigate our attacks.                                      same secure transport channel.


                                                                                                           1
                               Client                                     Server        as packets are verified and decrypted before the end of com-
                               Snd Rcv                                    Snd Rcv       munication has been seen. This allows for a trivial suffix
  ASCII-                                 SSH-2.0-PuTTY-Release-0.79
                                                                                        truncation attack, where the attacker interrupts the message
 over-TCP                                SSH-2.0-OpenSSH_9.5p1                          flow at some point during the communication. This is an
                                0 0
                                         KEXINIT: nC , algorithm_listsC
                                                                           0 0
                                                                                        inherent limitation of interactive protocols and an accepted
                                         KEXINIT: nS , algorithm_listsS                 trade-off in the design of SSH, but also, e.g., the TLS Record
                                1 0                                        0 1
                                         KEXDHINIT: gx                                  Layer. Although this attack cannot be prevented, it can be de-
                                1 1                                        1 1




     Binary Packet Protocol
                                         KEXDHREPLY: gy , pkS , sig
                                                                                        tected by requiring “end-of-communication” messages as the
                                2 1                                        1 2          last messages in both directions. TLS defines a “close_notify”
                                         NEWKEYS
                                2 2                                        2 2          alert for this purpose [42]. Although SSH also defines a D IS -
                                         NEWKEYS
                                2 3                                        3 2          CONNECT message to indicate the end of the secure channel,
                                3 3
                                         SERVICEREQUEST: ssh-userauth
                                                                           3 3          this message is optional, unidirectional, and not described as
                                4 3
                                         SERVICEACCEPT
                                                                           3 4
                                                                                        security-critical in the standard.
                                         SSH Authentication Protocol
                                         SSH Connection Protocol                        Implicit Sequence Numbers. If the MAC was only com-
                                                                                        puted over the payload of each packet, an attacker could still
Figure 1: Typical SSH handshake using a finite-field Diffie-                            delete, replay, or reorder packets. Therefore, a sequence num-
Hellman key exchange. Included sequence numbers are im-                                 ber is included in the MAC computation, corresponding to the
plicit and maintained by the BPP. Snd denotes the counter for                           position of the message in the stream. Each peer maintains
sent packets and Rcv for received packets. Sequence numbers                             two counters (starting at 0), one for each direction. The Snd
verified using authenticated encryption are in bold.                                    counter is incremented after a packet has been sent, and the
                                                                                        Rcv counter is incremented after a received packet has been
                                                                                        processed. Once the secure channel has been established, the
1.1                           SSH Channel Security                                      current value of Snd is used to compute the MAC of an outgo-
                                                                                        ing packet, and the current value of Rcv is used to verify the
In this work, we focus on the integrity of the SSH handshake
                                                                                        MAC of an incoming packet. If packets in the secure channel
and the resulting secure channel, as shown in Figure 1. Af-
                                                                                        are deleted, replayed, or reordered, the sequence numbers get
ter an initial exchange of version information directly over
                                                                                        out of sync, and MAC verification will fail.
TCP, the BPP exchanges packets, each containing precisely
one message. Initially, the BPP is used without encryption                                 Because TCP is a reliable transport, accidental reordering
or authentication for the duration of the handshake until the                           of SSH packets cannot occur on the network. Thus, SSH (like
N EW K EYS message. Afterward, the encryption and authenti-                             other TCP-based protocols) uses implicit sequence numbers
cation keys are used to form a secure channel, intending to                             that are not transmitted as part of the packet.
protect the confidentiality and integrity of the ordered stream
of all following messages. Note that technically, the secure                            Security Guarantees of Secure Channels. For TLS, the
channel consists of two separate cipher streams, one for each                           security guarantees of the Record Layer were formalized as
direction, and that the order of message arrival is only guar-                          stateful length-hiding encryption [39], with the state mainly
anteed for each direction separately.                                                   consisting of the implicit sequence number. The security
                                                                                        of the BPP and implicit sequence numbers was analyzed by
Message Authentication Codes. As SSH is an interactive                                  Bellare et al. in [4] and later refined and extended by Paterson
protocol, the integrity of each packet must be verified when it                         and Watson [40] and Albrecht et al. [1]. These works define,
is received so that it can be promptly processed. For this, the                         in slightly idealized scenarios, the following informal security
BPP appends a Message Authentication Code (MAC) to each                                 goal for a secure channel:
packet. A cipher mode and a MAC form an authenticated
                                                                                             When a secure channel between A and B is used,
encryption scheme [5]. SSH historically uses Encrypt-and-
                                                                                             the data (or message) stream received by B should
MAC (EaM), where the MAC is computed over the plaintext,
                                                                                             be identical to the one sent by A and vice versa
but this is vulnerable to oracle attacks [2]. Later, Encrypt-
                                                                                             (INT-PST, aINT-PTXT in [17]).
then-MAC (EtM) was added, where the MAC is computed
over the ciphertext instead. SSH has recently adopted the                               Within their idealizations, all three works confirm that the BPP
AEAD ciphers AES-GCM and ChaCha20-Poly1305, where                                       is indeed a secure channel. The difference between the models
ciphertext integrity is built into the encryption scheme [43].                          is that Paterson and Watson [40] also included the encrypted
                                                                                        length field of the Encrypt-and-MAC modes, while Albrecht
A Trivial Example: Suffix Truncation Attacks. Note that                                 et al. [1] considered the more recent cipher modes ChaCha20-
a per-packet MAC cannot fully protect the channel’s integrity,                          Poly1305, AES-GCM, and generic Encrypt-then-MAC. Our


                                                                                    2
attacks show that the models underlying the proofs in [1] are                                            Client                  MitM                 Server
only partially accurate. We will explain the discrepancies                                               Snd Rcv                                      Snd Rcv

between the proofs and our findings in Section 2.                         ASCII-                                     SSH-2.0-PuTTY-Release-0.79

                                                                         over-TCP                                    SSH-2.0-OpenSSH_9.5p1

                                                                                                                     KEXINIT: nC , algorithm_listsC
1.2    Overview of Our Attacks on SSH                                                                     0 0                                          0 0
                                                                                                                     KEXINIT: nS , algorithm_listsS
                                                                                                          1 0                                          0 1
In this paper, we show that SSH fails to protect the integrity of                                                    KEXDHINIT: gx
                                                                                                          1 1                                          1 1




                                                                             Binary Packet Protocol
the encrypted message stream against meddler-in-the-middle                                                           KEXDHREPLY: gy , pkS , sig
                                                                                                          2 1                                          1 2
(MitM) attacks. More precisely, we present novel prefix trun-                                                        IGNORE
cation attacks against SSH:                                                                               2 2
                                                                                                                     NEWKEYS
                                                                                                          2 3                                          2 2
      We show that the SSH Binary Packet Protocol is                                                      2 4
                                                                                                                     NEWKEYS
                                                                                                                                                       3 2
      not a secure channel because a MitM attacker can
                                                                                                                                            SC1
      delete a chosen number of integrity-protected pack-                                                                                              3 3
                                                                                                                     SC2
      ets from the beginning of the channel in either or                                                  3 4                                          4 3
      both directions without being detected (Figure 2).
                                                                        Figure 2: A novel prefix truncation attack on the BPP. The
Attacker Model. We consider a MitM attacker who can                     server sends SC1 and SC2, but the client only receives SC2.
observe, change, delete, or insert bytes at the TCP layer. We
do not assume that the attacker can break the confidentiality
of the session keys, i.e., the attacker has no information about                                      over a fixed list of handshake messages rather than the
the derived encryption keys, MAC keys, or IV. However, we                                             complete transcript. This gap in authentication allows
do assume that the attacker can determine the length of the                                           an attacker to insert messages into the handshake and
messages to be deleted even if the length field is encrypted.                                         thereby manipulate sequence numbers.
We discuss the practicality of this in Section 4.3. The rogue
session attack presented in Section 6.2 further assumes the               2. SSH does not reset sequence numbers at the beginning
attacker has an account on the same host as the victim.                      of the secure channel. Instead, SSH increases sequence
   As for the connection, we assume that the server is correctly             numbers monotonically, independent of the encryption
authenticated (i.e., the client recognizes the server’s host key)            state. Any manipulation of sequence numbers before the
and that a vulnerable encryption mode has been negotiated.                   secure channel carries over into the channel.
See Table 1 for a list of vulnerable encryption modes.
                                                                        Based on these two key observations, we present a series of
                                                                        novel attacks on SSH that increase in complexity and impact.
Prefix Truncation Attacks. While our attacks on SSH are
novel, the idea of prefix truncation attacks against network
                                                                        Sequence Number Manipulation. We show that an at-
protocols by sequence number manipulation is not. To the
                                                                        tacker can increase the receive counters of the server and the
best of our knowledge, the first and only description of such
                                                                        client by inserting messages into the handshake. Although not
an attack is by Fournet (on behalf of miTLS) in an email to
                                                                        required for any of our attacks, we also show that, for some
the TLS working group in 2015, targeting a draft version of
                                                                        implementations, an attacker can fully control the receive and
TLS 1.3 [18]. Fournet’s attack increases sequence numbers in
                                                                        send counters, setting them to arbitrary values (Section 4.1).
TLS by message fragmentation rather than message injection
and remains theoretical, as “prefix truncations will probably
cause the handshake to fail.” Subsequently, the draft was               A Prefix Truncation Attack on the BPP. An attacker can
modified, and no prefix truncation attacks against the final            use sequence number manipulation to delete a chosen number
version of TLS 1.3 are known. In contrast, we present the first         of packets at the beginning of the secure channel. Neither
real-world, practical prefix truncation attack against a mature,        the client nor the server detects this prefix truncation, conse-
widely used protocol.                                                   quently breaking the channel integrity of SSH (Section 4.2).


Root Cause Analysis. Our results depend on two technical                Extension Negotiation Downgrade Attack. As a practi-
observations about how SSH protects the integrity of the                cal example, we show an attack that uses prefix truncation
handshake and channel:                                                  to break extension negotiation [9], thereby downgrading the
                                                                        security of the connection. The attacked client might mistak-
  1. SSH does not protect the full handshake transcript. Al-            enly believe that the server does not support recent signature
     though server authentication uses a signature to verify            algorithms for user authentication or does not implement cer-
     the integrity of the handshake, the signature is formed            tain countermeasures to attacks (Section 5.2). Specifically,


                                                                    3
the attacker can turn off protection against keystroke timing                to log the victim into an attacker-controlled shell. Both
attacks in the recently released OpenSSH 9.5.                                escalate implementation flaws in AsyncSSH using the
                                                                             prefix truncation attack.
Rogue Extension Attack and Rogue Session Attack. As
                                                                           • An internet-wide scan with up-to-date information on the
another example, we show two attacks on the AsyncSSH
                                                                             distribution of SSH encryption modes and extensions.
client and server. In the first attack, the victim’s extension
info message is replaced with one chosen by the attacker
(Section 6.1). For the second attack, the attacker must have a         Artifacts. Proof-of-concept implementations for our attacks
user account on the same server as the victim. The attacker            and the aggregated results of our internet-wide scan are avail-
injects a malicious user authentication message so that the            able under the Apache-2.0 open-source license. See:
victim logs into a shell controlled by the attacker rather than          https://github.com/RUB-NDS/Terrapin-Artifacts
the victim’s shell, thereby giving the attacker complete control
over the victim’s terminal input (Section 6.2). These attacks
                                                                       Ethics Consideration and Responsible Disclosure. We
combine prefix truncation with implementation flaws in the
                                                                       disclosed our findings to 33 vendors of SSH implementa-
AsyncSSH library.
                                                                       tions, including OpenSSH and AsyncSSH, in October and
                                                                       November 2023, followed by a public disclosure on Decem-
Limitations. Our attacks critically depend on the SSH en-              ber 18th, 2023. As of February 2024, 28 vendors have pub-
cryption mode negotiated between the client and the server.            lished patches implementing a backward-compatible counter-
The attack works best with the AEAD cipher ChaCha20-                   measure proposed by OpenSSH. The general protocol flaw
Poly1305 (added in 2013). The attack also works with any               has been assigned CVE-2023-48795 (CVSSv3 5.9), while
EtM mode (added in 2012), although the success probability             the implementation flaws in AsyncSSH were assigned CVE-
depends on the cipher mode negotiated. CBC-EtM can be ex-              2023-46445 (Rogue Extension Negotiation; CVSSv3 5.9) and
ploited with a significant probability, while the exploitability       CVE-2023-46446 (Rogue Session Attack; CVSSv3 6.8). To
of CTR-EtM is low. On the other hand, CBC-EaM, CTR-                    estimate the adoption rate of the countermeasure, we scanned
EaM, and GCM modes are not affected. See Section 4.4 for a             the IPv4 address space on January 5th, 2024, indicating that
complete analysis.                                                     more than 3.4M servers were patched.
   In an internet-wide scan, we show that despite these lim-              We provide an opt-out option and an email address for
itations, 71.6% of all SSH servers on the internet support             inquiries about our internet-wide scans. Additionally, we
an affected encryption mode, and 63.2% even list it as their           employ a block list to exclude networks that opted out of pre-
preferred choice (Section 7).                                          vious scans. Scan results are solely published in aggregated
                                                                       form, without any information that could identify individual
1.3    Our Contributions                                               servers or networks.

We contribute the following novel results:
                                                                       2    Related Work
   • An analysis of the integrity of SSH channels, where we
     identify two previously unknown flaws in the SSH spec-            Secure Channels. In 2001, Canetti and Krawczyk [12] es-
     ification, namely gaps in the handshake authentication            tablished the first model for secure channels, which only re-
     and the use of sequence numbers across key activation.            quires protection against adversarial insertion of messages.
                                                                       Paterson et al. [39] defined stateful length-hiding authenti-
   • A novel prefix truncation attack on SSH channel integrity,        cated encryption (sLHAE) to model the TLS record layer as a
     where we show that an attacker can manipulate the se-             secure channel. This definition was used in [23, 24] to define
     quence numbers and delete several messages from the               authenticated and confidential channel establishment (ACCE)
     beginning of the secure channel.                                  to analyze the TLS handshake and record layer as a whole.
   • A first security analysis of SSH extension negotiation, in-       Bellare et al. [4] used stateful authenticated decryption to
     cluding a novel downgrade attack that disables extension          define a security notion for SSH that is directed against replay
     negotiation completely. Thus, support for some public             and out-of-order delivery attacks (INT-sfCTXT). Paterson
     key signature algorithms or, with OpenSSH 9.5, protec-            and Watson [40] later refined this work to cover buffered de-
     tion against keystroke timing attacks can be disabled.            cryption (INT-BSF-CTXT). Albrecht et al. [1] further refined
                                                                       and extended this definition to cover ciphertext fragmenta-
   • As a practical demonstration, two novel attacks on                tion attacks more generally (INT-sfCTF). Generalizing the
     AsyncSSH. First, a rogue extension attack, where the at-          work on TLS and SSH, Fishlin et al. [17] defined, among
     tacker can insert a chosen extension negotiation message.         other notions, plaintext integrity for generic data and (atomic)
     Second, a rogue session attack that allows the attacker           message streams (INT-PST, aINT-PTXT).


                                                                   4
   Our attacks show that SSH BPP, when instantiated with               shown due to the attack in [2]. However, they do mention that
ChaCha20-Poly1305, CBC-EtM, or CTR-EtM, does not pro-                  due to a limitation in the design of CryptoVerif, it cannot keep
vide integrity of plaintext or ciphertext (message) streams            mutable internal states such as sequence numbers or counters.
(INT-PST, aINT-PST, INT-sfCTF) as defined in [1, 17].                  In their model, the sequence numbers are passed explicitly
                                                                       as arguments and are, therefore, under the attacker’s control.
                                                                       The authors do not raise the issue of channel integrity. Other
Truncation Attacks. Suffix truncation attacks against web
                                                                       computer-aided proofs of server authentication and secrecy
services using TLS have been demonstrated by Smyth and
                                                                       of the session key in the symbolic or computational model
Pironti in [44]. A prefix truncation attack against a draft
                                                                       can be found in [13, 25], which also do not consider the in-
version of TLS 1.3 was described by Fournet (on behalf of
                                                                       tegrity of the secure channel. For an overview of the field of
miTLS) in an email to the TLS working group in 2015 [18].
                                                                       computer-aided cryptography, see [3].
Fournet’s attack increases TLS sequence numbers by mes-
sage fragmentation rather than injection to avoid breaking
handshake authentication. The attack remained theoretical
as “prefix truncations will probably cause the handshake to            3   Background
fail.” As a countermeasure, the draft was changed back to
reset sequence numbers to 0 when activating keys.                      SSH Handshake (Figure 1). To initiate an SSH connection,
                                                                       both peers exchange a version banner. The Binary Packet Pro-
                                                                       tocol (see below) is used from the third message on but with-
Attacks on SSH. The most severe attack on SSH was pre-                 out encryption and authentication. In the K EX I NIT messages,
sented by Albrecht, Paterson, and Watson [2] in 2009. It               nonces and ordered lists of algorithms are exchanged: One
exploited the encrypted length field, using the length of the          list for key exchange, one for server signatures, and two (one
ciphertext accepted by the server from the network as a de-            per direction) each for encryption, MAC, and compression.
cryption oracle for parts of a ciphertext block. In [40], this         For each list, the negotiated algorithm is the first algorithm in
peculiarity of the BPP was formalized, and in [1] a variant            the client’s list, which is also offered by the server.
of this attack was presented. Other attacks on SSH include                In the K EX D H I NIT and K EX D H R EPLY messages, a finite-
a timing attack on SSH keystrokes by Song, Wagner, and                 field Diffie-Hellman key exchange is performed. SSH also
Tian [45], a theoretical attack on SSH CBC cipher modes by             supports elliptic curves (ECDH) and hybrid schemes with
Wei Dai [14], and a SHA-1 chosen prefix collision attack on            post-quantum cryptography (PQC) as alternatives. The server
the handshake transcript by Bhargavan and Leurent [8]. The             authenticates itself with a digital signature as part of the hand-
weakness of some SSH host keys presented by Heninger et                shake. The signature is computed over the contents of the
al. [20] was caused by a lack of entropy and faulty implemen-          previously exchanged messages in a specified order.
tations and is not an inherent weakness of the protocol.

Formal Proofs for SSH. The SSH handshake was analyzed                  The Exchange Hash: A Partial Handshake Transcript.
by Williams [47] and Bergsma et al. [7]. Bellare et al. [4]            In contrast to TLS, SSH uses only a selection from the hand-
presented a generic security model for SSH BPP, and Paterson           shake transcript for authentication. The hash value computed
and Watson [40] a specific, more detailed one for CTR-EaM.             from this selection is called exchange hash H, defined as
Albrecht et al. [1] included security statements for ChaCha20-
Poly1305, generic Encrypt-then-MAC, and AES-CTR in SSH,                        H = HASH(VC || VS || IC || IS || KS || X || K),
claiming the indistinguishability and integrity of the cipher-
text. Careful analysis of their proofs reveals an essential            where HASH is the hash function of the negotiated key ex-
assumption about SSH sequence numbers that does not hold.              change, VC and VS are the version banners of the client and
In particular, they assume that the sequence counters in the           server, IC and IS are the K EX I NIT messages, KS is the server’s
stateful encryption scheme are initialized to 0 on both sides,         public host key, and K is the shared secret derived from the
which is false for the cipher modes affected by our attack. This       key exchange. The value of X depends on the key exchange
assumption is not apparent from the paper, which omits the             and contains a composition of negotiated parameters (if any)
pseudocode for the encryption schemes, but Hansen gives the            and the ephemeral public keys of the key exchange [33, Sec.
missing parts in [19] (Alg. ssh-ChaCha20-Poly1305-Gen                  8]. Each field includes a length field defined by the encoding.
in Fig. 6.5 and Alg. ssh-fgEtM-Gen in Fig. 6.6 there). We                 Although the exchange hash contains everything that may
note that this assumption is also present in [4] (Fig. 4 there).       influence the negotiation of algorithms or computation of
   Cadé and Blanchet [11] used the formal verification tool            the shared secret, it excludes seemingly ‘unimportant’ mes-
CryptoVerif [10] to prove the security of SSH server authenti-         sages or message parts, such as I GNORE messages and unrec-
cation and the secrecy of the session key in the computational         ognized messages. This authentication gap allows a MitM
model. The secrecy of messages in the channel cannot be                attacker to inject messages into the handshake.


                                                                   5
Sequence Numbers. Each sequence number is stored as a                    OpenSSH in 2013, inspired by a similar proposal for TLS by
4-byte unsigned integer initialized to zero upon connection.             Langley and Chang [26, 27]. Here, two different encryption
After a binary packet has been sent or received, the corre-              keys for the length field and the packet payload are derived,
sponding sequence number Snd or Rcv is incremented by one.               so the length field cannot be used as a decryption oracle for
Sequence numbers are never reset for a connection but roll               the payload. The MAC is computed over the concatenation of
over to 0 after 232 − 1. As sequence numbers are responsible             the two ciphertexts. Internally, the AEAD construction uses
for protecting against replay attacks, rekeying must occur at            the sequence number as a nonce for each packet.
least once every 232 packets [37, Sec. 6.1].                                We note that the SSH specification says that the length
   We illustrate the use of sequence numbers in Figure 1: Af-            field is encrypted [33, Sec. 6] and that the sequence number
ter the banner exchange, the counters Snd and Rcv are initial-           is used for integrity checks [33, Sec. 6.4]. This is only true
ized with (0, 0) on both sides. During algorithm negotiation             for CBC-EaM, CTR-EatM, and ChaCha20-Poly1305. The
and key exchange, sequence numbers are increased but not                 modes CBC-EtM, CTR-EtM, and GCM do not encrypt the
used in any MAC computation or verification. Only after keys             length field, and GCM also does not use the sequence number.
are activated the secure channel is established, and sequence
numbers are used for MAC computation and verification. For
each BPP packet, the sequence numbers in bold must match
                                                                         4     Breaking SSH Channel Integrity
at both peers; otherwise, the BPP packet is rejected.
                                                                         In this section, we present a novel prefix truncation attack on
                                                                         SSH. The basic idea is that the attacker injects messages into
SSH Binary Packet Protocol. The BPP is used to encrypt                   the handshake to increase the implicit sequence number in
and authenticate messages. First, a message is prefixed by               one of the peers and then deletes a corresponding number of
a 4-byte message length and a 1-byte padding length. Then,               messages to that peer at the beginning of the secure channel.
at least 4 bytes of padding are added to the message so that             Two key insights about the SSH protocol enable this attack:
the total length is a multiple of the block size or 8, whatever
is larger. On the secure channel, the packet is encrypted by
                                                                         SSH Does Not Protect the Full Handshake Transcript.
the cipher mode, and a MAC is added. The details depend on
                                                                         As detailed in Section 3, the exchange hash signed by the
the authenticated encryption scheme, which uses an implicit
                                                                         server during the handshake only authenticates some parts of
initialization vector IVKDF derived from the session key.
                                                                         the handshake transcript, while other parts are left unauthen-
   CBC-EaM [33] (Figure 3a) is part of the original SSH spec-
                                                                         ticated. This allows an attacker to inject messages into the
ification. The MAC is computed over the implicit sequence
                                                                         handshake, which cannot affect the key exchange but does
number and the packet plaintext. The IV of the first packet is
                                                                         affect the implicit sequence numbers of the peers.
IVKDF , and IV chaining is used (i.e., the IV of packet i is the
last ciphertext block of packet i − 1).
   CBC-EtM [36] (Figure 3b) was added to OpenSSH in 2012.                SSH Does Not Reset Sequence Numbers at the Beginning
Here, the packet length is not encrypted to allow checking               of the Secure Channel. In SSH, sequence numbers are
the MAC before decryption. The MAC is computed over                      only incremented and never reset to 0, even when the en-
the sequence number, the unencrypted packet length, and the              cryption key changes. This allows an attacker to manipulate
ciphertext. The IVs are handled as with CBC-EaM.                         the sequence number counters in the secure channel before
   CTR [37] mode was proposed by Bellare, Kohno, and                     encryption and authentication keys are activated.
Namprempre [4] as a countermeasure to attacks on CBC with
IV chaining. IVKDF is used as the initial counter value and              Comparison to Other Protocols. In IPsec/IKE, only a por-
incremented after encrypting a plaintext block. CTR can be               tion of the handshake transcript is signed, but unlike SSH, se-
used with EaM or EtM, with identical implications for the                quence numbers are reset to 0 when encryption and MAC keys
length field and MAC computation as above.                               are activated. In TLS, F INISHED messages are exchanged
   GCM [22] (Figure 3c) mode was specified by the NSA for                at the beginning of the secure channel to verify the integrity
Suite B-compliant SSH implementations [21]. Here, cipher-                of the complete handshake transcript, and sequence numbers
text integrity is part of the cipher mode. The length field is not       are reset to 0 after installing new keys. The Noise Protocol
encrypted (solely authenticated) to allow verification of the            Framework fully secures the handshake transcript and uses a
authentication tag before returning any plaintext. Internally,           nonce as a sequence counter that is initialized to 0 after the
GCM uses a 12-byte nonce that is initialized to IVKDF . The              handshake.
nonce is split into a 4-byte fixed value and an 8-byte invo-
cation counter that is incremented by one for each message.
                                                                         4.1    Sequence Number Manipulation
The sequence number is not used but is always offset by a
constant from the invocation counter.                                    In this section, we first show how a MitM attacker can ar-
   ChaCha20-Poly1305 [34] (Figure 3d) was added to                       bitrarily increase the receive sequence numbers C.Rcv and


                                                                     6
   4 Bytes      4 Bytes       1 Byte                                    ≥ 4 Bytes                        4 Bytes             4 Bytes      1 Byte                                                   ≥ 4 Bytes
  Sequence                   Padding                                                                     Sequence                         Padding
             Packet Length                          Payload             Padding                                           Packet Length                               Payload                      Padding
  Number                     Length                                                                      Number                           Length


                                       ENCRYPT                                                                                                      ENCRYPT


                                                              MAC                                                                                                               MAC


                                          Encrypted Packet                                  MAC                           Packet Length                             Encrypted Packet                           MAC



(a) Encrypt-and-MAC [33] (diagram based on [2, Fig. 1]). The                                          (b) Encrypt-then-MAC [36, Sec. 1.5] (diagram based on [41,
packet length is encrypted as part of the packet’s plaintext. The                                     Slide 37]). The packet length is not encrypted but authenticated as
sequence number is part of the input used to compute the MAC.                                         part of the MAC’s input. The sequence number is used similarly to
                                                                                                      Encrypt-and-MAC.
                                                                                                         4 Bytes             4 Bytes      1 Byte                                                   ≥ 4 Bytes
                                                                                                         Sequence                         Padding
   4 Bytes      4 Bytes       1 Byte                                    ≥ 4 Bytes                                         Packet Length                               Payload                      Padding
                                                                                                         Number                           Length
  Sequence                   Padding
             Packet Length                          Payload             Padding
  Number                     Length                                                                              Nonce                    Counter
                                                                                                                          ChaCha20 𝑲𝟏               [0]64
                                                              4 Bytes             8 Bytes                                                                   Nonce                      Counter
                                                                                                                                                                     ChaCha20 𝑲𝟐                 [1]64
                             AAD        AEAD      Nonce
                                                              Fixed        Invocation Counter
                                       ENCRYPT
                                                                                                                            Encrypted
                                                                                                                                                                    Encrypted Packet                           Tag
                                                                                                                          Packet Length

             Packet Length                       Encrypted Packet                           Tag                               0256
                                                                                                                 Nonce
                                                                                                        [0]64             ChaCha20 𝑲𝟐                                  Poly1305
                                                                                                                Counter                                       Key



(c) Galois/Counter Mode [22]. The packet length is not encrypted                                      (d) ChaCha20-Poly1305 [34] (diagram based on [41, Slide 39]). The
but authenticated as additional authenticated data (AAD). The se-                                     packet length is encrypted using a different key and authenticated
quence number is not used directly but is replaced with an invocation                                 as part of the Poly1305 input. The sequence number is encoded as
counter of constant offset.                                                                           an unsigned 64-bit integer to match the nonce length of ChaCha20.

                                       Figure 3: Commonly used authenticated encryption schemes in the BPP of SSH.


S.Rcv in the client and the server during the handshake. This                                         messages with unrecognized message IDs [38, Sec. 11.4]. Let
will be the basis for our prefix truncation attack and its ap-                                        U NKNOWN be a message with an unrecognized message ID.
plications, allowing the attacker to compensate for messages
deleted from the secure channel.                                                                      Technique RcvDecrease (Figure 4b). A MitM attacker can
                                                                                                      decrease C.Rcv (resp. S.Rcv) by N while not changing any
Technique RcvIncrease (Figure 4a). A MitM attacker can                                                other sequence number by sending 232 − N I GNORE messages
increase C.Rcv (resp. S.Rcv) by N while not changing any                                              to the client (resp. server).
other sequence number by sending N I GNORE messages to                                                   A single I GNORE message is only 5 bytes, so it fits into a
the client (resp. server).                                                                            single block even for a 128-bit block cipher. Sending 232 − N
   The correctness is evident from the fact that the SSH stan-                                        such messages transfers ≈ 232 · 16B ≈ 69GB of data. Con-
dard requires for I GNORE that “All implementations MUST                                              sequently, this technique can fail on implementations with
understand (and ignore) this message at any time.” [33, Sec.                                          timeouts or restrictions to the amount of data or the number
11.2]. The intended purpose of this message is to protect                                             of messages transferred during the handshake.
against traffic analysis, so it is considered a security feature,
although there is no benefit from it during the handshake                                             Technique SndIncrease (Figure 4c) and SndDecrease (Fig-
phase. We note that the attacker may also use any other mes-                                          ure 4d). A MitM can increase C.Snd (resp. S.Snd) by N
sage type that does not generate a response.                                                          while not changing any other sequence number by sending N
                                                                                                      U NKNOWN and 232 − N I GNORE messages to the client (resp.
Other Modifications of Sequence Numbers. In addition,                                                 server) and deleting all generated U NIMPLEMENTED mes-
we found that an attacker can set the sequence numbers to ar-                                         sages. Conversely, a MitM can decrease C.Snd (resp. S.Snd)
bitrary values by using the rollover after 232 messages during                                        by N while not changing any other sequence number by send-
the handshake. These advanced techniques require that the im-                                         ing 232 − N U NKNOWN and N I GNORE messages to the client
plementation allows handshakes with many messages, a large                                            (resp. server) and deleting all generated U NIMPLEMENTED
amount of data, and a long operating time. We also require a                                          messages.
message that generates a response message but is otherwise                                               Here, the total data transfer required is ≈ 69GB for SndIn-
ignored. Conveniently, the SSH standard requires this for all                                         crease and twice as much (≈ 138GB) for SndDecrease. Again,


                                                                                                  7
  Snd   Rcv                     Snd      Rcv                            We first analyze this attack with regard to handshake au-
              IGNORE                             IGNORE
   𝑥     𝑦                       𝑥       𝑦                           thentication and sequence numbers. As the key exchange does
                                                 IGNORE
   𝑥    𝑦+1                       𝑥     𝑦+1                          not protect the handshake transcript from inserting I GNORE
                                                     […]
                                                 IGNORE
                                                                     messages (Section 3), handshake authentication is not broken.
                                 𝑥     232 − 1
                                                 IGNORE
                                                                     Before the first step, we have C.Rcv = S.Snd. After the first
                                 𝑥       0
                                                     […]
                                                                     step, we have C.Rcv = S.Snd + 1, but this manipulation is
                                  𝑥     𝑦−2
                                                 IGNORE              not detected during the handshake. After the second step, we
                                 𝑥      𝑦−1
                                                                     have C.Rcv = S.Snd, and sequence numbers are back in sync.
                                                                        It remains to be shown that the attacker can delete the mes-
         (a) RcvIncrease                  (b) RcvDecrease            sage from the channel, which requires knowledge about the
  Snd   Rcv                     Snd     Rcv                          message’s length, and that its deletion does not affect the
              UNKNOWN                            UNKNOWN
   𝑥     𝑦                       𝑥       𝑦                           MAC verification and decryption output for the following
              UNIMPLEMENTED                      UNIMPLEMENTED
   𝑥    𝑦+1                      𝑥      𝑦+1                          messages. Both aspects require careful analysis with respect
              IGNORE                                 […]
 𝑥+1 𝑦+1                                                             to the used encryption mode, which will be given in Sec-
                                                 UNKNOWN
                                𝑥 −2   𝑦−2
                 […]                                                 tion 4.3 and Section 4.4. Here, we conclude by describing a
              IGNORE                             UNIMPLEMENTED
 𝑥+1 𝑦−1                        𝑥−2 𝑦−1                              straightforward generalization of the single message attack.
                                                 IGNORE
 𝑥+1     𝑦                      𝑥−1 𝑦−1

                                𝑥−1      𝑦
                                                                     (NS , NC )-Prefix Truncation Attack. In a single attack, the
                                                                     attacker can generally delete an arbitrary number of NS initial
         (c) SndIncrease                  (d) SndDecrease            messages sent from the server and NC initial messages sent
                                                                     from the client. This is straightforward: Instead of inserting
Figure 4: Techniques for sequence number manipulation as a
                                                                     one I GNORE message to the client before N EW K EYS, the
MitM in the SSH protocol. All techniques can target either
                                                                     attacker inserts NS such messages to the client and NC to the
client or server before the initial handshake concludes. The
                                                                     server. Consequently, instead of deleting the first message
MitM deletes all generated U NIMPLEMENTED messages.
                                                                     from the server, the attacker deletes NS initial messages from
                                                                     the server and NC initial messages from the client.
these techniques may fail on implementations that have time-            Note that the single message attack above is the specific
outs or restrict the amount of data or number of messages            case of a (1, 0)-prefix truncation attack.
exchanged during the handshake.
                                                                     4.3 Determining the Byte-Length of Messages
Evaluation. We verified all techniques successfully against
                                                                     To successfully delete packets from the secure channel, the
PuTTY 0.79. Additionally, our experiments show that
                                                                     attacker has to know their length. This is inherently true for en-
OpenSSH 9.5p1 recognizes a rollover of sequence numbers
                                                                     cryption modes that do not encrypt the packet length field (any
and terminates the connection, thus not affected by any tech-
                                                                     EtM mode, GCM). In the case of an encryption mode with
nique but RcvIncrease. AsyncSSH 2.13.2 and libssh 0.10.5
                                                                     an encrypted packet length field (any EaM mode, ChaCha20-
allow for RcvIncrease but terminate the connection due to
                                                                     Poly1305), the attacker may employ different strategies to
handshake timeouts before any advanced technique concludes.
                                                                     determine the packet’s length. One such strategy is to uti-
Dropbear 2022.83 disconnects on U NKNOWN messages in-
                                                                     lize knowledge about the plaintext if the length of the first
stead of responding with U NIMPLEMENTED but allows Rcv
                                                                     few messages inside the secure channel is either fixed (for
to roll over, therefore being affected by RcvIncrease and
                                                                     example, S ERVICE ACCEPT) or can be measured within a sin-
RcvDecrease only.
                                                                     gle connection ahead of time (for example, E XT I NFO). This
                                                                     approach was used for all attacks described here. More ad-
4.2     Prefix Truncation Attack on the BPP                          vanced strategies may exploit TCP segment sizes and timings,
                                                                     as well as the message order of the SSH protocol. For exam-
Single Message Prefix Truncation Attack. We assume the
                                                                     ple, an attacker may delay all encrypted traffic by the server
attacker wants to delete the first message SC1 sent from the
                                                                     until after the client’s S ERVICE R EQUEST message has been
server (Figure 2). The attack takes two steps:
                                                                     processed to determine the length of the E XT I NFO message.
 1. The attacker uses the RcvIncrease technique to increase          Here, we assume that the attacker always knows the lengths.
    C.Rcv by one, e.g., by injecting an I GNORE message to
    the client before N EW K EYS.                                    4.4    Analysis of Encryption Modes
 2. The attacker deletes the first message SC1 sent by the           In this section, we analyze which encryption modes our at-
    server.                                                          tacks affect and if they can be exploited in a real-world sce-


                                                                 8
    Authenticated Encryption Mode        Specification     Enc. State         Dec. State      Affected    Exploitable          Ref.
                               CBC            [33]         (IV , Snd)         (IV , Rcv)         ✗              #         Section 4.4.1
        Encrypt-and-MAC
                               CTR          [33, 37]       (ctr, Snd)         (ctr, Rcv)         ✗              #         Section 4.4.1
                               CBC          [33, 36]       (IV , Snd)         (IV , Rcv)         ✓              #
                                                                                                                G         Section 4.4.3
     Encrypt-then-MAC
                               CTR          [36, 37]       (ctr, Snd)         (ctr, Rcv)         ✓              #
                                                                                                                G         Section 4.4.3
             GCM                              [22]         ctrInvocation      ctrInvocation      ✗              #         Section 4.4.1
    ChaCha20-Poly1305                         [34]            Snd                  Rcv           ✓                        Section 4.4.2

Table 1: Authenticated encryption modes, corresponding specification documents, and their exposure to prefix truncation in the
BPP of SSH. The initial value of state variables printed in bold purple can be chosen by the attacker, cf. Section 4.1. Full control
of either state enables perfect prefix truncation ( , ChaCha20-Poly1305). Partial control may lead to limited exploitability,
depending on the inner workings of the authenticated encryption mode (G    #, Encrypt-then-MAC).


nario (see Table 1). An encryption mode is affected if, after                Note that the fault is not with ChaCha20-Poly1305 as an
prefix truncation, all following packets on the secure channel             AEAD encryption scheme but with its integration into the
are decrypted, i.e., an AEAD mode does not generate the                    SSH secure channel construction.
distinguished symbol I NVALID or a composed mode success-
fully verifies the MAC. Note that we allow decryption to a
                                                                           4.4.3   Affected With Limited Exploitability
different plaintext for probabilistic attacks. To capture this,
we define an encryption mode as exploitable for an attack                  CTR-EtM. With CTR-EtM, the MAC is computed over the
if the message stream after decryption is well-formed and                  unencrypted length, the sequence number, and the ciphertext.
supports that attack. If the attack’s success probability is less          So, removing some packets from the beginning of the channel
than 1, we say the attack has limited exploitability.                      does not cause a MAC failure, and cryptographically, the at-
                                                                           tack succeeds. However, CTR uses a block counter initialized
4.4.1    Not Affected                                                      to IVKDF , which increments after each block. After prefix
                                                                           truncation, the key stream is desynchronized, so all following
GCM. GCM [22] mode does not use the implicit sequence
                                                                           ciphertexts are decrypted as pseudorandom packets. Each
number. Instead, it uses an invocation counter, initialized
                                                                           corrupted packet has a significant probability of causing a
to IVKDF , and incremented after each message. The authors
                                                                           critical failure, eventually stopping our attack.
justify this by stating that the resulting nonce is always a
fixed offset from the sequence number. By deviating from the
SSH standard, GCM stops our attack, as the attacker cannot                 Remark: Decryption Oracle for CTR-EtM Using Prefix
manipulate the invocation counter during the handshake.                    Truncation. For CTR-EtM, prefix truncation of k blocks
                                                                           (which exactly contain one or more messages) provides a
CBC-EaM and CTR-EaM. CBC uses IV chaining, and                             very limited decryption oracle on the ciphertext c1 , . . . , ck
CTR uses a key stream. When the attacker deletes any prefix                where ci := Enc(IVKDF + i) ⊕ pi , 1 ≤ i ≤ k. After deleting
of the ciphertext in either mode, the first ciphertext block               the first k blocks, MAC verification for the following message
received will be decrypted as pseudorandom. Because EaM                    of length l blocks will succeed because the length, sequence
computes the MAC over the plaintext, MAC verification will                 number, and ciphertext are correct. The blocks ck+1 , . . . , ck+l
fail with a probability close to 1, thwarting our attack.                  will be decrypted as p′j := Enc(IVKDF + j) ⊕ ck+ j , 1 ≤ j ≤ l,
                                                                           and processed as a pseudorandom SSH message SC1’. Due
                                                                           to format oracle side channels in SSH at the BPP layer, e.g.,
4.4.2    Affected And Perfectly Exploitable
                                                                           the padding length, but also at the protocol layer, e.g., if a
ChaCha20-Poly1305. ChaCha20-Poly1305 [34] directly                         message is ignored or triggers a response, the attacker can get
uses the sequence number in its internal key stream derivation,            some information about the bits in p′j . This reveals informa-
which makes it vulnerable to our prefix truncation attack.                 tion about the first l key stream blocks, and thus also about
All messages following the truncated prefix are decrypted                  p1 , . . . , pl , potentially leaking confidential information like
to their original plaintext because the integrity check of the             passwords in user authentication. If processing SC1’ does not
AEAD cipher is done over the ciphertext and the sequence                   cause a critical failure, the attack can even continue, reveal-
number, which the attacker has manipulated to match. Under                 ing more about the following key stream and, thus, plaintext.
the assumption that the attacker can correctly guess the packet            Exploiting this requires a careful study of format oracles in
length, the prefix truncation attack always succeeds.                      SSH, which is outside the scope of this work.


                                                                    9
CBC-EtM. With CBC-EtM, the MAC is computed from                                    which is mandatory to start user authentication. The encrypted
the unencrypted length, the sequence number, and the cipher-                       part of the packet looks like this, where p is the padding
text. The IV is not required because IVKDF is implicit, and all                    length, m is the message ID, and n is the service name length:
other IVs are authenticated before use. Consequently, prefix                        p m           n                     Service Name
truncation does not cause a MAC failure, and cryptographi-
cally, the attack succeeds. Nevertheless, we need to consider                      0e 06 00 00 00 0c s s h - u s e                       r a u
the impact that IV chaining has on the immediately following                        t h                       Random Padding
packet to see if this attack is practically exploitable.
   Recall that the decryption of the first block is p1 :=                          The probability that the first block decrypts exactly as shown
Dec(c1 ) ⊕ IVKDF , and for block i, it is pi := Dec(ci ) ⊕ ci−1 .                  is only 2−128 for a 128-bit block cipher. However, for some
We assume the attacker uses prefix truncation to remove                            clients, the service name string is optional. These clients
blocks c1 , . . . , ck . The following block ck+1 will now be de-                  accept a 1-byte message with p = 30 (0x1E) and m = 6 as
crypted as p′1 := Dec(ck+1 ) ⊕ IVKDF . We are interested in                        marginally corrupt, which has a success probability of 2−16 ,
how SSH implementations process the resulting pseudoran-                           independent of the block size.
dom block p′1 as the first block in the decrypted packet. In-                         Although S ERVICE ACCEPT may be a lucky case (for the
tuitively, it should result in a corrupted packet that causes a                    attacker), there are structural reasons for this result: First,
critical failure.2                                                                 SSH messages are often short and can be smaller than a sin-
   Surprisingly, there is a significant probability that the attack                gle block. Second, the padding is random and cannot be
can continue, although it is highly implementation-dependent.                      verified. Third, some messages have redundant fields that
For a corrupted packet, there are four possible outcomes:                          implementations ignore (e.g., the service name above).
                                                                                      We experimentally verified that OpenSSH, Dropbear,
  1. Critically Corrupt: If corruption is detected at the BPP                      PuTTY, and libssh allow empty S ERVICE ACCEPT messages
     or application level, e.g., if a length field exceeds the                     from the server, enabling this attack. At the same time, Async-
     packet length, the connection should be closed.                               SSH is strict by requiring the correct service name.

  2. Marginally Corrupt: If the packet happens to be similar
                                                                                   Scenario 2: CBC-EtM Prefix Truncation Attack On More
     enough to the original, e.g., if the corruption is limited
                                                                                   Than One Message. In this scenario, we assume the at-
     to optional fields, it should be processed without error
                                                                                   tacker wants to remove the first N > 1 messages and preserve
     and have the same effect as the original would have had.
                                                                                   all the following messages perfectly. Then, the attacker can
  3. Evasively Corrupt: If the packet is well-formed (i.e.,                        use prefix truncation to delete the first N − 1 messages and
     has valid padding length) but has an unrecognized mes-                        take a bet on the N-th message to be evasively corrupt.
     sage ID, an U NIMPLEMENTED response must be sent,                                Let ℓ be the length of the ciphertext of the N-th message,
     and the connection continues normally [33, Sec. 11.4].                        with padding length p, message ID m, and random padding.
                                                                                   The attack succeeds regardless of the content of the corrupted
  4. Any other case not covered above, in particular, recog-                       packet as long as it is well-formed and unrecognized: A
     nized messages different from the original.                                   packet is well-formed if 4 ≤ p ≤ ℓ − 2 (accounting for the
                                                                                   padding length and message ID). A packet is unrecognized if
Clearly, the first outcome stops any attack from going for-                        m is a message ID not known by the implementation.
ward. However, the second, third, and fourth outcomes may                             Because the message is well-formed, it is not rejected at
be beneficial for the attacker. We will now present two in-                        the BPP layer. Furthermore, because the message is unrec-
structive scenarios for outcomes two and three, and estimate                       ognized, the peer must respond with U NIMPLEMENTED and
the success probability of an attack relying on that outcome.                      otherwise ignore it [33, Sec. 11.4], so our attack succeeds.
Later, we will verify these estimates experimentally.                                 The probability that a packet is well-formed depends on ℓ.
                                                                                   The padding length is between 4 and 255, and ℓ is a multiple
Scenario 1: CBC-EtM Prefix Truncation Of a Single Mes-                             of max(8, block size), so the number of valid padding length
sage, Second Message Has Format Flexibility. In this                               values is min(252, ℓ − 5) out of 28 .
scenario, the attacker wants to remove the first message, and                         The probability that a packet is unrecognized depends on
the second (corrupted) message needs to be functionally pre-                       the size of the set U of unrecognized message IDs in the imple-
served but has some format flexibility. For example, the                           mentation. The attack requires at least one unknown message
second message might be S ERVICE ACCEPT (see Section 5.2),                         ID. Through source code review, we identified 43 IDs that
    2 Similarly to CTR-EtM, any format oracle side channel for p′ reveals a
                                                                                   are in active use, so we estimate up to 213 unknown message
                                                                1                  IDs out of 28 .
relationship between IVKDF and pk+1 via IVKDF ⊕ pk+1 = ck ⊕ p′1 , which is
a marginal information leak for the (secret) IV given information on pk+1 ,           In total, we estimate a success probability of min(252, ℓ −
and vice versa. Again, we do not explore this further here.                        5) · |U| · 2−16 . Assuming a block size of at least 128-bit (i.e.,


                                                                              10
ℓ ≥ 16), we estimate that the success probability of this at-               RFC 8308 defines an initial set of four protocol extensions,
tack is between 11 · 2−16 ≈ 0.0002 (ℓmin = 16, |Umin | = 1)              and vendors have proposed and implemented additional ex-
and 252 · 213 · 2−16 ≈ 0.8190 (ℓmax ≥ 252, |Umax | = 213) for            tensions. We detail those relevant to our attacks here.3
vulnerable implementations. Our experiments show success                    server-sig-algs [9] is a server-side extension that in-
probabilities from 0.0003–0.8383, in good agreement with                 forms the client about all supported signature algorithms when
our analysis (Section 5.2). Increasing the block size increases          using a public key during client authentication.
the lower bound, while the upper bound stays the same.                      publickey-hostbound@openssh.com [35,36] is a server-
                                                                         side extension to advertise support for host-bound public key
                                                                         authentication, which deviates from public key authentication
5     Breaking SSH Extension Negotiation
                                                                         by also covering the server’s host key. This allows the enforce-
While the fact that BPP does not implement a secure channel              ment of per-key restrictions when generating the signature
is troublesome enough, exploiting this vulnerability requires            outside the SSH client (i.e., when using SSH Agent).
an analysis of the SSH protocol after the handshake, i.e., the              ping@openssh.com [36] is a server-side extension to ad-
SSH authentication protocol.                                             vertise support for a transport-level ping message similar to
   As our attack achieves prefix truncation, it is natural to            the Heartbeat extension in TLS [46].
ask which SSH messages can occur at the beginning of a
secure channel. Historically, the first messages exchanged               5.2     Extension Downgrade Attack
are S ERVICE R EQUEST and S ERVICE ACCEPT. Removing
either causes the connection to go stale, as the client will             We now show how the prefix truncation attack can be applied
not begin the user authentication. Then, our attack, while               to delete the E XT I NFO message sent by the client, server,
cryptographically successful, fails at the application layer.            or both parties without either noticing. Our attack differs
   However, the SSH Extension Negotiation mechanism [9]                  depending on the encryption mode. For ChaCha20-Poly1305,
introduces a new message, E XT I NFO, which can occur im-                we can use the basic attack strategy. For CBC-EtM, we show
mediately after N EW K EYS as the first message on the secure            two strategies to generate additional messages in the secure
channel. Some of the extensions that can be negotiated are               channel so that the attacker can use the “evasively corrupt”
security-relevant, providing an attack surface for our prefix            outcome of Scenario 2 in Section 4.4.3.
truncation attack and raising its impact.
   In this section, we will first describe SSH Extension Nego-           Impact. Successfully performing the extension downgrade
tiation and then demonstrate how an attacker can downgrade               can directly impact the security level of the connection. Most
the security of a connection by removing the E XT I NFO mes-             notably, the recently introduced keystroke timing countermea-
sage from the secure channel in a prefix truncation attack.              sures by OpenSSH 9.5 will remain disabled when the server
                                                                         has not sent ping@openssh.com. Furthermore, stripping an
5.1    SSH Extension Negotiation                                         E XT I NFO containing the server-sig-algs extension can
                                                                         lead to a signature downgrade during client authentication, as
Even though the original SSH RFCs were designed with ex-                 the client has to resort to trial-and-error instead.
tensibility in mind, they do not provide any mechanism to
negotiate protocol extensions securely. RFC 8308 [9] closes
this gap. The RFC describes a signaling mechanism enabling               Extension Downgrade for ChaCha20-Poly1305. The
extension negotiation, the extension negotiation mechanism               downgrade attack for ChaCha20-Poly1305 against the client
itself, and a set of initially defined extensions.                       is depicted in Figure 5a. It is identical to the single message
   Support for extension negotiation is signaled as part of the          prefix truncation attack from Section 4.2, with E XT I NFO now
K EX I NIT message. The structure of the message is not altered,         taking the place of SC1 in Figure 2. If the attack should be
and the reserved field is not used to avoid compatibility issues.        directed against the server instead, a (0, 1)-prefix truncation
Instead, each peer may include an indicator name within                  attack should be performed. This allows an attacker to delete
the list of key exchange algorithms. The indicator name                  any E XT I NFO sent immediately after N EW K EYS.
differs depending on the role of the peer (ext-info-c vs.                   While the server may send a second E XT I NFO just before
ext-info-s) to avoid accidental negotiation.                             signaling successful client authentication, stripping the E XT-
   Whenever a peer signals support for extension negotiation,            I NFO message sent after N EW K EYS renders most publicly
the other side may send an E XT I NFO message as the first               specified extensions unusable. This is because they are ei-
message after N EW K EYS. Additionally, the server can send a            ther scoped to the authentication protocol, sent by the client
second E XT I NFO later to authenticated clients to avoid dis-           only, or must be sent by both parties to take effect. Solely
closing extension support to unauthenticated clients. Each                   3We excluded the following extensions because we consider them unre-
E XT I NFO message can contain several extension entries. Ne-            lated to our attacks: no-flow-control, delay-compression, elevation,
gotiation requirements are defined on a per-extension level.             global-requests-ok, ext-auth-info


                                                                    11
                            Client                    MitM                 Server                                       Client                 MitM                 Server
                            Snd Rcv                                        Snd Rcv                                      Snd Rcv                                     Snd Rcv
                                                         […]                                                                                       […]
                                          KEXDHINIT: gx                                                                            KEXDHINIT: gx
                             1 1                                            1 1                                          1 1                                         1 1
                                          KEXDHREPLY: gy , pkS , sig                                                               KEXDHREPLY: gy , pkS , sig
                             2 1                                            1 2                                          2 1                                         1 2
                                          IGNORE                                                                                   UNKNOWN
                             2 2                                                                                         2 2




   Binary Packet Protocol
                                          NEWKEYS                                                                                 UNIMPLEMENTED
                             2 3                                            2 2                                          2 3
                                          NEWKEYS                                                                                  NEWKEYS




                                                                                               Binary Packet Protocol
                             2 4                                            3 2                                          3 3                                         2 2
                                                                                                                                                          UNKNOWN
                                                                 EXTINFO                                                                                             3 2
                                                                            3 3                                                    NEWKEYS
                                          SERVICEREQUEST: ssh-userauth                                                   3 4                                         3 3
                             3 4                                            4 3
                                          SERVICEACCEPT                                                                                                   EXTINFO
                             4 4                                            4 4                                                                                       3 4
                                          SSH Authentication Protocol                                                              UNIMPLEMENTED
                                                                                                                         4 4                                         4 4
                                          SSH Connection Protocol                                                                  SERVICEREQUEST: ssh-userauth
                                                                                                                         4 5                                          5 4
                                                                                                                                   SERVICEACCEPT
                                                                                                                         5 5                                         5 5
                                                                                                                                   SSH Authentication Protocol
                                                                                                                                   SSH Connection Protocol



(a) Extension Downgrade Attack for ChaCha20-Poly1305: The                                 (b) Extension Downgrade Attack for CBC-EtM: The MitM injects
MitM injects an I GNORE message before the handshake concludes.                           U NKNOWN before the N EW K EYS is sent by the client. As the
The change in sequence numbers allows the MitM to strip the                               server already sent N EW K EYS, the provoked U NIMPLEMENTED
E XT I NFO from within the secure channel.                                                message will be sent within the secure channel after E XT I NFO. The
                                                                                          corrupted U NIMPLEMENTED message has a significant probability
                                                                                          of being ignored (see Scenario 2 in Section 4.4.3).

                                      Figure 5: Variants of the extension downgrade attack for ChaCha20-Poly1305 and CBC-EtM.


the ping@openssh.com extension may be sent in the second                                  To make use of this, the attacker replaces the U NKNOWN mes-
E XT I NFO to enable keystroke timing countermeasures inside                              sage sent to the server with a P ING message containing at least
the connection protocol. However, OpenSSH 9.5 does not                                    255 bytes of payload. As per specification, the server will
implement any facility to send a second extension negotiation                             reflect this data in the P ONG response. This yields ℓ ≥ 264,
message. As shown in Section 7, extensions scoped to the                                  maxing out the probability of the packet being well-formed.
authentication protocol are the most common among SSH                                     Consequently, the upper estimate for the success probability
servers on the internet by a significant margin.                                          is now 252 · 213 · 2−16 ≈ 0.8190.

Extension Downgrade for CBC-EtM. In Figure 5b, we                                         Evaluation. We successfully evaluated the attack in
show how the attack can also work with CBC-EtM. Sup-                                      10,000 trials on ChaCha20-Poly1305 and CBC-EtM against
pose an attacker injects an U NKNOWN message to the server                                OpenSSH 9.5p1 and PuTTY 0.79 clients, connecting to
after the server sends N EW K EYS and E XT I NFO but before                               OpenSSH 9.4p1 (U NKNOWN only) and 9.5p1. For CBC-
the client’s N EW K EYS message (and also injects U NKNOWN                                EtM, our success rate in practice was 0.0003 (OpenSSH)
to the client to realign sequence numbers). In that case, the                             resp. 0.0300 (PuTTY), improved to 0.0074 (OpenSSH) resp.
server sends the response U NIMPLEMENTED as the second                                    0.8383 (PuTTY) when sending P ING instead of U NKNOWN.
message in the secure channel immediately after the E XT I NFO
message. The attacker now wants to remove two messages
from the channel and can benefit from the “evasively corrupt”                             6            Message Injection Attacks on AsyncSSH
in Scenario 2 in Section 4.4.3. The attacker removes E XT-
                                                                                          Going beyond the SSH specifications, we now demonstrate
I NFO from the secure channel, which causes the decryption
                                                                                          how prefix truncation attacks can also be used to exploit
of the first block of U NIMPLEMENTED to become pseudo-
                                                                                          implementation flaws. Specifically, we target AsyncSSH,4 an
random. Because U NIMPLEMENTED messages are relatively
                                                                                          SSH implementation for Python with an estimated 60k daily
small (ℓ = 16 for AES), the upper estimate for the success
                                                                                          downloads.5 We present two attacks that exploit weaknesses
probability is only 11 · 213 · 2−16 ≈ 0.0358.
   However, the success probability can be increased signifi-                                 4 https://github.com/ronf/asyncssh

cantly by exploiting the new ping extension in OpenSSH 9.5.                                   5 https://pypistats.org/packages/asyncssh




                                                                                     12
                             Client                  MitM                        Server                                      Client                MitM                           Server
                             Snd Rcv                                             Snd Rcv                                     Snd Rcv                                              Snd Rcv
                                                      […]                                                                                             […]
                                       KEXDHINIT: gx                                                                                   KEXDHINIT: gx
                              1 1                                                 1 1                                         1 1                                                  1 1
                                       KEXDHREPLY:    gy , pk   S , sig                                                                KEXDHREPLY: gy , pkS , sig
                              2 1                                                 1 2                                         2 1                                                  1 2
                                       EXTINFOMITM                                                                                                          USERAUTHREQUESTMITM
                              2 2                                                                                                                                                  2 2




   Binary Packet Protocol
                                       NEWKEYS                                                                                         NEWKEYS
                              2 3                                                 2 2                                         2 2                                                  2 3




                                                                                                   Binary Packet Protocol
                                       NEWKEYS                                                                                         NEWKEYS
                              2 4                                                 3 2                                         2 3                                                  3 3
                                                                 EXTINFOSERVER                                                         EXTINFO
                                                                                  3 3                                         3 3
                                       SERVICEREQUEST: ssh-userauth                                                                    SERVICEREQUEST: ssh-userauth
                              3 4                                                 4 3                                         4 3                                                  3 4
                                       SERVICEACCEPT                                                                                   SERVICEACCEPT
                              4 4                                                 4 4                                         5 3                                                  3 5
                                                                                                                                       USERAUTHREQUESTCLIENT
                                       SSH Authentication Protocol                                                            5 4
                                       SSH Connection Protocol                                                                                      USERAUTHSUCCESSMITM
                                                                                                                              6 4                                                  4 5

                                                                                                                                                                                   5 5
                                                                                                                                       SSH Connection Protocol
Figure 6: Rogue Extension Negotiation Attack on AsyncSSH:
The MitM injects a malicious extension information message
before the key exchange completes and deletes the server’s                                      Figure 7: Rogue Session Attack on AsyncSSH: The MitM
E XT I NFO message to account for the change in sequence                                        injects a malicious authentication request before the hand-
numbers. This attack relates to the generic extension down-                                     shake is complete and deletes the client’s E XT I NFO message
grade attack in Section 5.2.                                                                    to account for the change in sequence numbers. By delay-
                                                                                                ing the authentication request sent by the client, the MitM
                                                                                                ensures that the malicious one is processed. Any additional
in handling unauthenticated messages during the handshake.                                      authentication requests are silently ignored.
These attacks are enabled by prefix truncation and sequence
number manipulation.
   Note that we describe these attacks only for ChaCha20-                                       to I GNORE, E XT I NFO does not trigger a response from the
Poly1305. Adjusting them for CBC-EtM is straightforward,                                        client. A correct SSH implementation should not process an
injecting appropriate I GNORE and U NKNOWN messages, but                                        unauthenticated E XT I NFO message. However, the injected
requires some of the advanced techniques described in Sec-                                      message is accepted due to flaws in AsyncSSH.
tion 4.1. These advanced techniques only work against some
SSH implementations.                                                                            Evaluation. We successfully evaluated the attack against
                                                                                                AsyncSSH 2.13.2 as a client, connecting to AsyncSSH 2.13.2.
6.1                         Rogue Extension Negotiation Attack
The rogue extension negotiation attack targets an AsyncSSH                                      6.2                         Rogue Session Attack
client connecting to any SSH server sending an E XT I NFO                                       The rogue session attack targets any SSH client connecting
message. The attack exploits an implementation flaw in the                                      to an AsyncSSH server, on which the attacker must have a
AsyncSSH client to inject an E XT I NFO message chosen by                                       shell account. The attack’s goal is to log the client into the
the attacker and a prefix truncation against the server to delete                               attacker’s account without the client being able to detect this.
its E XT I NFO message, effectively replacing it.
                                                                                                Impact. With a successful attack, the attacker can gain com-
Impact. The attacker can replace the content of the                                             plete control over the remote end of the SSH session. The
E XT I NFO message.      AsyncSSH clients support the                                           attacker receives all keyboard input by the user, completely
server-sig-algs and global-requests-ok extensions.                                              controls the terminal output of the user’s session, can send
Hence, the attacker can try to downgrade the algorithm                                          and receive data to/from forwarded network ports, and can
used for client authentication by restricting the value of                                      create signatures with a forwarded SSH Agent, if any. The
server-sig-algs to a subset of those supported by the                                           result is a complete break of the confidentiality and integrity
server.                                                                                         of the secure channel, providing a strong vector for a targeted
                                                                                                phishing campaign against the user. For example, the attacker
Attack Description. The attack is a variant of the extension                                    can display a password prompt and wait for the user to enter
downgrade attack in Section 5.2, but instead of I GNORE, the                                    the password, elevating the attacker’s position to a MitM at
attacker sends a chosen E XT I NFO packet to the client. Similar                                the application layer and enabling impersonation attacks.


                                                                                           13
Attack Description. The messages exchanged during the                     Cipher Family                  Preferred          Supported
attack are depicted in Figure 7. The attacker injects a chosen
                                                                          ChaCha20-Poly1305           8,739k 57.64% 10,247k 67.58%
U SER AUTH R EQUEST before the client’s N EW K EYS. This                  AES-CTR                     4,785k 31.56% 14,866k 98.04%
request must be a valid authentication request containing                 AES-GCM                     1,219k 8.04% 10,450k 68.92%
the credentials of the attacker. The attacker can use any                 AES-CBC                       236k 1.56% 4,069k 26.84%
authentication mechanism that does not require exchang-                   Other                         147k 0.97%        -       -
ing additional messages between client and server, such as                Unknown / No K EX I NIT        34k 0.23%        -       -
password or publickey. Due to a state machine flaw, the                   Total                     15,164k     100%
AsyncSSH server accepts the unauthenticated U SER AUTH R E -
QUEST message and defers it until the client has requested
                                                                         Table 2: Preferred SSH cipher families as of October 2023.
the authentication protocol.
   To avoid a race condition between the U SER AUTH R E -
QUEST sent by the client and the U SER AUTH R EQUEST in-                 tool, on a subset of 220 open ports. The scan covered a total
jected by the attacker, the attacker delays the client’s U SER -         of 830k servers. All data relating to the use of extension
AUTH R EQUEST until after the server signals a successful                negotiation in SSH is sourced from this scan.
authentication in response to the injected U SER AUTH R E -                 In SSH, the algorithm order of the client determines which
QUEST. The AsyncSSH server silently ignores any additional               algorithm is preferred. However, we cannot scan for actual
authentication request after a successful authentication.                client use. Assuming that servers and clients are bundled in
   To complete the attack, the attacker has to fix the sequence          a single product and share algorithm preference and support,
numbers using one of two strategies (note that Figure 7 only             we use the server’s lists as a surrogate, as was also done in [1].
shows the first strategy):

    • Suppose the client sends an extra message before S ER -            Symmetric Encryption Algorithms. In Table 2, we show
      VICE R EQUEST. In that case, the attacker can delete that          the number of servers that prefer and support various encryp-
      message from the channel, effectively performing the               tion modes. A cipher is preferred if it is placed first in the list
      (0,1)-prefix truncation attack with U SER AUTH R EQUEST            of supported algorithms.
      instead of the usual I GNORE message.                                 We find that, by far, the most preferred encryption cipher
                                                                         is ChaCha20-Poly1305, with 57.64% listing this algorithm
    • Alternatively, suppose the server sends an extra mes-              first. This is followed by AES-CTR (31.56%) and, with some
      sage before S ERVICE ACCEPT. In that case, the attacker            distance, by AES-GCM (8.04%) and AES-CBC (1.56%).
      can delete that message after injecting an additional U N -
      KNOWN message to the client before N EW K EYS , trig-
                                                                         Authenticated Encryption Modes. As non-AEAD ciphers
      gering an U NIMPLEMENTED response that is deleted.
                                                                         must be combined with a MAC, we also evaluate which
      This increases both C.Snd and C.Rcv, moving the send
                                                                         authenticated encryption modes the servers prefer and sup-
      count deficit from the client to the server.
                                                                         port. The numbers for the AEAD modes ChaCha20-Poly1305
                                                                         (57.64%) and GCM (8.04%) are identical to those for encryp-
Evaluation. We successfully evaluated the attack against                 tion modes, as the MAC is already integrated. Preference
AsyncSSH 2.13.2 as a server, connecting to AsyncSSH 2.13.2               for CTR modes is split between a majority for CTR-EaM
and OpenSSH 9.4p1.                                                       (26.14%) and a minority for CTR-EtM (5.46%). Preference
                                                                         for CBC modes is mostly CBC-EaM (2.37%), while prefer-
7    SSH Deployment Statistics                                           ence for CBC-EtM (0.09%) is marginal.
                                                                            In summary, 63.2% of all servers prefer an authenticated
To estimate the impact of the prefix truncation attacks, we              encryption mode affected by our attacks.
scan for the SSH servers preferring or supporting any affected              Looking at the support for authenticated encryption modes
encryption mode. Similarly, to estimate the impact of the                vulnerable to our attacks, we find that 67.58% of all servers
extension downgrade attack, we scan for servers sending                  support ChaCha20-Poly1305, while 17.24% support CBC-
E XT I NFO messages.                                                     EtM. In total, 71.6% support at least one affected mode.

Methodology. For scanning, we used ZMap [16] and                         SSH Extensions. We also looked at SSH extensions offered
ZGrab2 [15] on port 22 of the entire IPv4 address space.                 by servers before user authentication; see Table 4. We can
The scan was performed over two days in early October 2023,              see that 76.81% of all servers send the server-sig-algs
totaling 15.164M SSH servers.                                            extensions to indicate support for better signature schemes
   As ZGrab2 cannot capture SSH extensions, we performed                 for client public key authentication. Furthermore, 8.8% send
a complementary scan at the end of June 2023, using a custom             the publickey-hostbound extension, improving security


                                                                    14
 AE Mode                      Preferred         Supported              [36, Sec. 1.10]. In addition to resetting sequence numbers,
                                                                       “strict kex” mandates that unexpected or unknown messages
 ChaCha20-Poly1305          8,739k 57.64% 10,247k 67.58%
 CTR-EaM                    3,964k 26.14% 4,200k 27.70%
                                                                       during the initial key exchange must lead to the connection’s
 GCM                        1,219k 8.04% 10,450k 68.92%                termination. An unexpected message in this context is any
 CTR-EtM                      828k 5.46% 10,685k 70.46%                message that is not strictly required for key exchange. “strict
 CBC-EaM                      359k 2.37% 1,585k 10.46%                 kex” has since been adopted by various vendors to ensure
 CBC-EtM                       14k 0.09% 2,614k 17.24%                 interoperability between SSH implementations.
 Other                          2k 0.01%        -       -
 Unknown / No K EX I NIT       36k 0.24%        -       -
                                                                       Full Transcript MAC. Authenticating the full handshake
 Total                     15,164k    100%                             transcript, as seen by the client and server, can detect attempts
                                                                       of handshake manipulation by a MitM attacker, including
Table 3: Distribution of supported authenticated encryption            sequence number manipulation through our techniques. It is
modes as of October 2023.                                              impossible to extend the scope of the existing exchange hash,
                                                                       as the server signature is transmitted before the new keys
         Extension name               Times Offered                    are taken into use. Therefore, any messages sent after the
         server-sig-algs             637,466   76.81%                  key exchange but before N EW K EYS cannot be included. We
         publickey-hostbound@         73,040    8.80%                  suggest that both peers send a MAC authenticating the entire
         delay-compression               283    0.03%                  transcript at the start of the channel, similar to TLS Finished
         no-flow-control                 283    0.03%                  messages. Signaling support should be done as above. How-
         global-requests-ok              283    0.03%                  ever, the transcript must be carefully canonicalized. While
                                                                       client and server messages are sequential, they can interleave
                                                                       asynchronously, leading to transcript variations. Also, the pro-
Table 4: SSH extensions offered by servers after the initial
                                                                       tocol must be extended to define the algorithm, encoding, and
handshake, @openssh.com abbreviated to @. Extensions sent
                                                                       position of the transcript MAC. Thus, securing the handshake
by servers upon successful client authentication are not in-
                                                                       is more complex than resetting the sequence number.
cluded.

                                                                       Relationship to Formal Proofs. Both countermeasures
for authentication using SSH agent. Both extensions provide            have a common goal: Align the SSH standard with expecta-
opportunities for downgrade attacks, as their absence can              tions for stateful encryption schemes from formal models for
weaken the strength of the authentication.                             the BPP presented in [1,4]. A sequence number reset achieves
                                                                       this directly by initializing the sequence numbers to zero, as
8   Suggested Countermeasures                                          in the models. On the other hand, verifying the full transcript
                                                                       hash forces the sequence number in the stateful encryption
As a stop-gap measure, the affected cipher modes can be                and decryption methods to be synchronized by the sender and
turned off. Widely supported alternatives are AES-GCM or               receiver. Although the sequence numbers are then not initial-
AES-CTR. However, the root cause analysis shows that the               ized to zero, each pair is nevertheless initialized to a common
underlying issues lie in the SSH specification. We therefore           value out of the attacker’s control. The existing models could
suggest two changes to the specification.                              then be adjusted in the following way: If TC , TS ∈ {0, 1}∗ are
                                                                       the (canonicalized) transcripts of the SSH handshake as seen
Sequence Number Reset. Resetting sequence numbers to                   by the client and the server, and MCS , MSC : {0, 1}∗ 7→ N are
zero when encryption keys are activated ensures that sequence          functions counting the messages from the client to the server
number manipulations during the handshake can no longer                and vice versa in a transcript, then sequence numbers in the
affect the secure channel. Unfortunately, sequence number              stateful encryption and decryption modes are initialized to:
reset is a major break in compatibility. To avoid connection
failures due to one-sided sequence number resets, we suggest                   C.Snd = MCS (TC ),          C.Rcv = MSC (TC ),
that an implementation signals the support for this counter-                   S.Snd = MSC (TS ),          S.Rcv = MCS (TS ).
measure by including an identification string in the list of
supported key exchange algorithms. The SSH extension ne-               Authenticating the transcript then ensures that TC = TS , and
gotiation mechanism is already employing this method. If               thus C.Snd = S.Rcv and C.Rcv = S.Snd, before the first
and only if both peers signal support for this countermeasure,         messages in the secure channel are encrypted or decrypted.
the sequence numbers will be reset.                                    Authenticating the handshake transcript has the added ben-
   In response to our findings, OpenSSH implemented this               efit that the handshake could be analyzed in a “matching
behavior as part of their so-called “strict kex” countermeasure        conversations”-based security model [23, 24].


                                                                  15
Other Issues. We suggest that SSH specifies “end-of-                     sights into the complex interplay between a practical security
communication” messages to detect suffix truncation attacks.             mechanism (sequence numbers) and abstract security notions
Also, AsyncSSH should be hardened to disallow unauthenti-                (INT-PTXT vs. INT-CTXT, [6]).
cated, application-layer messages during the SSH handshake.                 Our close look at the extension negotiation mechanism
In response to our findings, the state machine of AsyncSSH               reveals its design weaknesses: First, sending E XT I NFO is
was improved in version 2.14.1 to mitigate our attacks.                  optional even if both parties signal support for extension ne-
                                                                         gotiation during the handshake. Second, E XT I NFO cannot
                                                                         be used to change the SSH handshake itself, e.g., to imple-
9    Future Work                                                         ment the countermeasures proposed in this paper. However,
                                                                         it outperforms extension negotiation within the K EX I NIT in
Formally, SSH BPP security was modeled as stateful decryp-               aspects of privacy as protocol extension can be negotiated
tion [1, 4, 40]. Implicitly, this state was associated with SSH          securely, i.e., privately, similar to encrypted extensions in
sequence numbers, and it was assumed that an adversary                   TLS 1.3. As a consequence, extension negotiation within the
could not manipulate this state. These models can be ex-                 K EX I NIT should be strictly limited to extensions affecting
tended in two directions: (1) Include a broader definition of            the SSH handshake. Protocol extensions affecting the user au-
state. By including chained IVs, key stream state, and GCM               thentication or application layer should be negotiated through
invocation counters, these models can be used to show why                the extension negotiation mechanism.
certain cipher modes resist our attacks and that they indeed                Although we suggest backward-compatible countermea-
achieve INT-PST security. (2) Introduce a novel adversarial              sures to stop our attacks, the security of the SSH protocol
query, ModifyState, to model the attacks described here.                 could benefit from a redesign from scratch. The redesign
   Our attack combines weaknesses in the SSH handshake                   process could be inspired by that of TLS 1.3, which brought
with weaknesses in the encrypted channel. Earlier work an-               implementers together with experts in protocol analysis and
alyzed these separately, leading to small models. To find                formal verification [3]. This could simplify the protocol while
our attack automatically, models of SSH for computer-aided               preserving and/or achieving desired security notions for SSH,
proofs could (1) model the handshake as well as the BPP to-              which may differ from those of TLS. For example, while the
gether, (2) keep track of sequence numbers in the BPP, includ-           privacy of client authentication and extension negotiation are
ing the handshake, which requires modeling integer numbers               relatively new features for TLS, they are already present in
that can overflow as the internal state, (3) model seemingly             SSH and should thus be preserved in a redesign.
unimportant messages like I GNORE, and (4) consider each
encryption mode separately. The properties to verify should
include strong security notions such as INT-aPTXT [17].                  Acknowledgements. Fabian Bäumer was supported by the
   Applying state learning to implementations also has the po-           German Federal Ministry for Economic Affairs and Cli-
tential to find our attacks automatically in the future, although        mate Action (BMWK) project “Industrie 4.0 Recht-Testbed”
it suffers from a combinatorial explosion in the number of               (13I40V002C). Marcus Brinkmann was supported by the
messages (see Section 5.1 in [28]). Messages like I GNORE                Deutsche Forschungsgemeinschaft (DFG, German Research
and E XT I NFO need to be included in the alphabet to find our           Foundation) under Germany’s Excellence Strategy - EXC
attacks, and an active MitM attacker has to be considered.               2092 CASA - 390781972.


10    Conclusion                                                         References
                                                                          [1] Martin R. Albrecht, Jean Paul Degabriele, Tor-
We have shown that the complexity of SSHv2 has increased
                                                                              ben Brandt Hansen, and Kenneth G. Paterson. A surfeit
over its 25 years of development to a point where the addition
                                                                              of SSH cipher suites. In Edgar R. Weippl, Stefan
of new algorithms and features has introduced new vulnerabil-
                                                                              Katzenbeisser, Christopher Kruegel, Andrew C. My-
ities. The root cause analysis has shown that the potential for
                                                                              ers, and Shai Halevi, editors, ACM CCS 2016, pages
our attacks was already present in the original specification.
                                                                              1480–1491. ACM Press, October 2016.
Handshake transcripts were never fully authenticated, and
sequence numbers were never reset to 0. However, as new                   [2] Martin R. Albrecht, Kenneth G. Paterson, and Gaven J.
authenticated encryption modes and extension messages were                    Watson. Plaintext recovery attacks against SSH. In
added, these weaknesses grew into exploitable vulnerabilities.                2009 IEEE Symposium on Security and Privacy, pages
   We introduced novel sequence number manipulation and                       16–26. IEEE Computer Society Press, May 2009.
prefix truncation attacks for secure channels, which invalidate
the INT-aPTXT [17] security of SSH BPP for certain ciphers.               [3] Manuel Barbosa, Gilles Barthe, Karthik Bhargavan,
We extended these vulnerabilities to real-world exploits like                 Bruno Blanchet, Cas Cremers, Kevin Liao, and Bryan
disabling SSH extension negotiation. This yields novel in-                    Parno. SoK: Computer-aided cryptography. In 2021


                                                                    16
     IEEE Symposium on Security and Privacy, pages 777–             [14] Wei Dai.   email to IETF mailing list.   https:
     795. IEEE Computer Society Press, May 2021.                         //www.ietf.org/ietf-ftp/ietf-mail-archive/
                                                                         secsh/2002-02.mail, 2002. Accessed: 2023-10-11.
 [4] Mihir Bellare, Tadayoshi Kohno, and Chanathip Nam-
     prempre. Authenticated encryption in SSH: Provably             [15] Zakir Durumeric, David Adrian, Ariana Mirian, Michael
     fixing the SSH binary packet protocol. In Vijayalakshmi             Bailey, and J. Alex Halderman. A search engine backed
     Atluri, editor, ACM CCS 2002, pages 1–11. ACM Press,                by internet-wide scanning. In Indrajit Ray, Ninghui
     November 2002.                                                      Li, and Christopher Kruegel, editors, ACM CCS 2015,
                                                                         pages 542–553. ACM Press, October 2015.
 [5] Mihir Bellare and Chanathip Namprempre. Authenti-
     cated encryption: Relations among notions and analy-           [16] Zakir Durumeric, Eric Wustrow, and J. Alex Halderman.
     sis of the generic composition paradigm. In Tatsuaki                ZMap: Fast internet-wide scanning and its security ap-
     Okamoto, editor, ASIACRYPT 2000, volume 1976 of                     plications. In Samuel T. King, editor, USENIX Security
     LNCS, pages 531–545. Springer, Heidelberg, December                 2013, pages 605–620. USENIX Association, August
     2000.                                                               2013.
 [6] Mihir Bellare and Chanathip Namprempre. Authenti-              [17] Marc Fischlin, Felix Günther, Giorgia Azzurra Marson,
     cated encryption: Relations among notions and analysis              and Kenneth G. Paterson. Data is a stream: Security
     of the generic composition paradigm. Journal of Cryp-               of stream-based channels. In Rosario Gennaro and
     tology, 21(4):469–491, October 2008.                                Matthew J. B. Robshaw, editors, CRYPTO 2015, Part II,
                                                                         volume 9216 of LNCS, pages 545–564. Springer, Hei-
 [7] Florian Bergsma, Benjamin Dowling, Florian Kohlar,
                                                                         delberg, August 2015.
     Jörg Schwenk, and Douglas Stebila. Multi-ciphersuite
     security of the Secure Shell (SSH) protocol. In Gail-          [18] Cédric Fournet.    email to IETF mailing list.
     Joon Ahn, Moti Yung, and Ninghui Li, editors, ACM                   https://mailarchive.ietf.org/arch/msg/tls/
     CCS 2014, pages 369–381. ACM Press, November                        extoO9ETJLnEm3MRDTO23x70DFM, 2015. Accessed:
     2014.                                                               2023-10-16.
 [8] Karthikeyan Bhargavan and Gaëtan Leurent. Transcript
                                                                    [19] Torben Brandt Hansen. Cryptographic Security of SSH
     collision attacks: Breaking authentication in TLS, IKE
                                                                         Encryption Schemes. Phd thesis, University of London,
     and SSH. In NDSS 2016. The Internet Society, February
                                                                         2020.
     2016.

 [9] Denis Bider. Extension Negotiation in the Secure Shell         [20] Nadia Heninger, Zakir Durumeric, Eric Wustrow, and
     (SSH) Protocol. RFC 8308, March 2018.                               J. Alex Halderman. Mining your ps and qs: Detection of
                                                                         widespread weak keys in network devices. In Tadayoshi
[10] Bruno Blanchet and David Pointcheval. Automated                     Kohno, editor, USENIX Security 2012, pages 205–220.
     security proofs with sequences of games. In Cynthia                 USENIX Association, August 2012.
     Dwork, editor, CRYPTO 2006, volume 4117 of LNCS,
     pages 537–554. Springer, Heidelberg, August 2006.              [21] Kevin Igoe. Suite B Cryptographic Suites for Secure
                                                                         Shell (SSH). RFC 6239, May 2011.
[11] David Cadé and Bruno Blanchet. From computationally-
     proved protocol specifications to implementations. In          [22] Kevin Igoe and Jerome Solinas. AES Galois Counter
     2012 Seventh International Conference on Availability,              Mode for the Secure Shell Transport Layer Protocol.
     Reliability and Security, pages 65–74, 2012.                        RFC 5647, August 2009.

[12] Ran Canetti and Hugo Krawczyk.          Analysis of            [23] Tibor Jager, Florian Kohlar, Sven Schäge, and Jörg
     key-exchange protocols and their use for building se-               Schwenk. On the security of TLS-DHE in the stan-
     cure channels. In Birgit Pfitzmann, editor, EURO-                   dard model. In Reihaneh Safavi-Naini and Ran Canetti,
     CRYPT 2001, volume 2045 of LNCS, pages 453–474.                     editors, CRYPTO 2012, volume 7417 of LNCS, pages
     Springer, Heidelberg, May 2001.                                     273–293. Springer, Heidelberg, August 2012.

[13] Vincent Cheval, Charlie Jacomme, Steve Kremer, and             [24] Hugo Krawczyk, Kenneth G. Paterson, and Hoeteck
     Robert Künnemann. SAPIC+: protocol verifiers of                     Wee. On the security of the TLS protocol: A systematic
     the world, unite! In 31st USENIX Security Symposium                 analysis. In Ran Canetti and Juan A. Garay, editors,
     (USENIX Security 22), pages 3935–3952, Boston, MA,                  CRYPTO 2013, Part I, volume 8042 of LNCS, pages
     August 2022. USENIX Association.                                    429–448. Springer, Heidelberg, August 2013.


                                                               17
[25] Pascal Lafourcade and Maxime Puys. Performance                   [37] Chanathip Namprempre, Tadayoshi Kohno, and Mihir
     evaluations of cryptographic protocols verification tools             Bellare. The Secure Shell (SSH) Transport Layer En-
     dealing with algebraic properties. In Joaquin Garcia-                 cryption Modes. RFC 4344, January 2006.
     Alfaro, Evangelos Kranakis, and Guillaume Bonfante,
     editors, Foundations and Practice of Security, pages             [38] Richard Ogier. OSPF Database Exchange Summary
     137–155, Cham, 2016. Springer International Publish-                  List Optimization. RFC 5243, May 2008.
     ing.                                                             [39] Kenneth G. Paterson, Thomas Ristenpart, and Thomas
[26] Adam Langley and Wan-Teh Chang. ChaCha20 and                          Shrimpton. Tag size does matter: Attacks and proofs
     Poly1305 based Cipher Suites for TLS. Internet-Draft                  for the TLS record protocol. In Dong Hoon Lee and Xi-
     draft-agl-tls-chacha20poly1305-04, Internet Engineer-                 aoyun Wang, editors, ASIACRYPT 2011, volume 7073
     ing Task Force, November 2013. Work in Progress.                      of LNCS, pages 372–389. Springer, Heidelberg, Decem-
                                                                           ber 2011.
[27] Adam Langley, Wan-Teh Chang, Nikos Mavro-
     giannopoulos, Joachim Strombergson, and Simon Josef-             [40] Kenneth G. Paterson and Gaven J. Watson. Plaintext-
     sson. ChaCha20-Poly1305 Cipher Suites for Transport                   dependent decryption: A formal security treatment of
     Layer Security (TLS). RFC 7905, June 2016.                            SSH-CTR. In Henri Gilbert, editor, EUROCRYPT 2010,
                                                                           volume 6110 of LNCS, pages 345–361. Springer, Hei-
[28] Eric Lesiuta, Victor Bandur, and Mark Lawford. Slime:                 delberg, May / June 2010.
     State learning in the middle of everything for tool-
     assisted vulnerability detection. In Computer Security.          [41] Kenny Paterson.     Advanced security notions for
     ESORICS 2022 International Workshops, pages 686–                      the SSH secure channel: theory and practice.
     704, Cham, 2023. Springer International Publishing.                   https://summerschool-croatia.cs.ru.nl/2017/
                                                                           slides/Advanced%20security%20notions%20for%
[29] Chris M. Lonvick and Sami Lehtinen. The Secure Shell                  20the%20SSH%20secure%20channel.pdf,         2017.
     (SSH) Protocol Assigned Numbers. RFC 4250, January                    Accessed: 2024-02-16.
     2006.
                                                                      [42] Eric Rescorla. The Transport Layer Security (TLS)
[30] Chris M. Lonvick and Tatu Ylonen. The Secure Shell
                                                                           Protocol Version 1.3. RFC 8446, August 2018.
     (SSH) Authentication Protocol. RFC 4252, January
     2006.                                                            [43] Phillip Rogaway.       Authenticated-encryption with
                                                                           associated-data. In Vijayalakshmi Atluri, editor, ACM
[31] Chris M. Lonvick and Tatu Ylonen. The Secure Shell
                                                                           CCS 2002, pages 98–107. ACM Press, November 2002.
     (SSH) Connection Protocol. RFC 4254, January 2006.
[32] Chris M. Lonvick and Tatu Ylonen. The Secure Shell               [44] Ben Smyth and Alfredo Pironti. Truncating TLS con-
     (SSH) Protocol Architecture. RFC 4251, January 2006.                  nections to violate beliefs in web applications. In 7th
                                                                           USENIX Workshop on Offensive Technologies (WOOT
[33] Chris M. Lonvick and Tatu Ylonen. The Secure Shell                    13), Washington, D.C., August 2013. USENIX Associa-
     (SSH) Transport Layer Protocol. RFC 4253, January                     tion.
     2006.
                                                                      [45] Dawn Xiaodong Song, David A. Wagner, and Xuqing
[34] Damien Miller. This document describes the chacha20-                  Tian. Timing analysis of keystrokes and timing attacks
     poly1305@openssh.com authenticated encryption                         on SSH. In Dan S. Wallach, editor, USENIX Security
     cipher supported by openssh.      https://cvsweb.                     2001. USENIX Association, August 2001.
     openbsd.org/cgi-bin/cvsweb/src/usr.bin/ssh/
     PROTOCOL.chacha20poly1305?rev=1.5. Accessed:                     [46] Michael Williams, Michael Tüxen, and Robin Seggel-
     2023-10-18.                                                           mann. Transport Layer Security (TLS) and Datagram
                                                                           Transport Layer Security (DTLS) Heartbeat Extension.
[35] Damien Miller. SSH agent restriction. https://www.                    RFC 6520, February 2012.
     openssh.com/agent-restrict.html, 2022.         Ac-
     cessed: 2023-10-17.                                              [47] Stephen C. Williams. Analysis of the SSH key exchange
                                                                           protocol. In Liqun Chen, editor, 13th IMA International
[36] Damien Miller, Markus Friedl, Mike Frysinger, Todd C.                 Conference on Cryptography and Coding, volume 7089
     Miller, and Darren Tucker. This documents openssh’s                   of LNCS, pages 356–374. Springer, Heidelberg, Decem-
     deviations and extensions to the published ssh protocol.              ber 2011.
     https://cvsweb.openbsd.org/cgi-bin/cvsweb/
     src/usr.bin/ssh/PROTOCOL?rev=1.55. Accessed:
     2024-02-18.


                                                                 18
A     Artifact Appendix                                                   A.2.5   Benchmarks
                                                                          None.
A.1      Abstract
This document describes the artifacts to the USENIX Secu-                 A.3     Set-up
rity ’24 Publication Terrapin Attack: Breaking SSH Channel
Integrity By Sequence Number Manipulation.                                All required Docker images are built on demand when the
   Using these instructions, the evaluations of Sequence Num-             evaluation scripts are executed, so no setup is required.
ber Manipulation (Sect. 4.1), Extension Downgrade Attack                     The TCP ports 2200 and 2201 should be free and available.
(Sect. 5.2), Rogue Extension Attack (Sect. 6.1) and Rogue                 This is the case by default on many systems. Some systems
Session Attack (Sect. 6.2) can be reproduced.                             might require a configuration of the firewall to allow the test
   Also, the aggregation scripts for the internet scans are avail-        servers to bind 0.0.0.0 on these ports. On some systems,
able and can be tested on a small subset of the samples.                  the firewall will show a pop-up dialog when the first server
                                                                          starts up, requiring manual confirmation.

A.2      Description & Requirements                                       A.3.1   Installation
A.2.1     Security, privacy, and ethical concerns                         Linux: Install the Docker engine under a supported Linux
The configuration uses the host network to allow (optional)               distribution by following the instructions available at https:
monitoring of the attack using Wireshark or other network                 //docs.docker.com/engine/install/.
packet analysis tools on the loopback interface. During the
runtime of the evaluation, this makes the tested SSH server               MacOS: Install Docker Desktop available at https://www.
and proof of concept (PoC) available to all systems with ac-              docker.com/products/docker-desktop/.
cess to the local network (TCP bind to 0.0.0.0, ports 2200
and 2201). Reviewers should take care to isolate the test                 A.3.2   Basic Test
system from the internet, for example using a firewall.
                                                                          The following scripts build all required Docker images and
                                                                          can be used as a basic functionality test. It will also be called
A.2.2     How to access                                                   by all evaluation scripts, so this step is optional.
The artifacts are publically available at https:                          1   $ impl/build.sh
//github.com/RUB-NDS/Terrapin-Artifacts/tree/                             2   [+] Building 2.0s (15/15) FINISHED
                                                                          3   [...]
9907c80fa7e4184a29ceac352947ea51a49dce6a.
                                                                          4   => => naming to docker.io/terrapin-artifacts/openssh-
                                                                                   ,→ server:9.4p1
                                                                          5   [...]
A.2.3     Hardware dependencies
                                                                          6   $ pocs/build.sh
                                                                          7   [...]
None.
                                                                            The output shows the progress on downloading base images
A.2.4     Software dependencies                                           and building the evaluation images. If there is no output, all
                                                                          docker images are already built.
    • Linux or MacOS.6 No specific distribution or version
      is required. We used Manjaro (rolling release in March
      2024) and MacOS 14.4 (Sonoma).                                      A.4     Evaluation workflow
                                                                          A.4.1   Major Claims
    • Bash shell interpreter (typically included in the above).
      No specific version is required. We used bash 5.2.26 and            We evaluated our attacks against several clients using an
      3.2.57.                                                             OpenSSH 9.5p1 (C1, C2) or AsyncSSH 2.13.2 (C3, C4)
                                                                          server. For an overview of the expected outcomes, see also Ta-
    • Docker Engine or Docker Desktop. While Docker En-                   ble 5.
      gine suffices and is typically included in Linux distribu-          (C1): Sequence Number Manipulation (Sect. 4.1). We
      tions, Docker Desktop is a separate install on MacOS.                    verified all techniques successfully against PuTTY 0.79.
      No specific version is required. We used Docker En-                      Additionally, our experiments show that OpenSSH 9.5p1
      gine 25.0.3 and Docker Desktop 4.28.0.                                   recognizes a rollover of sequence numbers and termi-
                                                                               nates the connection, thus not affected by any technique
    • Wireshark (optional), for network packet analysis.
                                                                               but RcvIncrease. AsyncSSH 2.13.2 and libssh 0.10.5 al-
    6Windows WSL might work but is untested and not supported.                 low for RcvIncrease but terminate the connection due to


                                                                     19
                                  O pe
                                            0.79                                    Shortcut    Description
                                       nS SH   9.
                                  O
                                  D
                                    pe nS SH
                                                  4p1                               q           Quit
                                    ro         9.
                              Y
                             TT
                                  A pbe 5p1
                                    sy ar
                                  lib ncS 20
                                     ss SH 22                                       h           Help
                                       h 0 . 2 .1 8 3.
Attack                   Pu                 10 3
                                               .5 .2                                S           Wrap long lines on/off
                                                                                    /           Search
C1 RcvIncrease           ✓         -      ✓       ✓ ✓ ✓
C1 RcvDecrease           ✓         -      R       ✓ T T                             :n          Next file
C1 SndIncrease           ✓         -      R       U T T                             :p          Previous file
C1 SndDecrease           ✓         -      R       U T T
C2 ChaCha-Poly           ✓         ✓      ✓       -   -   -                  Table 6: Common keyboard shortcuts of less.
C2 CBC-EtM
   - U NKNOWN         0.0300 0.0003 0.0003 -          -   -
   - P ING            0.8383    -   0.0074 -          -   -            gentle introduction to less, see Table 6 for a quick reference
C3 Rogue Extension       -         -       -      - ✓ -                of useful keyboard shortcuts.
C4 Rogue Session         -         -       -      - ✓ -                (E1): test-sqn-manipulation.sh [≈ 1 − 3 hours per
                                                                            client/variant combination]: Run one of the four se-
 - Not evaluated.
 ✓ Attack succeeds.
                                                                            quence number manipulation attacks to prove (C1).
 R Client terminates the connection (rollover).
                                                                            RcvIncrease is very fast; the others can be slow.
 T Client terminates the connection (timeout).                              Execution: After starting the script, choose a client, one
 U Client terminates the connection (U NKNOWN message).                     of the four attack options, and input the manipulation
                                                                            offset N. To prove (C1), input N = 1.
   Table 5: Expected outcomes for attacks against clients                   Results: The attack is complete once the progress bar
                                                                            fills. After that, there will be an error message because
                                                                            the secure channel is broken, as the script does not im-
    handshake timeouts before any advanced technique con-                   plement any prefix truncation to complete the attack.
    cludes. Dropbear 2022.83 disconnects on U NKNOWN                   (E2a): test-ext-downgrade.sh [≈ 1 minute]: Run the ex-
    messages instead of responding with U NIMPLEMENTED                      tension downgrade attack to prove (C2) for ChaCha20-
    but allows Rcv to roll over, therefore being affected by                Poly1305.
    RcvIncrease and RcvDecrease only.                                       Execution: After starting the script, choose an arbitrary
(C2): Extension Downgrade (Sect. 5.2). We successfully                      client and server combination. Afterward, choose attack
    evaluated the attack in 10,000 trials on ChaCha20-                      variant 1 to select ChaCha20-Poly1305.
    Poly1305 and CBC-EtM against OpenSSH 9.5p1 and                          Results: The script will conclude by opening the fol-
    PuTTY 0.79 clients, connecting to OpenSSH 9.4p1                         lowing files simultaneously in less:
    (U NKNOWN only) and 9.5p1. For CBC-EtM, our suc-                          1. diff of files 3 and 4
    cess rate in practice was 0.0003 (OpenSSH) resp. 0.0300                   2. diff of files 5 and 6
    (PuTTY), improved to 0.0074 (OpenSSH) resp. 0.8383
    (PuTTY) when sending P ING instead of U NKNOWN.                           3. Server log (unmodified connection)
(C3): Rogue Extension Negotiation (Sect. 6.1). We suc-                        4. Server log (tampered connection)
    cessfully evaluated the attack against AsyncSSH 2.13.2                    5. Client log (unmodified connection)
    as a client, connecting to AsyncSSH 2.13.2.                               6. Client log (tampered connection)
(C4): Rogue Session Attack (Sect. 6.2). We successfully
                                                                              7. PoC proxy log
    evaluated the attack against AsyncSSH 2.13.2 as a server,
    connecting to AsyncSSH 2.13.2.                                          Navigate to the second file. The file compares the out-
                                                                            put of the selected SSH client in the case of an ex-
                                                                            tension downgrade attack to the output of an unmod-
Internet Scan (Sect. 7) We are also including sample data,                  ified connection. The diff will indicate the presence of
aggregated data, and evaluation scripts on the Internet scan.               SSH_MSG_EXT_INFO and absence of SSH_MSG_IGNORE
                                                                            in the unmodified connection only, thus proving (C2) for
A.4.2    Experiments                                                        ChaCha20-Poly1305.
                                                                       (E2b): bench-ext-downgrade.sh [≈ 1 − 2 hours per clien-
The evaluation scripts (in the directory scripts) are inter-                t/variant combination]: Run the extension downgrade
active and self-describing. Some of them have several out-                  attack 10,000 times to prove (C2) for CBC-EtM
put files. In that case, the files (as described below) are all             (U NKNOWN and P ING).
opened in the text file viewer less at the same time, requir-               Execution: After starting the script, choose between
ing keyboard-based navigation to see all of the results. As a               U NKNOWN and P ING variants of the attack, then select


                                                                  20
     between OpenSSH and PuTTY client. A progress bar                           sample.acc.json files (aside from the evaluation start
    will show the current trial.                                                and end timestamps).
     Results: After finishing all trial connections, the num-
     ber of successful trial runs will be outputted to the con-           A.4.3   Troubleshooting
     sole. The relative success rate will be close to the values
     claimed in (C2), thus proving the functionality and suc-             Address already in use. If an attack script is interrupted,
     cess probability claims in (C2) in the case of CBC-EtM.              some docker containers may not be cleaned up properly, block-
(E3): test-asyncssh-rogue-ext-negotiation.sh [≈ 1                         ing the server port permanently or for the duration of TIME-
     minute]: Run rogue extension attack to prove (C3).                   WAIT (1 min. on Linux, 30 sec. on MacOS).
     Execution: The attack is automatic.                                    Please follow these steps in this case:
     Results: The script will conclude by opening a set of
     seven files in less. Refer to the results of (E2a) for a               1. Run the script cleanup-system.sh. This will stop and
     list of files opened. Navigate to the second file. The                    remove any pending Docker containers.
     diff will indicate the presence of the server-sig-algs                 2. If the problem persists, wait for up to 4 minutes.
     extension with an attacker-chosen value in the tampered
     connection, thus proving (C3).
                                                                          System Reset. To fully clean up the Docker containers and
(E4): test-asyncssh-rogue-session-attack.sh [≈ 1
                                                                          images, you can run cleanup-system.sh --full.
     minute]: Run the rogue session attack to prove (C4).
     Execution: The attack is automatic.
     Results: The script will conclude by opening a set of                A.5     Notes on Reusability
     seven files in less. Refer to the results of (E2a) for a list
                                                                          The proof-of-concept code (pocs/) has been kept short for
     of files opened. Navigate to the first file. The diff will
                                                                          simplicity and is thus not modularized for reusability. How-
     indicate successful authentication for the victim (unmod-
                                                                          ever, the artifacts may serve as a template for other MitM
     ified connection) and attacker (tampered connection),
                                                                          attacks on network protocols like SSH.
     respectively. Afterward, navigate to the second file and
                                                                             The Docker files for the evaluated SSH implementations
     examine the output of each client connection at the end
                                                                          (impl/) may be generally useful in other research on SSH.
     of the file. In the unmodified connection, the server will
                                                                             Improvements to Wireshark for better dissection of SSH
     respond with the username victim, while in the attacked
                                                                          protocols (not included in these artifacts) have been submitted
     connection, the server will respond with the username
                                                                          and accepted upstream and will be available in a future version
     attacker. This proves (C4).
                                                                          of Wireshark.
(E5): scan_util.py [≈ 1 minute]: Run the script to aggre-
     gate a set of zgrab2 scan results. Note that this script
     is in the sub-directory scans. Without the full data, we             A.6     Version
     can not prove the statistics in our paper. However, we               Based on the LaTeX template for Artifact Evaluation
     can demonstrate how we aggregated the scan results and               V20231005. Submission, reviewing and badging methodol-
     classified the algorithms.                                           ogy followed for the evaluation of this artifact can be found at
     Execution: Build the docker image by running the fol-                https://secartifacts.github.io/usenixsec2024/.
     lowing command inside the scans directory:
      1   $ docker build . -t terrapin-artifacts/scan-util

     Now aggregate the sample.json file which can be
     found in the sample sub-directory by running the fol-
     lowing command inside the scans directory:
      1   $ docker run --rm -v ./sample:/input terrapin-
               ,→ artifacts/scan-util evaluate -i /input/
               ,→ sample.json -o /input/sample-ae.acc.json

     Results: The aggregation result will become avail-
     able as sample-ae.acc.json inside the sample sub-
     directory. The total number of clients, status and ver-
     sion distribution, offered key exchange algorithms, com-
     ment strings, and other evaluation criteria will match the
     data present within the sample.json file. Also, there
     is no difference between the sample-ae.acc.json and


                                                                     21
