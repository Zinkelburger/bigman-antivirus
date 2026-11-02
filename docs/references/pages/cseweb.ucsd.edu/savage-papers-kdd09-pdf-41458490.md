---
url: "https://cseweb.ucsd.edu/~savage/papers/KDD09.pdf"
title: "Beyond Blacklists: Learning to Detect Malicious Web Sites"
captured: "2026-11-02T18:25:25Z"
via: "live"
sha256: "b90b5c18ccc23e6a5cf80c7f72a12e0cb15e87a65dadfefbed6d64a48cd14438"
wayback: "https://web.archive.org/web/20260516182224/https://cseweb.ucsd.edu/~savage/papers/KDD09.pdf"
cited_in:
  - "docs/threats/01-phishing-social-engineering.md"
---

# Beyond Blacklists: Learning to Detect Malicious Web Sites

> Archived copy of a third-party page, kept so Bigman's findings survive if the original goes away. Copyright stays with the original publisher; cite the original URL.
>
> Original: <https://cseweb.ucsd.edu/~savage/papers/KDD09.pdf>
> Snapshot: <https://web.archive.org/web/20260516182224/https://cseweb.ucsd.edu/~savage/papers/KDD09.pdf>

Beyond Blacklists: Learning to Detect Malicious Web Sites
                 from Suspicious URLs

                          Justin Ma, Lawrence K. Saul, Stefan Savage, Geoffrey M. Voelker
                                                   Department of Computer Science and Engineering
                                                          University of California, San Diego




ABSTRACT                                                                                Clearly, if one could inform users beforehand that a particular
Malicious Web sites are a cornerstone of Internet criminal activi-                   URL was dangerous to visit, much of this problem could be allevi-
ties. As a result, there has been broad interest in developing sys-                  ated. To this end, the security community has responded by devel-
tems to prevent the end user from visiting such sites. In this paper,                oping blacklisting services — encapsulated in toolbars, appliances
we describe an approach to this problem based on automated URL                       and search engines — that provide precisely this feedback. These
classification, using statistical methods to discover the tell-tale lex-             blacklists are in turn constructed by a range of techniques including
ical and host-based properties of malicious Web site URLs. These                     manual reporting, honeypots, and Web crawlers combined with site
methods are able to learn highly predictive models by extracting                     analysis heuristics. Inevitably, many malicious sites are not black-
and automatically analyzing tens of thousands of features poten-                     listed — either because they are too new, were never evaluated, or
tially indicative of suspicious URLs. The resulting classifiers ob-                  were evaluated incorrectly (e.g., due to “cloaking”). To address this
tain 95–99% accuracy, detecting large numbers of malicious Web                       problem, some client-side systems analyze the content or behavior
sites from their URLs, with only modest false positives.                             of a Web site as it is visited. But, in addition to run-time overhead,
                                                                                     these approaches can expose the user to the very browser-based
                                                                                     vulnerabilities that we seek to avoid.
Categories and Subject Descriptors                                                      In this paper, we focus on a complementary part of the design
C.2.0 [Computer-Communication Networks]: General—Secu-                               space: lightweight URL classification — that is, classifying the rep-
rity and protection; I.5.1 [Pattern Recognition]: Models—Statis-                     utation of a Web site entirely based on the inbound URL. The mo-
tical; I.5.2 [Pattern Recognition]: Design Methodology—Feature                       tivation is to provide inherently better coverage than blacklisting-
evaluation and selection                                                             based approaches (e.g., correctly predicting the status of new sites)
                                                                                     while avoiding the client-side overhead and risk of approaches that
General Terms                                                                        analyze Web content on demand.
                                                                                        In particular, we explore the use of statistical methods from ma-
Algorithms, Security
                                                                                     chine learning for classifying site reputation based on the relation-
                                                                                     ship between URLs and the lexical and host-based features that
Keywords                                                                             characterize them. Unlike previous work, we show that these meth-
supervised learning, malicious Web sites, L1-regularization                          ods are able to sift through tens of thousands of features derived
                                                                                     from publicly available data sources, and can identify which URL
1. INTRODUCTION                                                                      components and meta-data are important without requiring heavy
                                                                                     domain expertise. Indeed, our system automatically selects many
   The Web has become a platform for supporting a wide range
                                                                                     of the same features identified by domain experts as being typical
of criminal enterprises such as spam-advertised commerce (e.g.,
                                                                                     of “malicious” Web sites. But more importantly, our system se-
counterfeit watches or pharmaceuticals), financial fraud (e.g., via
                                                                                     lects new, non-obvious features that are highly predictive and yield
phishing or 419-type scams) and as a vector for propagating mal-
                                                                                     substantial performance improvements. We evaluate this approach
ware (e.g., so-called “drive-by downloads”). Although the precise
                                                                                     across 20,000 to 30,000 URLs drawn from different sources, and
commercial motivations behind these schemes may differ, the com-
                                                                                     show that it can obtain an overall prediction accuracy of 95–99%,
mon thread among them is the requirement that unsuspecting users
                                                                                     detecting a large fraction of malicious websites while maintaining
visit their sites. These visits can be driven by email, Web search re-
                                                                                     a very low false positive rate.
sults or links from other Web pages, but all require the user to take
                                                                                        We begin the remainder of this paper with an explanation of the
some action, such as clicking, that specifies the desired Uniform
                                                                                     formal basis for our approach.
Resource Locator (URL).

                                                                                     2.    APPROACH
Permission to make digital or hard copies of all or part of this work for               In this section, we provide a detailed discussion of our approach
personal or classroom use is granted without fee provided that copies are            to classifying site reputation. We begin with an overview of the
not made or distributed for profit or commercial advantage and that copies           classification problem, followed by a discussion of the features we
bear this notice and the full citation on the first page. To copy otherwise, to      extract, and finally the set of machine learning classifiers we use
republish, to post on servers or to redistribute to lists, requires prior specific   for the study.
permission and/or a fee.
KDD’09, June 28–July 1, 2009, Paris, France.                                            For our purposes, we treat URL reputation as a binary classifi-
Copyright 2009 ACM 978-1-60558-495-9/09/06 ...$5.00.                                 cation problem where positive examples are malicious URLs and
negative examples are benign URLs. This learning-based approach          based features can describe “where” malicious sites are hosted,
to the problem can succeed if the distribution of feature values for     “who” own them, and “how” they are managed.
malicious examples is different from benign examples, the training          The following are properties of the hosts (there could be multi-
set shares the same feature distribution as the testing set, and the     ple) that are identified by the hostname part of the URL. We note
ground truth labels for the URLs are correct.                            some of these features overlap with lexical properties of the URL.
   Significantly, we classify sites based only on the relationship be-
