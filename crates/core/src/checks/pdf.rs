use crate::{
    model::{Report, Severity},
    runner::{self, Cancel},
    store::{digest, read_file, Store, MAX_INPUT},
};
use anyhow::{bail, Result};
use std::path::Path;
fn scan_pdf_inner(store: &Store, path: &Path, stop: &Cancel) -> Result<Report> {
    let bytes = read_file(path, 8 * MAX_INPUT)?;
    if !bytes.starts_with(b"%PDF-") {
        bail!("File does not start with a PDF header");
    }
    let mut r = Report::new("PDF triage", path.display());
    r.sha256 = Some(digest(&bytes));
    r.coverage.push("Deterministic PDF triage, not an exploit/safety verdict. Encrypted/compressed content and deceptive links can require further inspection. Parsing never executes document actions.".into());
    let mut normalized = vec![];
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&String::from_utf8_lossy(&bytes[i + 1..i + 3]), 16) {
                normalized.push(v);
                i += 3;
                continue;
            }
        }
        normalized.push(bytes[i]);
        i += 1;
    }
    let text = String::from_utf8_lossy(&normalized);
    for (tag, description) in [
        ("OpenAction", "Action on document open"),
        ("AA", "Additional actions"),
        ("JavaScript", "JavaScript content"),
        ("JS", "JavaScript entry"),
        ("Launch", "Launch action"),
        ("EmbeddedFile", "Embedded file"),
        ("URI", "External link"),
        ("Encrypt", "Encrypted content"),
    ] {
        let pattern = regex::Regex::new(&format!(r"/{}(?:[\s/<>()\[\]{{}}%]|$)", tag))?;
        let count = pattern.find_iter(&text).count();
        if count > 0 {
            r.add("pdf.structure",if ["Launch","JavaScript","JS"].contains(&tag){Severity::Review}else{Severity::Info},description,format!("/{tag}: {count} raw/escaped-name occurrences"),"Review parser output and extracted destinations; this tag alone is not proof of malware.");
        }
    }
    let tmp = tempfile::NamedTempFile::new_in(&store.root)?;
    std::fs::write(tmp.path(), &bytes)?;
    for (tool, args) in [
        ("pdfid.py", vec![]),
        ("pdf-parser.py", vec!["--stats"]),
        ("pdfinfo", vec!["-url"]),
    ] {
        if !runner::available(tool) {
            r.errors.push(format!("{tool} unavailable"));
            continue;
        }
        let mut cmd = match runner::parser_command(tool, &args, tmp.path()) {
            Ok(c) => c,
            Err(e) => {
                r.errors.push(e.to_string());
                continue;
            }
        };
        let o = runner::run(&mut cmd, 30, stop)?;
        if !o.ok() {
            r.errors.push(format!(
                "{tool} failed, timed out or exceeded output limit: {}",
                o.stderr
            ));
        }
        r.details
            .push_str(&format!("\n{tool}\n{}\n{}", o.stdout, o.stderr));
    }
    r.coverage.push("External parsers run in an offline Bubblewrap filesystem exposing only system tools and the PDF snapshot. Missing sandbox support disables external parsing.".into());
    store.save(&r)?;
    Ok(r)
}

pub fn scan_pdf(store: &Store, path: &Path, stop: &Cancel) -> Result<Report> {
    match scan_pdf_inner(store, path, stop) {
        Ok(r) => Ok(r),
        Err(e) => {
            let mut r = Report::new("PDF triage", path.display());
            r.errors.push(e.to_string());
            store.save(&r)?;
            Ok(r)
        }
    }
}
