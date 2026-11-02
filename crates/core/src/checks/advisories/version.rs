//! Version ordering per packaging scheme. Never compare across schemes: an RPM
//! release string means nothing to dpkg. Each follows the reference algorithm
//! (rpmvercmp, dpkg verrevcmp, alpm's rpmvercmp variant) and is tested against
//! the orderings those projects document.
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Rpm,
    Dpkg,
    Pacman,
}

pub fn compare(scheme: Scheme, a: &str, b: &str) -> Ordering {
    match scheme {
        Scheme::Rpm => rpm_evr(a, b),
        Scheme::Dpkg => dpkg(a, b),
        Scheme::Pacman => pacman(a, b),
    }
}

/// `[epoch:]version[-release]`; a missing epoch is 0, a missing release matches any.
fn split_evr(s: &str) -> (u64, &str, Option<&str>) {
    let (epoch, rest) = match s.split_once(':') {
        Some((e, r)) if e.chars().all(|c| c.is_ascii_digit()) && !e.is_empty() => {
            (e.parse().unwrap_or(0), r)
        }
        _ => (0, s),
    };
    match rest.rsplit_once('-') {
        Some((v, r)) => (epoch, v, Some(r)),
        None => (epoch, rest, None),
    }
}

/// rpm's rpmvercmp (rpm 4.x): alphanumeric segments, numbers beat letters,
/// `~` sorts before everything, `^` after the base version.
pub fn rpmvercmp(a: &str, b: &str) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    let (mut one, mut two) = (a.as_bytes(), b.as_bytes());
    let sep = |c: u8| !c.is_ascii_alphanumeric() && c != b'~' && c != b'^';
    loop {
        while one.first().is_some_and(|&c| sep(c)) {
            one = &one[1..];
        }
        while two.first().is_some_and(|&c| sep(c)) {
            two = &two[1..];
        }
        if one.first() == Some(&b'~') || two.first() == Some(&b'~') {
            if one.first() != Some(&b'~') {
                return Ordering::Greater;
            }
            if two.first() != Some(&b'~') {
                return Ordering::Less;
            }
            one = &one[1..];
            two = &two[1..];
            continue;
        }
        if one.first() == Some(&b'^') || two.first() == Some(&b'^') {
            if one.is_empty() {
                return Ordering::Less;
            }
            if two.is_empty() {
                return Ordering::Greater;
            }
            if one.first() != Some(&b'^') {
                return Ordering::Greater;
            }
            if two.first() != Some(&b'^') {
                return Ordering::Less;
            }
            one = &one[1..];
            two = &two[1..];
            continue;
        }
        if one.is_empty() || two.is_empty() {
            break;
        }
        let numeric = one[0].is_ascii_digit();
        let take = |s: &[u8]| {
            s.iter()
                .take_while(|c| {
                    if numeric {
                        c.is_ascii_digit()
                    } else {
                        c.is_ascii_alphabetic()
                    }
                })
                .count()
        };
        let (n1, n2) = (take(one), take(two));
        let (seg1, seg2) = (&one[..n1], &two[..n2]);
        if seg2.is_empty() {
            // Segment types differ: a number beats letters.
            return if numeric {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
        let order = if numeric {
            let strip = |s: &[u8]| {
                let z = s.iter().take_while(|&&c| c == b'0').count();
                s[z..].to_vec()
            };
            let (x, y) = (strip(seg1), strip(seg2));
            x.len().cmp(&y.len()).then_with(|| x.cmp(&y))
        } else {
            seg1.cmp(seg2)
        };
        if order != Ordering::Equal {
            return order;
        }
        one = &one[n1..];
        two = &two[n2..];
    }
    match (one.is_empty(), two.is_empty()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        _ => Ordering::Greater,
    }
}

fn rpm_evr(a: &str, b: &str) -> Ordering {
    let (e1, v1, r1) = split_evr(a);
    let (e2, v2, r2) = split_evr(b);
    e1.cmp(&e2)
        .then_with(|| rpmvercmp(v1, v2))
        .then_with(|| match (r1, r2) {
            (Some(x), Some(y)) => rpmvercmp(x, y),
            _ => Ordering::Equal,
        })
}

/// dpkg's ordering of a character in the non-digit part: `~` first, then the end
/// of the string, then letters, then everything else.
fn dpkg_order(c: Option<u8>) -> i32 {
    match c {
        None => 0,
        Some(b'~') => -1,
        Some(c) if c.is_ascii_digit() => 0,
        Some(c) if c.is_ascii_alphabetic() => c as i32,
        Some(c) => c as i32 + 256,
    }
}

/// dpkg's verrevcmp over one part (upstream version or revision).
fn verrevcmp(a: &str, b: &str) -> Ordering {
    let (mut x, mut y) = (a.as_bytes(), b.as_bytes());
    while !x.is_empty() || !y.is_empty() {
        while x.first().is_some_and(|c| !c.is_ascii_digit())
            || y.first().is_some_and(|c| !c.is_ascii_digit())
        {
            let (ox, oy) = (
                dpkg_order(x.first().copied()),
                dpkg_order(y.first().copied()),
            );
            if ox != oy {
                return ox.cmp(&oy);
            }
            x = &x[1.min(x.len())..];
            y = &y[1.min(y.len())..];
        }
        while x.first() == Some(&b'0') {
            x = &x[1..];
        }
        while y.first() == Some(&b'0') {
            y = &y[1..];
        }
        let mut first_diff = Ordering::Equal;
        while x.first().is_some_and(u8::is_ascii_digit) && y.first().is_some_and(u8::is_ascii_digit)
        {
            if first_diff == Ordering::Equal {
                first_diff = x[0].cmp(&y[0]);
            }
            x = &x[1..];
            y = &y[1..];
        }
        if x.first().is_some_and(u8::is_ascii_digit) {
            return Ordering::Greater;
        }
        if y.first().is_some_and(u8::is_ascii_digit) {
            return Ordering::Less;
        }
        if first_diff != Ordering::Equal {
            return first_diff;
        }
    }
    Ordering::Equal
}

fn dpkg(a: &str, b: &str) -> Ordering {
    let parse = |s: &str| {
        let (epoch, rest) = match s.split_once(':') {
            Some((e, r)) if e.chars().all(|c| c.is_ascii_digit()) => {
                (e.parse::<u64>().unwrap_or(0), r)
            }
            _ => (0, s),
        };
        let (upstream, revision) = rest.rsplit_once('-').unwrap_or((rest, ""));
        (epoch, upstream.to_string(), revision.to_string())
    };
    let (e1, u1, r1) = parse(a);
    let (e2, u2, r2) = parse(b);
    e1.cmp(&e2)
        .then_with(|| verrevcmp(&u1, &u2))
        .then_with(|| verrevcmp(&r1, &r2))
}

/// alpm's rpmvercmp: like rpm's, but different separator runs decide, and a
/// trailing alpha segment loses to the end of the string (1.0a < 1.0).
fn alpm_rpmvercmp(a: &str, b: &str) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    let (mut one, mut two) = (a.as_bytes(), b.as_bytes());
    while !one.is_empty() && !two.is_empty() {
        let s1 = one
            .iter()
            .take_while(|c| !c.is_ascii_alphanumeric())
            .count();
        let s2 = two
            .iter()
            .take_while(|c| !c.is_ascii_alphanumeric())
            .count();
        one = &one[s1..];
        two = &two[s2..];
        if one.is_empty() || two.is_empty() {
            break;
        }
        if s1 != s2 {
            return s1.cmp(&s2);
        }
        let numeric = one[0].is_ascii_digit();
        let take = |s: &[u8]| {
            s.iter()
                .take_while(|c| {
                    if numeric {
                        c.is_ascii_digit()
                    } else {
                        c.is_ascii_alphabetic()
                    }
                })
                .count()
        };
        let (n1, n2) = (take(one), take(two));
        if n2 == 0 {
            return if numeric {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
        let (seg1, seg2) = (&one[..n1], &two[..n2]);
        let order = if numeric {
            let strip = |s: &[u8]| {
                let z = s.iter().take_while(|&&c| c == b'0').count();
                s[z..].to_vec()
            };
            let (x, y) = (strip(seg1), strip(seg2));
            x.len().cmp(&y.len()).then_with(|| x.cmp(&y))
        } else {
            seg1.cmp(seg2)
        };
        if order != Ordering::Equal {
            return order;
        }
        one = &one[n1..];
        two = &two[n2..];
    }
    if one.is_empty() && two.is_empty() {
        return Ordering::Equal;
    }
    // A remaining alpha string never beats an empty one.
    let alpha = |s: &[u8]| s.first().is_some_and(u8::is_ascii_alphabetic);
    if (one.is_empty() && !alpha(two)) || alpha(one) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn pacman(a: &str, b: &str) -> Ordering {
    let (e1, v1, r1) = split_evr(a);
    let (e2, v2, r2) = split_evr(b);
    e1.cmp(&e2)
        .then_with(|| alpm_rpmvercmp(v1, v2))
        .then_with(|| match (r1, r2) {
            (Some(x), Some(y)) => alpm_rpmvercmp(x, y),
            _ => Ordering::Equal,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ascending(scheme: Scheme, versions: &[&str]) {
        for pair in versions.windows(2) {
            assert_eq!(
                compare(scheme, pair[0], pair[1]),
                Ordering::Less,
                "{scheme:?}: {} < {}",
                pair[0],
                pair[1]
            );
            assert_eq!(
                compare(scheme, pair[1], pair[0]),
                Ordering::Greater,
                "{scheme:?}: {} > {}",
                pair[1],
                pair[0]
            );
        }
    }

    #[test]
    fn pacman_follows_the_vercmp_manual() {
        // vercmp(8): the documented orderings.
        ascending(
            Scheme::Pacman,
            &[
                "1.0a", "1.0alpha", "1.0b", "1.0beta", "1.0p", "1.0pre", "1.0rc", "1.0", "1.0.a",
                "1.0.1",
            ],
        );
        ascending(
            Scheme::Pacman,
            &["1", "1.0", "1.1", "1.1.1", "1.2", "2.0", "3.0.0"],
        );
        ascending(Scheme::Pacman, &["9.0.1224-1", "9.0.1225-1", "1:0.1-1"]);
        assert_eq!(
            compare(Scheme::Pacman, "1.0-1", "1.0"),
            Ordering::Equal,
            "a missing release matches any"
        );
    }

    #[test]
    fn rpm_follows_rpmvercmp() {
        ascending(Scheme::Rpm, &["1.0~rc1", "1.0", "1.0^git1", "1.0.1"]);
        ascending(Scheme::Rpm, &["1.0a", "1.0.1"]);
        ascending(Scheme::Rpm, &["2.9", "2.10", "10.0"]);
        ascending(
            Scheme::Rpm,
            &["158.0-2.fc44", "159.0-1.fc44", "1:1.0-1.fc44"],
        );
        ascending(
            Scheme::Rpm,
            &["3.0.7-27.el9", "3.0.7-27.el9_4", "3.0.7-28.el9"],
        );
        assert_eq!(
            compare(Scheme::Rpm, "1.0010", "1.10"),
            Ordering::Equal,
            "leading zeros do not count"
        );
        assert_eq!(
            compare(Scheme::Rpm, "1.0", "1_0"),
            Ordering::Equal,
            "separators do not count"
        );
    }

    #[test]
    fn dpkg_follows_debian_policy() {
        ascending(
            Scheme::Dpkg,
            &["1.0~rc1", "1.0", "1.0-1", "1.0+b1", "1.0.1"],
        );
        ascending(
            Scheme::Dpkg,
            &["3.0.20-1~deb12u1", "3.0.20-1~deb12u2", "3.0.20-1"],
        );
        ascending(Scheme::Dpkg, &["5.4.5-0.3", "5.6.1+really5.4.5-1", "1:0.1"]);
        assert_eq!(compare(Scheme::Dpkg, "0:1.2", "1.2"), Ordering::Equal);
        assert_eq!(
            compare(Scheme::Dpkg, "1.2-0", "1.2"),
            Ordering::Equal,
            "an empty revision compares like 0"
        );
    }
}

#[cfg(test)]
mod against_rpm {
    use super::*;

    /// 435 pairs judged by the real `rpm.vercmp` (rpm 4 on Fedora 44), so this
    /// runs anywhere. Regenerate with `rpm --eval '%{lua: print(rpm.vercmp(a, b))}'`.
    #[test]
    fn matches_the_rpm_library() {
        let pairs: Vec<(String, String, i32)> =
            serde_json::from_str(include_str!("../../../tests/data/rpmvercmp.json")).unwrap();
        let wrong: Vec<_> = pairs
            .iter()
            .filter(|(a, b, want)| rpmvercmp(a, b) as i32 != *want)
            .collect();
        assert!(
            wrong.is_empty(),
            "{} of {} differ: {wrong:?}",
            wrong.len(),
            pairs.len()
        );
    }
}