tween URLs and the lexical and host-based features that character-          1. IP address properties — Is the IP address in a blacklist? Are
ize them, and we do not consider two other kinds of potentially use-           the IPs of the A, MX or NS records in the same autonomous
ful sources of information for features: the URL’s page content, and           systems (ASes) or prefixes as one another? To what ASes or
the context of the URL (e.g., the page or email in which the URL               prefixes do they belong?
is embedded). Although this information has the potential to im-            2. WHOIS properties — What is the date of registration, update,
prove classification accuracy, we exclude it for a variety of reasons.         and expiration ? Who is the registrar? Who is the registrant?
First, avoiding downloading page content is strictly safer for users.          Is the WHOIS entry locked?
Second, classifying a URL with a trained model is a lightweight
operation compared to first downloading the page and then using             3. Domain name properties — What is the time-to-live (TTL)
its contents for classification. Third, focusing on URL features               value for the DNS records associated with the hostname?
makes the classifier applicable to any context in which URLs are               Additionally, the following domain name properties are used
found (Web pages, email, chat, calendars, games, etc.), rather than            in SpamAssassin Botnet plugin for detecting links to mali-
dependent on a particular application setting. Finally, reliably ob-           cious sites in emails: Does the hostname contain “client” or
taining the malicious version of a page for both training and testing          “server” keywords? Is the IP address in the hostname? Is
can become a difficult practical issue. Malicious sites have demon-            there a PTR record for the host? Does the PTR record in-
strated the ability to “cloak” the content of their Web pages, i.e.,           turn resolve one of the host’s IP addresses?
serving different content to different clients [20]. For example, a
malicious server may send benign versions of a page to honeypot             4. Geographic properties — In which continent/country/ city
IP addresses that belong to security practitioners, but send mali-             does the IP address belong? What is the speed of the uplink
cious versions to other clients. Nonetheless, we show in Section 4             connection (broadband, dial-up, etc)?
that classifying on lexical features of the URL and features about
                                                                         This list of features we use is not exhaustive, as there is always
the host are sufficient for highly accurate prediction.
                                                                         the possibility of generating or aggregating new meta-information
                                                                         about the URL such as popularity rankings in Netcraft, indexing
2.1 Features                                                             in Google, etc. Nevertheless, the list is still extensive, as it repre-
   We categorize the features that we gather for URLs as being ei-       sents many pieces of information about the URL and host (much of
ther lexical or host-based.                                              which is publicly available) that we can collect efficiently through
   Lexical features: The justification for using lexical features is     the automated crawling tools at our disposal.
that URLs to malicious sites tend to “look different” in the eyes of
the users who see them. Hence, including lexical features allows         2.2     Classification Models
us to methodically capture this property for classification purposes,       We use the features described in the previous section to encode
and perhaps infer patterns in malicious URLs that we would other-        individual URLs as very high dimensional feature vectors. Most
wise miss through ad-hoc inspection.                                     of the features are generated by the “bag-of-words" representation
   For the purpose of this discussion, we want to distinguish the        of the URL, registrar name, and registrant name; binary features
two parts of a URL: the hostname and the path. As an example,            are also used to encode all possible ASes, prefixes and geographic
with the URL www.geocities.com/usr/index.html, the                       locales of an IP address. The resulting URL descriptors typically
hostname portion is www.geocities.com and the path portion               have tens of thousands of binary features.
is usr/index.html.                                                          The high dimensionality of these feature vectors poses certain
   Lexical features are the textual properties of the URL itself (not    challenges for classification. Though only a subset of the generated
the content of the page it references). We use a combination of          features may correlate with malicious Web sites, we do not know
features suggested by the studies of McGrath and Gupta [16] and          in advance which features are relevant. More generally, when there
Kolari et al. [13]. These properties include the length of the host-     are more features than labeled examples, we enter the regime in
name, the length of the entire URL, as well as the number of dots in     which statistical models are most prone to overfitting.
the URL — all of these are real-valued features. Additionally, we           In this section, we briefly review the models that we studied for
create a binary feature for each token in the hostname (delimited        classification, as well as their relative strengths and weaknesses.
by ‘.’) and in the path URL (strings delimited by ‘/’, ‘?’, ‘.’, ‘=’,    For all these models, we adopt the following notation. We use n
‘-’ and ‘_’). This is also known as a “bag-of-words.” Although we        to denote the number of labeled examples in the training set and d
do not preserve the order of the tokens, we do make a distinction        to denote the dimensionality of the feature space (i.e., the number
between tokens belonging to the hostname, the path, the top-level        of features). We use x ∈ ℜd to denote a feature vector and xj to
domain (TLD) and primary domain name (the domain name given              denote its jth component. Finally, we use y ∈ {0, 1} to denote the
to a registrar). More sophisticated techniques for modeling lexical      label of an example, with y = 1 for malicious sites and y = 0 for
features are available, such as Markov models of text. However,          benign ones.
even with the bag-of-words representation, we can achieve very              We provide further details on the individual classifiers below. In
accurate classification results.                                         particular, for each classifier, we briefly describe its testing process
   Host-based features: The reason for using host-based features         (how it predicts a label from the features of URLs) and its training
is that malicious Web sites may be hosted in less reputable host-        process (how it learns the decision rule to predict these labels).
ing centers, on machines that are not conventional web hosts, or            Naive Bayes: Commonly used in spam filters, this basic model
through disreputable registrars. To an approximate degree, host-         assumes that for a given label, the individual features of URLs are
distributed independently of the values of other features [5]. Let-       eq. (3) is thresholded to obtain a binary prediction for the label of
ting P (x|y) denote the conditional probability of the feature vec-       the feature vector x.
tor given its label, the model assumes P (x|y) = dj=1 P (xj |y).             We trained models for logistic regression using a regularized
                                                     Q
Then, from Bayes rule, assuming that malicious and benign Web             form of maximum likelihood estimation. Specifically, we chose
sites occur with equal probability, we compute the posterior proba-       the weight vector w and bias b to maximize the objective function:
bility that the feature vector x belongs to a malicious URL as:                                  n                            d
                                                                                                 X                            X
                                 P (x|y = 1)                                         L(w, b) =          log P (yi |xi ) − γ         |wα |.   (4)
            P (y = 1|x) =                           .              (1)
                          P (x|y = 1) + P (x|y = 0)                                               i=1                         α=1

Finally, the right hand side of eq. (1) can be thresholded to predict     The first term computes the conditional log-likelihood that the model
a binary label for the feature vector x.                                  correctly labels all the examples in the training set. The second
   A Naive Bayes classifier is most easily trained by computing the       term in eq. (4) penalizes large magnitude values of the elements in
conditional probabilities P (xj |y) from their maximum likelihood         the weight vector w. Known as ℓ1 -norm regularization, this penalty
estimates [5]. For real-valued features, we model P (xj |y) by a          not only serves as a measure against overfitting, but also encourages
Gaussian distribution whose mean and standard deviation are com-          sparse solutions in which many elements of the weight vector are
puted over the jth component of feature vectors in the training set       exactly zero. Such solutions emerge naturally in domains where the
with label y. For binary-valued features, we estimate P (xj = 1|y)        feature vectors contain a large number of irrelevant features. The
as the fraction of feature vectors in the training set with label y for   relative weight of the second term in eq. (4) is determined by the
which the jth component is one.                                           regularization parameter. We selected the value of γ in our experi-
   The model parameters in the Naive Bayes classifier are estimated       ments by cross validation.
to maximize the joint log-likelihood of URL features and labels,             We included ℓ1 -regularized logistic regression for its potential
as opposed to the accuracy of classification. Optimizing the lat-         advantages over Naive Bayes and SVMs in our particular domain.
ter typically leads to more accurate classifiers, notwithstanding the     Unlike Naive Bayes classification, the parameters in logistic regres-
increased risk of overfitting.                                            sion are estimated by optimizing an objective function that closely
   Support Vector Machine (SVM): SVMs are widely regarded                 tracks the error rate. Unlike SVMs, ℓ1 -regularized logistic regres-
as state-of-the-art models for binary classification of high dimen-       sion is especially well suited to domains with large numbers of ir-
sional data. SVMs are trained to maximize the margin of cor-              relevant features. (In large numbers, such features can drown out
rect classification, and the resulting decision boundaries are robust     the similarities between related examples that SVMs expect to be
to slight perturbations of the feature vectors, thereby providing a       measured by the kernel function.) Finally, because ℓ1 -regularized
hedge against overfitting. The superior generalization abilities of       logistic regression encourages sparse solutions, the resulting mod-
SVMs have been borne out by both theoretical studies and experi-          els often have decision rules that are easier to interpret in terms of
mental successes [23].                                                    relevant and irrelevant features.
   The decision rule in SVMs is expressed in terms of a kernel
function K(x, x′ ) that computes the similarity between two feature       3.    DATA SETS
vectors and non-negative coefficients {αi }n  i=1 that indicate which         This section describes the data sets that we use for our evalu-
training examples lie close to the decision boundary. SVMs classify       ation. For benign URLs, we used two data sources. One is the
new examples by computing their (signed) distance to the decision         DMOZ Open Directory Project [19]. DMOZ is a directory whose
boundary. Up to a constant, this distance is given by:                    entries are vetted manually by editors. The editors themselves
                          n
                          X                                               go through a vetting process, with editors given responsibility for
                 h(x) =         αi (2yi −1)K(xi , x),              (2)    larger portions of the directory as they gain trust and experience.
                          i=1                                             The second source of benign URLs was the random URL selector
where the sum is over all training examples. The sign of this dis-        for Yahoo’s directory. A sample of this directory can be generated
tance indicates the side of the decision boundary on which the ex-        by visiting http://random.yahoo.com/bin/ryl.
ample lies. In practice, the value of h(x) is thresholded to predict          We also drew from two sources for URLs to malicious sites:
a binary label for the feature vector x.                                  PhishTank [21] and Spamscatter [3]. PhishTank is a blacklist of
   SVMs are trained by first specifying a kernel function K(x, x′ )       phishing URLs consisting of manually-verified user contributions.
and then computing the coefficients αi that maximize the margin           Spamscatter is a spam collection infrastructure from which we ex-
of correct classification on the training set. The required optimiza-     tract URLs from the bodies of those messages. Note that the ma-
tion can be formulated as an instance of quadratic programming, a         licious URL data sets have different scopes. PhishTank focuses on
problem for which many efficient solvers have been developed [6].         phishing URLs, while Spamscatter includes URLs for a wide range
In our study, we experimented with both linear and radial basis           of scams advertised in email spam (phishing, pharmaceuticals, soft-
function (RBF) kernels.                                                   ware, etc.). Both sources include URLs crafted to evade automated
   Logistic Regression: This is a simple parametric model for bi-         filters, while phishing URLs in particular may be crafted to visually
nary classification where examples are classified based on their dis-     trick users as well.
tance from a hyperplane decision boundary [11]. The decision rule             Table 1 summarizes the number and types of features in the
is expressed in terms of the sigmoid function σ(z) = [1 + e−z ]−1 ,       data sets that we use in our evaluations. The four data sets con-
which converts these distances into probabilities that feature vec-       sist of pairing 15,000 URLs from a benign source (either Yahoo or
tors have positive or negative labels. The conditional probability        DMOZ) with URLs from a malicious source (5,500 from Phish-
that feature vector x has a positive label y = 1 is the following:        Tank and 15,000 from Spamscatter). We refer to these sets as the
                                                                          DMOZ-PhishTank (DP), DMOZ-Spamscatter (DS), Yahoo-Phish-
                   P (y = 1|x) = σ(w·x + b),                       (3)
                                                                          Tank (YP), and Yahoo-Spamscatter (YS) sets. We collected URL
                                 d
where the weight vector w ∈ ℜ and scalar bias b are parameters to         features between August 22, 2008 – September 1, 2008.
be estimated from training data. In practice, the right hand side of          We normalized the real-valued features in each feature set to
       Feature type             DP       YP       DS       YS
       DNS NS record          10818     4186    10838     3764                                             Basic
       WHOIS info              9070     3959     8702     3525
       Hostname                7928     3112     7797     2959                                            Botnet
       TLD + domain            5787     2186     5609     1980
       DNS A record            5627     2470     5207     1862                                          Blacklist
       Geographic              4554     2250     4430     1836
       Path tokens             4363     8021     8390    11974                                   Blacklist+Botnet



                                                                              Feature Sets
       Last token of path      1568     3751     2892     5071
       DNS MX record           1323      497     1422      559                                           WHOIS
       TLD                      319      284      102       51
       Connection speed          31       29       30       30                                      Host−based
       DNS TTL                   14       13       14       12
       Blacklists                 7        7        7        7
                                                                                                          Lexical
       WHOIS dates                6        6        6        6
       Spamassassin plugin        5        5        5        5
                                                                                                             Full
       IP address misc.           4        4        4        4                                                                             DMOZ−PhishTank
                                                                                                                                           Yahoo−PhishTank
       Lexical misc.              3        3        3        3                                                                             DMOZ−Spamscatter
       All features           51427    30783    55458    33648                               w/o WHOIS/Blacklist
                                                                                                                                           Yahoo−Spamscatter
                                                                                                                0   5   10     15     20      25     30    35
Table 1: Breakdown of feature types for data sets used in eval-                                                              Error rate (%)
uations.
lie between zero and one, shifting and rescaling each real-valued         Figure 1: Error rates for LR with nine features sets on each of
feature so that zero and one corresponded to the minimum and              the four URL data sets. Overall, using more features improves
maximum values observed in the training set. Values outside this          classification accuracy.
range in the testing set were clipped to zero or one as appropriate.
The normalization served to equalize the range of the features in
each feature set, both real-valued and binary. Intuitively, it reflects   implemented a custom optimizer for ℓ1 -regularized logistic regres-
our prior belief that the real-valued and binary-valued features are      sion in MATLAB using multiplicative updates [24].
equally informative and therefore should be calibrated on the same
measurement scales.
                                                                          4.2                 Feature Comparison
   One further complication arises due to undefined, or missing,             Our first experiments on URL classification were designed to
features. Many real-valued features are undefined for large num-          explore the potential benefits of considering large numbers of auto-
bers of examples in the data set (e.g., DNS time-to-live values).         matically generated features as opposed to small numbers of man-
We handled missing values using the following heuristic: for each         ually chosen features. Fig. 1 compares the classification error rates
real-valued feature, we defined an extra binary feature indicating        on the four data sets described in Section 3 using nine different
whether the feature was defined. This heuristic enables the classi-       sets of features. The different feature sets involve combinations of
fiers to learn how to treat missing features from the way they appear     features reflecting various practical considerations. For brevity, we
in the training set.                                                      only show detailed results from ℓ1 -regularized logistic regression
                                                                          (LR), which yields both low error rates (see Section 4.3) and also
                                                                          highly interpretable models (see Section 4.6). However, the other
4. EVALUATION                                                             classifiers also produce qualitatively similar results.
   In this section, we evaluate the effectiveness of the classifiers         Table 2 shows the total number of features in each feature set for
on identifying URLs to malicious sites. Specifically, we want to          the Yahoo-PhishTank data set. For these experiments, we also re-
answer the following questions: Does using more features lead to          port the number of relevant features that received non-zero weight
more accurate classification? What is the most appropriate classifi-      from ℓ1 -regularized logistic regression. (The other three data sets
cation model to use? What is the impact on accuracy if we tune the        yield qualitatively similar results.) The feature sets are listed in as-
classifier for lower false positives? Can we effectively classify data    cending order of the total number of features. In what follows, we
from once source with a model that was trained on a different data        describe the detailed compositions of these feature sets, as well as
source? What trends do we see among the relevant features? And            their effects on classification accuracy.
what do the misclassified examples have in common?                           We start with a “Basic” feature set that corresponds to the set of
                                                                          URL-related (not content-related) heuristics commonly chosen by
4.1 Methodology                                                           various anti-phishing studies, including Fette et al. [8], Bergholz
   We start by describing our experimental methodology. For each          et al. [4] and CANTINA [28]. This set consists of four features:
feature set and data set in our experiments, we perform classifi-         the number of dots in a URL, whether the hostname contains an IP
cation over 10 random splits of the data, and the splits are 50/50        address, the WHOIS registration date (a real-valued feature), and
between training and testing. We learn a decision threshold t that        an indicator variable for whether the registration date is defined.
will minimize the overall classification error (see Section 2.2 for       Under this scheme, the classifier achieves a 10% error rate.
methodology of learning t). We show the average classification               The next three feature sets (Botnet, Blacklist, and Blacklist +
results of those 10 splits.                                               Botnet) represent the use of current spam-fighting techniques for
   We ran our experiments on a machine with 2 dual-core 2.33 GHz          predicting malicious URLs. The features for “Botnet” are from the
Xeon processors with 4 GB memory. Memory exhaustion was                   SpamAssassin Botnet plugin (Section 2.1). These consist of five
not an issue, but typical usage was on the order of a few hundred         binary features indicating the presence of certain client- or server-
megabytes. We implemented a Naive Bayes solver in MATLAB.                 specific keywords, whether the hostname contains an IP address,
For the SVM solvers, we used the .mex implementations of LIB-             and two more features involving the PTR record of the host. With
SVM [6] and LIBLINEAR [7] that interface with MATLAB. We                  these features, we obtain a 15% error rate at best.
                                     # Features                                               6
                                                                                                                                                             Bayes
         Feature set              Total Relevant       Err%                                                                                                  SVM−lin
                                                                                              5                                                              SVM−rbf
         Basic                        4           4    10.12                                                                                                 LR
         Botnet                       5           5    15.34                                  4



                                                                             Error rate (%)
         Blacklist                    7           6    19.92
         Blacklist+Botnet           12           10    12.14                                  3
         WHOIS                    3967          727     3.22
         Host-based              13386        1666      2.26                                  2
         Lexical                 17211        4488      1.93
         Full                    30597        3891      1.24                                  1
         w/o WHOIS/Blacklist     26623        2178      1.48
                                                                                              0
                                                                                                  DMOZ−PhishTank   Yahoo−PhishTank DMOZ−Spamscatter Yahoo−Spamscatter
                                                                                                                               Data Sets
Table 2: Total and relevant number of features for the LR clas-
sifier and the Yahoo-PhishTank data set. We reference the over-
all error rate from Figure 1.                                             Figure 2: Overall error rates for the “Full” feature set using
                                                                          different classification algorithms and data sets. The SVM and
                                                                          LR classifiers do well with large feature sets such as “Full” as
   The “Blacklist” feature set consists of binary variables for mem-      compared to Naive Bayes.
bership in six blacklists (and one white list) run by SORBS, URIBL,
SURBL, and Spamhaus. These lists combined provide at best a
20% error rate. When we combine the blacklists with the SpamAs-              Finally, blacklists and WHOIS information provide us known
sassin Botnet plugin features in the “Blacklist+Botnet” feature set,      high-quality information for classifying URLs. But do the low clas-
the error rate improves to 12%.                                           sification error rates of the “Full” feature set critically depend on
   These small features sets are reasonably effective, but including      those particular features? The results for “w/o WHOIS/Blacklist”
features that result in very large feature sets significantly improve     show the error rates without WHOIS and blacklist features are 1.1–
classification accuracy.                                                  3.4%. Since the error rates across this feature set remain close to
   The WHOIS registration date was a popular feature in previous          those of the “Full” set, we find that the remaining features provide
studies [8][4][28]. Inspired by these examples, we also create a          sufficient information for achieving accurate classification results
WHOIS feature set that includes the registration date as well addi-       — using high-quality WHOIS and blacklist information is not nec-
tional WHOIS properties. The “WHOIS” set we evaluated includes            essary for excellent results.
the registration, update, and expiration dates, as well as a bag-of-         Having reviewed the composition and results for each feature
words representation of the registrar and registrant. The feature set     set, we point out two noticeable trends: (1) Before “WHOIS,” the
grows to nearly 4,000 on the YP data set (Table 2.1), while reduc-        DMOZ-PhishTank set has similar error rates to Yahoo-PhishTank,
ing the error to 3–15% across the data sets. WHOIS features can           but starting with “WHOIS” the Yahoo-trained set has lower error.
provide useful information for differentiating malicious and benign       This happens because the distribution of WHOIS and lexical fea-
URLs, since creators of legitimate sites are likely to adopt different    tures between DMOZ and Yahoo are distinct enough that certain
registration patterns than creators of malicious sites (whose sites       URL tokens and certain WHOIS properties receive more weight in
are taken down more frequently, or who want to anonymize their            one data set than the other. (2) Prior to “WHOIS,” the PhishTank-
registration information).                                                trained sets have better error rates because PhishTank Web sites
   WHOIS features, however, are just a subset of the available host-      have more undefined WHOIS registration dates (hence better “Ba-
based features. If we also include the remainder of our host-based        sic” results), more IP addresses in the hostname (better “Botnet”
features — IP, DNS, and geographic properties as well as mem-             results), and more URLs in blacklists (better “Blacklist” results).
bership in a blacklist and SpamAssassin Botnet plugin features —          But starting with “WHOIS,” the Spamscatter-trained sets do bet-
the feature sets from our data sets grow to over 10,000 features.         ter because the introduction of registrar/registrant names provides
And these additional features are beneficial: the LR classifier has       a wealth of information for differentiating malicious and benign
an even lower error rate of 1.2–6%. Using host-based features pro-        sites, and the distribution of lexical tokens in Spamscatter is dis-
vides a richer set of properties for the classifier to capture phenom-    tinct enough from benign sources to provide traction for accurate
ena such as malicious sites being hosted from a particular IP prefix,     classification results.
ISP, country and the like.                                                   These results demonstrate that different data sets provide differ-
   In addition to host-based features, we can also use lexical prop-      ent feature distributions for distinguishing malicious and benign
erties of the URLs themselves as potentially strong features (again,      URLs. Rather than manually discovering and adjusting the deci-
we are not considering the Web page content). That is, if the URLs        sion rules for different data sets, machine learning techniques can
themselves “look” malicious, then a lexical feature set can help us       adapt to these differing distributions by learning the appropriate de-
automatically learn the tell-tale strings that characterize a malicious   cision rules automatically.
or benign URL. For example, malicious sites may have legitimate-
looking tokens appended to the sub-domain of the hostname, or             4.3                     Classifier Comparison
have them embedded in the path of the URL for obfuscation pur-               Next we compare the accuracy of the four classifiers described
poses. In the “Lexical” set, we consider a “bag-of-words” represen-       in Section 2, each of which has different tradeoffs between model
tation of the tokens in the URL augmented with additional proper-         complexity and training execution time. We use the “Full” feature
ties such as the length of the hostname, the length of the entire URL     set from Section 4.2 for comparison.
and the number of dots. This is indeed an effective feature set, with        Figure 2 compares the results of the classifiers: Naive Bayes
a resulting error is between 1.9% and 3.5%.                               (Bayes), SVM with a linear kernel (SVM-lin), SVM with an RBF
   Combining the host-based and lexical features into the “Full”          kernel (SVM-rbf), and ℓ1 -regularized logistic regression (LR). The
feature set, we obtain the lowest classification error among all fea-     bars show the overall error rates for each classifier on each of our
ture sets at 0.9–3.0%. For YP, the FP/FN rates are 0.8%/2.6%.             four data sets. Additionally, Table 3 shows the training and testing
                                                                                                         100
times for each classifier. The cross-validation time refers specifi-
cally to the cross-validation process used in LR to choose a regu-                                                 ↑
larization constant. Although expensive, it can be effectively amor-                                      80       FP = 0.1%




                                                                                True positive rate (%)
                                                                                                                   FN = 7.6%
tized when training over time.
                                                                                                          60
                         Bayes     SVM-lin     SVM-rbf         LR                                                                                              Full
                                                                                                                                                               Blacklist
      Train               51 s        50 s        63 s      30 min                                        40
                                                                                                                   FP = 0.1%
      Test                87 s        91 s       170 s         90 s                                                FN = 74.3%
      Cross-validation      —          —            —         5 hrs                                                ↓
                                                                                                          20

Table 3: Training and testing times for the Yahoo-PhishTank
data set.                                                                                                  0
                                                                                                               0       0.2         0.4         0.6       0.8               1
                                                                                                                               False positive rate (%)
    The SVM and LR classifiers have at least half of the error of
Naive Bayes, which is not surprising given the models that we               Figure 3: Tradeoff between false positives and false negatives:
chose. Naive Bayes is a classifier that sees wide-spread use in spam        ROC graph for LR over an instance of the Yahoo-PhishTank
filters and related security applications, in part because the training     data set using (1) the “Full” feature set and (2) blacklists only.
and testing performance of Naive Bayes is fast. However, the bene-          We highlight the points where the false positives are tuned to
fit of reduced training time is outweighed in this case by the benefit      0.1%.
of using classifiers whose explicit goal is to minimize errors. This
tradeoff is particularly worthwhile when we are dealing with a large                                                                              Testing
feature set.                                                                           Training                                      YS          YP        DS             DP
    As mentioned in Section 2.2, SVM classifiers can perform poorly                    YS                                         0.90%       5.66%     9.38%         17.73%
if irrelevant features in a large feature set make the kernel functions                YP                                        21.33%       1.24% 44.02%            33.54%
poor measures of similarity. Given that the difference in error be-                    DS                                        13.55%      31.57%     1.00%         13.70%
                                                                                       DP                                        22.95%       3.06% 22.92%             3.01%
tween SVM and LR is so small, though, this problem did not mate-
rialize in our data set. As such, we continue to show the results for                  All sources (YDSP)                         0.95%       2.40%     1.43%          3.19%
LR for the remaining experiments because of its interpretability,
which is useful for understanding how the model performs (Sec-              Table 4: Overall error rates when training on one data source
tion 4.6) and how it might be improved (Section 4.7).                       and testing on another (possibly different) data source using the
                                                                            LR classifier.
4.4 Tuning False Positives & Negatives                                      regression. Each row represents a pairing of benign and malicious
   An advantage of using these models is that they can tradeoff false       URL training sources, and each column represents a pairing of be-
positives and false negatives. We have seen in the previous sections        nign and malicious URL testing sources. As expected, the entries
that classifying with the “Full” feature set yields very low overall        along the NW-SE diagonal yield the lowest classification errors be-
error rates. For policy reasons, however, we may not want to choose         cause they represent matching similar data sources (these numbers
a decision threshold t to minimize the overall error rate. Instead,         are repeated from previous experiments). When only the benign
we may want to tune the threshold to have very low false positives          URL source is mismatched (e.g., YS and DS), error increases due
at the expense of more false negatives (or vice versa).                     to higher false positives. And if only the malicious URL source is
   Consider blacklisting as a motivating example. Blacklisting has          mismatched (e.g., YS and YP), error increases due to higher false
the intrinsic advantage that it will have very low false positives.         negatives. Nevertheless, we note that training on the Spamscatter
Suppose a network administrator wants to take advantage of the              set — which includes URLs advertising a wide range of scam sites
benefits of classifying over a full feature set while maintaining the       — generally performs better on a mismatched source (YS-YP at
low false positives of a blacklist-only policy as applied to URL            6%, DS-DP at 14%) than the PhishTank set (YP-YS at 21%, DP-
classification. To do so, we can select a threshold for the full feature    DS at 23%) — which focuses only on phishing sites (Section 3).
set that will yield a false positive rate competitive with blacklisting        Finally, if the training and testing sources are completely mis-
while having a much lower false negative rates.                             matched (SW-NE diagonal), the error ranges between 18–44%. Al-
   Figure 3 shows the results of this experiment as an ROC graph            though better than random, the disparity in accuracy emphasizes
with respect to the decision threshold t. We see that even if we            the judicious selection of training data. This selection is of partic-
tune the decision threshold of the full-featured classifier to the same     ular importance for use in a deployment: the training data should
false positives as a blacklist-only classifier, the full-featured classi-   reflect the “testing” environment in which the system is used. To
fier still predicts malicious URLs with much better accuracy than           that end, if we use all four training sources, the generalization abil-
with blacklists alone.                                                      ity of the classifier is strong across all testing sources (last row).
                                                                            Thus, in a deployed system practitioners will want to pay special
4.5 Mismatched Data Sources                                                 attention to collecting training data that is representative.
   Coupling the appropriate classifier with a large feature set can
yield a highly accurate classifier when trained and tested on disjoint      4.6                           Model Analysis
sets of the same data source. But do these results hold when train-            Besides high classification accuracy, LR has the advantages of
ing and testing examples are drawn from different data sources?             performing automatic feature selection as well as providing an in-
To answer this question, we experiment with training and testing            terpretable linear model of the training data. In particular, because
on various combinations of benign and malicious sources of URLs.            the output of a linear model depends on the weighted sum of the
(We use the abbreviations defined in Section 3 to refer to each com-        features, the sign and magnitude of the individual parameter vector
bination of data sources, e.g. YP for Yahoo-PhishTank).                     coefficients can tell us how individual features contribute to a “ma-
   Table 4 shows classification results using ℓ1 -regularized logistic      licious” prediction or a “benign” prediction. Positive coefficients
                                                    Sign(coeff.)          level TLDs like ‘.ca’ and ‘.se’ are the top-6 benign features by
      Feature type             Total   Relevant        -       +
                                                                          weight. By contrast, ‘.info’, ‘.kr’, ‘.it’, ‘.hu’, and ‘.es’ form the
      Path tokens              8021         419       95    324           top-6 malicious features by weight, domains correlated with some
      DNS NS record            4186        1021     468     553
      WHOIS info               3959         633     284     349           malicious sites.
      Last token of path       3751          76       17     59              For hostname tokens that do not include the TLD, the model
      Hostname                 3112         315     123     192           selected 315 out of 3,112 features. Some interesting examples of
      DNS A record             2470         407     173     234           malicious hostname tokens include the appearance of ‘com’ in the
      Geographic               2250         692     329     363           name, resulting from URLs to malicious sites that prepend the to-
      TLD + domain             2186         237       97    140           kens of legitimate hostnames (e.g., from banks) as an obfuscation
      DNS MX record             497          70       34     36
      TLD                       284          40       15     25           measure. And while it is no surprise to see that the presence of
      Connection speed           29          19        9     10           ‘www’ is considered a benign (albeit spoofable) feature, ‘members’
      DNS TTL                    13           9        6       3          receives the most negative weight because of its co-occurrence with
      Blacklists                   7          7        1       6          benign sites hosted on ‘members.aol.com’, ‘members.tripod.com’
      WHOIS dates                  6          6        1       5          and other free hosting services.
      Spamassassin plugin          5          5        3       2             Although the URL’s path tokens contribute the most features
      IP address misc.             4          3        3       0
      Lexical misc.                3          3        0       3          (about 8,000), the model selects 419 relevant path tokens during
      All feature types       30783        3962    1658 2304              training, 324 of which are malicious. Among the automatically se-
                                                                          lected malicious features are ‘account’, ‘webscr’, ‘login’, ‘ebayis-
Table 5: Breakdown of features for LR for an instance of the              api’, ‘signin’, ‘banking’, and ‘confirm’ — for comparison, these
Yahoo-PhishTank data set. We show total number of features                tokens the model learned automatically are also 7 out of the 8
in each feature type category, the number of relevant (non-zero)          manually chosen path tokens that were considered tell-tale phish-
features, as well as the number of selected features with nega-           ing features in the study by Garera et al. [9]. Additional selected
tive (benign) and positive (malicious) coefficients.                      path tokens include ‘images’, ‘com’, ‘www’, and ‘paypal’. To-
                                                                          kens like these indicate attempts by the site authors to spoof le-
                                                                          gitimate sites by including domain names within the URL path
correspond with malicious features while negative coefficients cor-
                                                                          (‘www.example.com’). Even ‘http:’ is considered a malicious fea-
respond with benign features. Additionally, the training phase for
                                                                          ture, aiding in the prediction of URLs that attempt to hide legitimate-
ℓ1 -regularized logistic regression yields a sparse parameter vec-
                                                                          looking URLs in the path component (the case of using a legitimate
tor w where the number of non-zero coefficients is much smaller
                                                                          domain’s redirection facilities was more rare).
than the total number of features, making it easier for us to focus
                                                                             Out of the 3,959 WHOIS information features (mainly tokens
on a smaller number of relevant features (a zero coefficient means
                                                                          in the names of the registrar and registrant of the domain name),
that the corresponding feature will not contribute to the prediction
                                                                          the classifier selected 633 as relevant. As for dates in the WHOIS
outcome). These properties simplify the analysis of trends in mali-
                                                                          record, missing any of the three WHOIS dates (registration, update,
cious and benign features.
                                                                          expiration) is considered a malicious feature. Moreover, having a
   In this section, we analyze the model that LR constructed for
                                                                          recent registration or update date is considered malicious — this
one of the ten random splits of the Yahoo-PhishTank data set, and
                                                                          corresponds with the behavior of malicious sites having newer reg-
discuss trends that we observe in the model and correlate them with
                                                                          istration dates because they are taken down more frequently. Hav-
real-world phenomena. This example has 3,962 active (non-zero)
                                                                          ing a newer expiration date is the only benign WHOIS date feature.
features out of a total of 30,783 features. There are 1,658 negative
                                                                             For connection speed features, the model selected 19 out of 29.
(benign) coefficients and 2,304 positive (malicious) coefficients in
                                                                          Having a T1 speed for the DNS A and MX records are the top-2 be-
the weight vector. Table 5 shows a breakdown of the different types
                                                                          nign features, while a residential cable, DSL or satellite connection
of features selected in this LR model. Overall, we find that the
                                                                          hosting an address in the A, MX or NS record of an entry is consid-
automatic feature selection of certain feature groups are specific
                                                                          ered malicious. These features reflect the phenomenon of malicious
to the data set at hand, while other feature types indicate broader
                                                                          sites hosted on compromised machines in residential ISPs.
trends in malicious sites.
                                                                             Finally, the selected DNS A/MX/NS record features and geo-
   Not surprisingly, the weights learned for the blacklist features are
                                                                          graphic features correspond to hosting activity associated with var-
all non-zero because the presence of domain names and IPs in one
                                                                          ious regions of the Internet address space. For example, IP ranges
of the six blacklists, or its absence in the whitelist, is an accurate
                                                                          belonging to Google, Yahoo and AOL are treated as benign fea-
indicator of maliciousness.
                                                                          tures. Also, having DNS NS records in IP ranges belonging to
   Additionally, the weights selected for the “SpamAssassin plu-
                                                                          major registrars such as RIPE (the network information center for
gin” features match their purpose as an indicator. For example, the
                                                                          Europe) are considered benign features. However, having an NS
model learned negative weights for having server words in the host-
                                                                          record in one of the IP prefixes run by GoDaddy is considered a
name, PTR records, and full-circle records. And it learned positive
                                                                          malicious feature.
weights for having client words or an IP address in the hostname.
                                                                             Overall, the classifier was able to automatically select malicious
   The miscellaneous IP address properties indicate whether the A
                                                                          and benign features for which domain experts had prior intuition.
record of a host shares the same prefix or AS as its MX or NS
                                                                          Perhaps more importantly, the classifier automatically selected new,
record. These features are considered benign on all counts, which
                                                                          non-obvious features that were highly predictive and yielded addi-
makes sense because hosting the Web site and email server in the
                                                                          tional, substantial performance improvements.
same data center reflects a more orthodox hosting setup that a cus-
                                                                             When faced with such a reputation system, the ability of adver-
tomer would purchase through legitimate means, as opposed to a
                                                                          saries to evade detection depends on their ability to avoid conform-
hosting infrastructure acquired by compromising a scattered array
                                                                          ing to the trends and selections determined by the classifier; we
of machines.
                                                                          discuss the issue further in Section 5.
   Next, the classifier selected 40 out of the 284 top-level TLD fea-
tures. Generic TLDs like ‘.gov’, ‘.edu’, ‘.com’, ‘.org’, and country-
4.7 Error Analysis                                                       URLs. An adversary evading detection can strive to craft a site that
   A machine learning approach using a full feature set, consist-        can spoof sufficient benign features with high weights, and avoid
ing of both lexical and host-based features, yields a URL repu-          acquiring sufficient malicious features, to appear as a benign site
tation classifier that has low false positive and low false negative     to the detection algorithm. For instance, ‘.org’ is a benign TLD
rates. Despite the high accuracy of this approach, in this section       according to our classifier, yet costs only $10/year to register.
we examine examples of misclassified URLs to reveal trends that             There are, however, functional and cost-related limits to an ad-
can suggest refinements to our current approach, and also help us        versary’s ability to forge features. For instance, older WHOIS reg-
understand the limitations of using just URL features for classi-        istration dates have high weights as benign features, but are difficult
fication (as opposed to additionally including features from Web         to casually obtain for malicious sites. Further, whether a site ap-
page content). For our analysis, we examine one of the instances of      pears on a blacklist is out of its control. Whether such features are
the Yahoo-PhishTank data sets from the LR classifier, chosen ran-        sufficient in the long term for differentiating benign from malicious
domly, which had 60 false positives and 71 false negatives (other        sites remains an open question. As with any security approach fac-
instances resulted in similar conclusions).                              ing an adaptable adversary, though, we envision having to evolve a
   The most common cause of false positives (benign sites mistak-        deployment in response over time.
enly classified as malicious) was due to sites hosted at disreputable
ISPs. This is a case of guilt by association — our classifier penal-     6.    RELATED WORK
izes legitimate sites hosted in the same AS or IP prefix as malicious       This section surveys related approaches in URL classification,
sites. This set of false positives could be mitigated by examin-         related applications of statistical modeling, and other techniques.
ing the content of a site. Also, if reputation systems become more
prevalent, such false positives imply that it would behoove legiti-      6.1    Classification of URLs
mate site owners to seek service providers that have a reputation           The work by Garera et al. is the most closely related to our
for cracking down on malicious sites hosted on their networks.           study [9]. They use logistic regression over 18 hand-selected fea-
   The false negatives (undetected malicious sites) fell into the fol-   tures to classify phishing URLs. The features include the presence
lowing categories: (1) URLs to sites with benign tokens in the           red flag key words in the URL, features based on Google’s Page
URL, (2) malicious sites using free hosting services (such as ‘geoc-     Rank and Google’s Web page quality guidelines. Although a di-
ities.com’ or ‘tripod.com’), (3) compromised sites, (4) redirection      rect comparison with our approach is difficult without access to the
services, (5) sites hosted in reputable geographic regions (such as      same URLs or features, they achieve a classification accuracy of
the US), and (6) sites with international TLDs hosted in the US. The     97.3% over a set of 2,500 URLs. Though similar in motivation and
first category shows that if certain lexical features have substantial   methodology, our approach differs significantly in both scope (aim-
weights in classification, false negatives can result. Eliminating       ing to detect other types of malicious activity) and scale (consider-
this false negative could be accomplished by more careful vetting        ing an order-of-magnitude more features and training examples).
of which URL tokens become features.                                        McGrath and Gupta do not construct a classifier but neverthe-
   The last five categories are related because they reflect the situ-   less perform a comparative analysis of phishing and non-phishing
ation where host-based features of a malicious URL appear to be          URLs [16]. For examples, they compare non-phishing URLs drawn
non-malicious. However, the remedies for mitigating these kinds          from the DMOZ Open Directory Project [19] to phishing URLs
of false negatives differ. For example, eliminating malicious sites      from PhishTank [21] and a non-public source. The features they
in category (3) is a matter of encouraging sites to maintain up-to-      analyze include IP addresses, WHOIS thin records (containing date
date patches of software, and category (5) reflects the practice of      and registrar-provided information only), geographic information,
purchasing services from a reputable ISP; such remedies, though,         and lexical features of the URL (length, character distribution, and
are out of the control of a reputation system. However, the closely-     presence of pre-defined brand names). We build on their work by
related category (6) sites could be addressed by adding features         incorporating similar sources and features into our approach.
that indicate whether the country of a site’s TLD corresponds to            Provos et al. perform a study of drive-by exploit URLs and use
the country where the ISP is located. Finally, mitigating false neg-     a patented machine learning algorithm as a pre-filter for VM-based
atives in (2) and (4) may require examining the content of a site.       analysis [22]. Unlike our approach, they extract content-based fea-
                                                                         tures from the page, including whether IFrames are “out of place,”
                                                                         the presence of obfuscated javascript, and whether IFrames point
5. EVASION                                                               to known exploit sites.
   If deployed and effective, adversaries will naturally try to in-         CANTINA classifies phishing URLs by thresholding a weighted
vent methods for evading this statistical modeling approach. As          sum of 8 features (4 content-related, 3 lexical, and 1 WHOIS-
we discussed in Section 2, the effectiveness of statistical modeling     related) [28]. Among the lexical features, it looks at dots in the
depends on a few key assumptions. When hosting malicious sites,          URL, whether certain characters are present, and whether the URL
adversaries can try to violate these assumptions to evade detection.     contains an IP address. The WHOIS-related feature CANTINA ex-
   One approach for evasion is to reduce the information content in      amines is the age of the domain. We use similar features in our
the lexical features of URLs. For example, using redirection ser-        approach, but entirely different models of classification.
vices such as TinyURL produces fewer distinguishing lexical fea-
tures (e.g., an identical host name coupled with a random token).        6.2    Machine Learning in Related Contexts
Another approach is to acquire many benign host-based features,             Fette et al. use statistical methods in machine learning to clas-
such as hiding behind the well-provisioned infrastructure provided       sify phishing emails [8]. Their classifiers examine the properties
by legitimate free hosting services (e.g., ‘geocities.com’ or ‘tri-      of URLs contained within a message (e.g., the number of URLs,
pod.com’). These aliasing techniques could provide adversaries           number of domains, and number of dots in a URL), but unlike our
methods for misclassifying their sites as benign.                        approach they also consider features of the email structure and con-
   As discussed in Section 4.6, the weights assigned to features in      tent. Bergholz et al. further improve the accuracy of Fette et al.
our classifiers reflect trends in the features of malicious and benign   by introducing models of text classification to analyze email con-
tent [4]. Abu-Nimeh et al. compare different classifiers over a cor-         [5] C. M. Bishop. Pattern Recognition and Machine Learning. Springer
pus of phishing emails, using as features the frequency of the 43                Publishing Company, New York, NY, 2006.
most-popular words in the corpus [1].                                        [6] C.-C. Chang and C.-J. Lin. LIBSVM: A Library for Support Vector
   Kolari et al. use URLs found within a blog page as features to de-            Machines. http://www.csie.ntu.edu.tw/ cjlin/libsvm/.
                                                                             [7] R.-E. Fan, K.-W. Chang, C.-J. Hsieh, X.-R. Wang, and C.-J. Lin.
termine whether the page is spam [13]. They use a “bag-of-words”
                                                                                 LIBLINEAR: A Library for Large Linear Classification.
representation of URL tokens, and we use a similar set of features.              http://www.csie.ntu.edu.tw/ cjlin/liblinear/.
                                                                             [8] I. Fette, N. Sadeh, and A. Tomasic. Learning to Detect Phishing
6.3 Non-Machine Learning Approaches                                              Emails. In Proceedings of the International World Wide Web
   Highly Predictive Blacklists use a Page Rank-style algorithm to               Conference (WWW), Banff, Alberta, Canada, May 2007.
estimate the likelihood that an IP address will conduct a network            [9] S. Garera, N. Provos, M. Chew, and A. D. Rubin. A Framework for
intrusion against particular networks [27].                                      Detection and Measurement of Phishing Attacks. In Proceedings of
   Moshchuk et al. use VMs to analyze downloaded trojan executa-                 the ACM Workshop on Rapid Malcode (WORM), Alexandria, VA,
                                                                                 Nov. 2007.
bles, relying on third-party anti-spyware tools to detect whether
                                                                            [10] Google. Google Toolbar. http://tools.google.com/firefox/toolbar/.
VMs were infected by executables [18]. Wang et al. detect drive-by
                                                                            [11] T. Hastie, R. Tibshirani, and J. Friedman. The Elements of Statistical
exploits by using behavioral detection (monitoring anomalous state               Learning: Data Mining, Inference, and Prediction. Springer
changes in the VM) as well as detecting exploits of known vulnera-               Publishing Company, New York, NY, 2001.
bilities [25]. Provos et al. monitor VM state changes and use multi-        [12] IronPort. IronPort Web Reputation: Protect and Defend Against
ple anti-virus engines to detect VM infection [22]. Moshchuk et al.              URL-Based Threat. IronPort White Paper, 2008.
also construct a VM-based web proxy defense that uses behavior-             [13] P. Kolari, T. Finin, and A. Joshi. SVMs for the Blogosphere: Blog
based detection and adds only a few seconds of overhead to page                  Identification and Splog Detection. In Proceedings of the AAAI
rendering for the end-client [17].                                               Spring Symposium on Computational Approaches to Analysing
                                                                                 Weblogs, Stanford, CA, Mar. 2006.
   Many commercial efforts exist to detect malicious URLs, includ-
                                                                            [14] J. Ma, L. K. Saul, S. Savage, and G. M. Voelker. Identifying
ing McAfee SiteAdvisor [15], IronPort Web Reputation [12], the                   Suspicious URLs: An Application of Large-Scale Online Learning.
WebSense ThreatSeeker Network [26], WOT Web of Trust [2], and                    In Proc. of the International Conference on Machine Learning
Google Toolbar [10]. These approaches are based on blacklist con-                (ICML), Montreal, Quebec, June 2009.
struction, user feedback, and proprietary feature analysis.                 [15] McAfee. SiteAdvisor. http://www.siteadvisor.com.
                                                                            [16] D. K. McGrath and M. Gupta. Behind Phishing: An Examination of
                                                                                 Phisher Modi Operandi. In Proc. of the USENIX Workshop on
7. CONCLUSION                                                                    Large-Scale Exploits and Emergent Threats (LEET), San Francisco,
   We have described an approach for classifying URLs automat-                   CA, Apr. 2008.
ically as either malicious or benign based on supervised learning           [17] A. Moshchuk, T. Bragin, D. Deville, S. D. Gribble, and H. M. Levy.
across both lexical and host-based features. We argue that this ap-              SpyProxy: Execution-based Detection of Malicious Web Content. In
proach is complementary to both blacklisting — which cannot pre-                 Proc. of the USENIX Security Symposium, Boston, MA, Aug. 2007.
dict the status of previously unseen URLs — and systems based               [18] A. Moshchuk, T. Bragin, S. D. Gribble, and H. M. Levy. A
                                                                                 Crawler-Based Study of Spyware on the Web. In Proceedings of the
on evaluating site content and behavior — which require visiting                 Symposium on Network and Distributed System Security (NDSS), San
potentially dangerous sites. Further, we show that with appropriate              Diego, CA, Feb. 2006.
classifiers it is feasible to automatically sift through comprehensive      [19] Netscape. DMOZ Open Directory Project. http://www.dmoz.org.
feature sets (i.e., without requiring domain expertise) and identify        [20] Y. Niu, Y.-M. Wang, H. Chen, M. Ma, and F. Hsu. A Quantitative
the most predictive features for classification.                                 Study of Forum Spamming Using Context-based Analysis. In
   An open issue is how to scale our approach to handle millions                 Proceedings of the Symposium on Network and Distributed System
of URLs whose features evolve over time. We address the issue in                 Security (NDSS), San Diego, CA, Mar. 2007.
subsequent work by using online learning algorithms [14].                   [21] OpenDNS. PhishTank. http://www.phishtank.com.
                                                                            [22] N. Provos, P. Mavrommatis, M. A. Rajab, and F. Monrose. All Your
                                                                                 iFRAMEs Point to Us. In Proc. of the USENIX Security Symposium,
Acknowledgments                                                                  San Jose, CA, July 2008.
Thanks go to Alvin AuYoung, Kirill Levchenko, Patrick Verkaik               [23] B. Schölkopf and A. J. Smola. Learning with Kernels: Support
                                                                                 Vector Machines, Regularization, Optimization, and Beyond. MIT
and Michael Vrable for insightful comments on earlier drafts of this
                                                                                 Press, Cambridge, MA, 2002.
paper. Thanks to anonymous reviewers for their valuable feedback.           [24] F. Sha, A. Park, and L. K. Saul. Multiplicative Updates for
This work was supported by National Science Foundation grants                    L1 -Regularized Linear and Logistic Regression. In Proceedings of
NSF-0238323, NSF-0433668 and NSF-0829469 and by generous                         the Symposium on Intelligent Data Analysis (IDA), Ljubljana,
research, operational and in-kind support from Cisco, Google, Mi-                Slovenia, Sept. 2007.
crosoft, Yahoo and the UCSD Center for Networked Systems.                   [25] Y.-M. Wang, D. Beck, X. Jiang, R. Roussev, C. Verbowski, S. Chen,
                                                                                 and S. King. Automated Web Patrol with Strider HoneyMonkeys:
                                                                                 Finding Web Sites That Exploit Browser Vulnerabilities. In
8.[1] S.REFERENCES
         Abu-Nimeh, D. Nappa, X. Wang, and S. Nair. A Comparison of              Proceedings of the Symposium on Network and Distributed System
     Machine Learning Techniques for Phishing Detection. In                      Security (NDSS), San Diego, CA, Feb. 2006.
     Proceedings of the Anti-Phishing Working Group eCrime                  [26] WebSense. ThreatSeeker Network.
     Researchers Summit, Pittsburgh, PA, Oct. 2007.                              http://www.websense.com/content/Threatseeker.aspx.
 [2] Against Intuition. WOT Web of Trust. http://www.mywot.com.             [27] J. Zhang, P. Porras, and J. Ullrich. Highly Predictive Blacklisting. In
 [3] D. S. Anderson, C. Fleizach, S. Savage, and G. M. Voelker.                  Proc. of the USENIX Security Symposium, San Jose, CA, July 2008.
     Spamscatter: Characterizing Internet Scam Hosting Infrastructure. In   [28] Y. Zhang, J. Hong, and L. Cranor. CANTINA: A Content-Based
     Proc. of the USENIX Security Symposium, Boston, MA, Aug. 2007.              Approach to Detecting Phishing Web Sites. In Proceedings of the
 [4] A. Bergholz, J.-H. Chang, G. Paaß, F. Reichartz, and S. Strobel.            International World Wide Web Conference (WWW), Banff, Alberta,
     Improved Phishing Detection using Model-Based Features. In                  Canada, May 2007.
     Proceedings of the Conference on Email and Anti-Spam (CEAS),
     Mountain View, CA, Aug. 2008.
