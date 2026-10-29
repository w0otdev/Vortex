use base64::Engine;
use vortex_core::model::{ExtractedLink, Item, SourceKind};

pub fn parse_raw_email(
    account: &str,
    external_id: &str,
    raw: &str,
    gmail_labels: Vec<String>,
    received_at_fallback: i64,
) -> Item {
    let normalized = raw.replace("\r\n", "\n");
    let (header_block, body_block) = normalized.split_once("\n\n").unwrap_or((&normalized, ""));

    let headers = unfold_and_parse_headers(header_block);
    let sender = find_header(&headers, "from")
        .map(decode_rfc2047)
        .unwrap_or_default();
    let recipient = find_header(&headers, "to")
        .map(decode_rfc2047)
        .unwrap_or_default();
    let received_at = find_header(&headers, "date")
        .and_then(|date| {
            time::OffsetDateTime::parse(date.trim(), &time::format_description::well_known::Rfc2822)
                .ok()
        })
        .map(|date| date.unix_timestamp())
        .filter(|&date| date > 0 && date <= received_at_fallback)
        .unwrap_or(received_at_fallback);
    let subject = find_header(&headers, "subject")
        .map(decode_rfc2047)
        .unwrap_or_default();
    let message_id = if !external_id.trim().is_empty() {
        external_id.trim().to_string()
    } else {
        find_header(&headers, "message-id")
            .map(|s| {
                s.trim_matches(|c| c == '<' || c == '>' || c == ' ')
                    .chars()
                    .filter(|c| !c.is_control())
                    .collect::<String>()
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_default()
    };
    let list_unsub = find_header(&headers, "list-unsubscribe").map(str::to_string);
    let precedence = find_header(&headers, "precedence").map(str::to_string);
    let content_type = find_header(&headers, "content-type").unwrap_or("text/plain");
    let transfer_encoding = find_header(&headers, "content-transfer-encoding").unwrap_or("7bit");

    let (plain_body, html_body) = extract_mime_bodies(body_block, content_type, transfer_encoding);
    let sanitized_html = if html_body.is_empty() {
        String::new()
    } else {
        sanitize_html_safe(&html_body)
    };
    let html_text = if sanitized_html.is_empty() {
        String::new()
    } else {
        html_to_plain_text(&sanitized_html)
    };
    let plain_clean = strip_invisible_unicode(&plain_body);
    let effective_text = if plain_clean.trim().is_empty()
        || (!html_text.is_empty() && html_text.len() > plain_clean.trim().len() + 32)
    {
        if html_text.is_empty() {
            plain_clean
        } else {
            html_text
        }
    } else {
        plain_clean
    };

    let mut links = extract_links_from_text(&effective_text);
    for link in extract_links_from_text(&sanitized_html) {
        if !links.iter().any(|l| l.url == link.url) {
            links.push(link);
        }
    }

    let mut unsub_url = None;
    if let Some(ref unsub_hdr) = list_unsub {
        for link in extract_links_from_text(unsub_hdr) {
            unsub_url = Some(link.url.clone());
            if !links.iter().any(|l| l.url == link.url) {
                links.push(ExtractedLink {
                    url: link.url,
                    domain: link.domain,
                    is_verification: false,
                    is_unsubscribe: true,
                });
            }
        }
    }
    if unsub_url.is_none() {
        unsub_url = links
            .iter()
            .find(|l| l.is_unsubscribe)
            .map(|l| l.url.clone());
    }

    let mut item = Item::new(
        SourceKind::Email,
        account,
        message_id,
        sender,
        subject,
        effective_text,
        received_at,
    );
    item.recipient = recipient;
    item.body_html = sanitized_html;
    item.extracted_links = links;
    item.unsubscribe_link = unsub_url;
    item.gmail_labels = gmail_labels;
    item.list_unsubscribe_header = list_unsub;
    item.precedence_header = precedence;
    item
}

fn unfold_and_parse_headers(header_block: &str) -> Vec<(String, String)> {
    let mut unfolded: Vec<String> = Vec::new();
    for line in header_block.lines() {
        if (line.starts_with(' ') || line.starts_with('\t')) && !unfolded.is_empty() {
            if let Some(last) = unfolded.last_mut() {
                last.push(' ');
                last.push_str(line.trim());
            }
        } else {
            unfolded.push(line.to_string());
        }
    }

    let mut out = Vec::new();
    for line in unfolded {
        if let Some((k, v)) = line.split_once(':') {
            out.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    out
}

fn find_header<'a>(headers: &'a [(String, String)], key: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
}

fn extract_mime_bodies(
    body: &str,
    content_type: &str,
    transfer_encoding: &str,
) -> (String, String) {
    let ct_lower = content_type.to_ascii_lowercase();
    if ct_lower.contains("multipart/")
        && let Some(boundary) = extract_boundary(content_type)
    {
        let mut plain = String::new();
        let mut html = String::new();
        let delim = format!("--{boundary}");
        for part in body.split(&delim).skip(1) {
            let trimmed = part.trim_start_matches('\n');
            if trimmed.starts_with("--") {
                break;
            }
            let (part_hdr, part_body) = trimmed.split_once("\n\n").unwrap_or(("", trimmed));
            let hdrs = unfold_and_parse_headers(part_hdr);
            let p_ct = find_header(&hdrs, "content-type").unwrap_or("text/plain");
            let p_enc = find_header(&hdrs, "content-transfer-encoding").unwrap_or("7bit");
            let (sub_plain, sub_html) = extract_mime_bodies(part_body.trim_end(), p_ct, p_enc);
            if sub_plain.trim().len() > plain.trim().len() {
                plain = sub_plain;
            }
            if sub_html.trim().len() > html.trim().len() {
                html = sub_html;
            }
        }
        return (plain, html);
    }

    let decoded = decode_transfer_encoding(body, transfer_encoding);
    let trimmed_dec = decoded.trim_start();
    if ct_lower.contains("text/html")
        || trimmed_dec.starts_with("<!DOCTYPE")
        || trimmed_dec.starts_with("<!doctype")
        || trimmed_dec.starts_with("<html")
    {
        (String::new(), decoded)
    } else {
        (decoded, String::new())
    }
}

fn extract_boundary(content_type: &str) -> Option<String> {
    for seg in content_type.split(';') {
        let trimmed = seg.trim();
        if trimmed.len() >= 9 && trimmed[..9].eq_ignore_ascii_case("boundary=") {
            let val = trimmed[9..].trim_matches('"').trim_matches('\'').trim();
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

pub fn decode_transfer_encoding(body: &str, encoding: &str) -> String {
    let enc = encoding.trim().to_ascii_lowercase();
    if enc == "base64" {
        let cleaned: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&cleaned) {
            return String::from_utf8_lossy(&bytes).into_owned();
        }
    } else if enc == "quoted-printable" {
        return decode_quoted_printable(body);
    }
    body.to_string()
}

pub fn decode_quoted_printable(input: &str) -> String {
    let mut bytes = Vec::with_capacity(input.len());
    let raw = input.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'=' {
            if i + 1 < raw.len() && raw[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            if i + 2 < raw.len() {
                let hi = (raw[i + 1] as char).to_digit(16);
                let lo = (raw[i + 2] as char).to_digit(16);
                if let (Some(h), Some(l)) = (hi, lo) {
                    bytes.push(((h << 4) | l) as u8);
                    i += 3;
                    continue;
                }
            }
        }
        bytes.push(raw[i]);
        i += 1;
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn decode_rfc2047(input: &str) -> String {
    if !input.contains("=?") {
        return input.trim().to_string();
    }
    let mut out = String::new();
    let mut rest = input;
    while let Some(start) = rest.find("=?") {
        out.push_str(&rest[..start]);
        let after_start = &rest[start + 2..];
        let parts: Vec<&str> = after_start.splitn(4, '?').collect();
        if parts.len() == 4 {
            let encoding = parts[1].to_ascii_uppercase();
            let payload = parts[2];
            if let Some(end_rest) = parts[3].strip_prefix('=') {
                if encoding == "B" {
                    if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(payload) {
                        out.push_str(&String::from_utf8_lossy(&decoded));
                    }
                } else if encoding == "Q" {
                    let qp = payload.replace('_', " ");
                    out.push_str(&decode_quoted_printable(&qp));
                }
                rest = end_rest;
                continue;
            }
        }
        out.push_str("=?");
        rest = after_start;
    }
    out.push_str(rest);
    out.trim().to_string()
}

/// Sanitizes HTML by removing scripts, styles, hidden prompt-injection elements,
/// event attributes, and remote tracking images.
pub fn sanitize_html_safe(html: &str) -> String {
    let mut cleaned = strip_html_comments(html);
    for blocked_tag in [
        "script", "style", "iframe", "object", "embed", "svg", "math", "form", "base", "meta",
        "link", "template",
    ] {
        cleaned = strip_tag_with_contents(&cleaned, blocked_tag);
    }
    cleaned = strip_hidden_elements(&cleaned);

    let mut out = String::with_capacity(cleaned.len());
    let mut rest = cleaned.as_str();
    while let Some(lt) = rest.find('<') {
        out.push_str(&rest[..lt]);
        let after_lt = &rest[lt + 1..];
        let Some(gt) = after_lt.find('>') else {
            break;
        };
        let tag_content = &after_lt[..gt];
        let tag_decoded = decode_html_entities(tag_content).to_ascii_lowercase();
        let tag_trimmed = tag_decoded.trim_start();

        let is_img_tag = tag_trimmed.starts_with("img")
            && tag_trimmed
                .as_bytes()
                .get(3)
                .is_none_or(|b| b.is_ascii_whitespace() || *b == b'/');
        if is_img_tag
            && (tag_trimmed.contains("http://")
                || tag_trimmed.contains("https://")
                || tag_trimmed.contains("//"))
        {
            out.push_str("<span class=\"blocked-image\">[remote image blocked]</span>");
        } else {
            let safe_tag = sanitize_tag_attributes(tag_content);
            if !safe_tag.is_empty() {
                out.push('<');
                out.push_str(&safe_tag);
                out.push('>');
            }
        }
        rest = &after_lt[gt + 1..];
    }
    out.push_str(rest);
    strip_invisible_unicode(&out)
}

fn strip_html_comments(html: &str) -> String {
    let mut current = html.to_string();
    while let Some(start) = current.find("<!--") {
        if let Some(rel_end) = current[start + 4..].find("-->") {
            current.replace_range(start..start + 4 + rel_end + 3, "");
        } else {
            current.replace_range(start.., "");
            break;
        }
    }
    current
}

fn strip_tag_with_contents(html: &str, tag: &str) -> String {
    let open_pat = format!("<{tag}");
    let close_pat = format!("</{tag}");
    let mut current = html.to_string();
    let mut search_from = 0usize;
    loop {
        if search_from >= current.len() {
            break;
        }
        let lower = current.to_ascii_lowercase();
        let Some(rel_start) = lower[search_from..].find(&open_pat) else {
            break;
        };
        let start = search_from + rel_start;
        let after_open = lower.as_bytes().get(start + open_pat.len()).copied();
        if !matches!(
            after_open,
            None | Some(b' ' | b'>' | b'/' | b'\t' | b'\n' | b'\r')
        ) {
            search_from = start + open_pat.len();
            continue;
        }
        if let Some(rel_gt) = lower[start..].find('>') {
            let tag_slice = &lower[start..start + rel_gt];
            if tag_slice.trim_end().ends_with('/') || matches!(tag, "meta" | "link" | "base") {
                current.replace_range(start..start + rel_gt + 1, "");
                search_from = start;
                continue;
            }
        }
        if let Some(rel_close) = lower[start..].find(&close_pat) {
            let close_start = start + rel_close;
            if let Some(close_gt) = lower[close_start..].find('>') {
                current.replace_range(start..close_start + close_gt + 1, "");
                search_from = start;
            } else {
                current.replace_range(start.., "");
                break;
            }
        } else {
            current.replace_range(start.., "");
            break;
        }
    }
    current
}

fn decode_html_entities(input: &str) -> String {
    let s = input
        .replace("&colon;", ":")
        .replace("&tab;", "\t")
        .replace("&newline;", "\n")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">");
    if !s.contains("&#") {
        return s;
    }
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'&' && i + 2 < bytes.len() && bytes[i + 1] == b'#' {
            let mut end = i + 2;
            let is_hex = end < bytes.len() && matches!(bytes[end], b'x' | b'X');
            if is_hex {
                end += 1;
            }
            let num_start = end;
            while end < bytes.len()
                && ((is_hex && bytes[end].is_ascii_hexdigit())
                    || (!is_hex && bytes[end].is_ascii_digit()))
            {
                end += 1;
            }
            if end > num_start {
                let num_str = &s[num_start..end];
                let parsed = if is_hex {
                    u32::from_str_radix(num_str, 16).ok()
                } else {
                    num_str.parse::<u32>().ok()
                };
                if let Some(ch) = parsed.and_then(char::from_u32) {
                    out.push(ch);
                    i = if end < bytes.len() && bytes[end] == b';' {
                        end + 1
                    } else {
                        end
                    };
                    continue;
                }
            }
        }
        let ch = s[i..].chars().next().unwrap_or(' ');
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn strip_hidden_elements(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(lt) = rest.find('<') {
        out.push_str(&rest[..lt]);
        let after_lt = &rest[lt + 1..];
        let Some(gt) = after_lt.find('>') else {
            break;
        };
        let tag = &after_lt[..gt];
        let compact: String = tag
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase();

        let has_hidden_attr = tag
            .split(|c: char| c.is_whitespace() || c == '/')
            .skip(1)
            .any(|tok| {
                let l = tok.to_ascii_lowercase();
                l == "hidden" || l.starts_with("hidden=")
            });
        let is_hidden = has_hidden_attr
            || compact.contains("display:none")
            || compact.contains("visibility:hidden")
            || compact.contains("color:transparent")
            || compact.contains("max-height:0")
            || compact.contains("opacity:0;")
            || compact.contains("opacity:0\"")
            || compact.contains("opacity:0'")
            || compact.contains("font-size:0px")
            || compact.contains("font-size:0;")
            || compact.contains("font-size:0\"")
            || compact.contains("font-size:0'");

        if is_hidden && !compact.ends_with("/>") {
            let tag_name = tag
                .split(|c: char| c.is_whitespace() || c == '/' || c == '>')
                .next()
                .unwrap_or("")
                .to_ascii_lowercase();
            if !matches!(
                tag_name.as_str(),
                "" | "br" | "hr" | "img" | "meta" | "link" | "input" | "col" | "area" | "base"
            ) {
                let open_pat = format!("<{tag_name}");
                let close_pat = format!("</{tag_name}");
                let remainder = &after_lt[gt + 1..];
                let lower_rem = remainder.to_ascii_lowercase();
                let mut depth = 1usize;
                let mut cursor = 0usize;
                let mut close_end = None;
                while cursor < lower_rem.len() {
                    let next_open = lower_rem[cursor..].find(&open_pat).map(|i| cursor + i);
                    let next_close = lower_rem[cursor..].find(&close_pat).map(|i| cursor + i);
                    match (next_open, next_close) {
                        (Some(o), Some(c)) if o < c => {
                            let after_char = lower_rem.as_bytes().get(o + open_pat.len()).copied();
                            if matches!(
                                after_char,
                                Some(b' ' | b'>' | b'/' | b'\t' | b'\n' | b'\r')
                            ) {
                                depth += 1;
                            }
                            cursor = o + open_pat.len();
                        }
                        (_, Some(c)) => {
                            let after_char = lower_rem.as_bytes().get(c + close_pat.len()).copied();
                            if matches!(after_char, Some(b' ' | b'>' | b'\t' | b'\n' | b'\r')) {
                                depth -= 1;
                                if depth == 0 {
                                    if let Some(end_gt) = lower_rem[c..].find('>') {
                                        close_end = Some(c + end_gt + 1);
                                    }
                                    break;
                                }
                            }
                            cursor = c + close_pat.len();
                        }
                        _ => break,
                    }
                }
                if let Some(skip_to) = close_end {
                    rest = &remainder[skip_to..];
                    continue;
                }
            }
        }
        out.push('<');
        out.push_str(tag);
        out.push('>');
        rest = &after_lt[gt + 1..];
    }
    out.push_str(rest);
    out
}

fn sanitize_tag_attributes(tag: &str) -> String {
    let trimmed = tag.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let is_closing = trimmed.starts_with('/');
    let after_slash = if is_closing {
        trimmed[1..].trim_start()
    } else {
        trimmed
    };
    let name_len = after_slash
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '!')
        .unwrap_or(after_slash.len());
    if name_len == 0 {
        return String::new();
    }
    let name = &after_slash[..name_len];
    if is_closing {
        return format!("/{name}");
    }

    let mut rest = after_slash[name_len..].trim();
    let self_closing = rest.ends_with('/');
    if self_closing {
        rest = rest[..rest.len() - 1].trim();
    }

    let mut kept = vec![name.to_string()];
    let bytes = rest.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b'/') {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let attr_start = i;
        while i < bytes.len()
            && !bytes[i].is_ascii_whitespace()
            && !matches!(bytes[i], b'=' | b'/' | b'"' | b'\'')
        {
            i += 1;
        }
        let attr_name = rest[attr_start..i].to_ascii_lowercase();
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < bytes.len() && bytes[i] == b'=' {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let quote = bytes[i];
                i += 1;
                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
            } else {
                while i < bytes.len()
                    && !bytes[i].is_ascii_whitespace()
                    && !matches!(bytes[i], b'/' | b'"' | b'\'')
                {
                    i += 1;
                }
            }
        } else if attr_name.is_empty() {
            i += 1;
            continue;
        }

        let raw_attr = rest[attr_start..i].trim();
        if raw_attr.is_empty() || attr_name.is_empty() {
            continue;
        }
        let decoded_compact: String = decode_html_entities(raw_attr)
            .chars()
            .filter(|c| !c.is_whitespace() && !c.is_control())
            .collect::<String>()
            .to_ascii_lowercase();

        if attr_name.starts_with("on")
            || matches!(attr_name.as_str(), "srcdoc" | "formaction")
            || decoded_compact.contains("javascript:")
            || decoded_compact.contains("vbscript:")
            || decoded_compact.contains("data:text/html")
            || decoded_compact.contains("data:image/svg")
            || (attr_name == "style"
                && (decoded_compact.contains("url(") || decoded_compact.contains("expression(")))
        {
            continue;
        }
        kept.push(raw_attr.to_string());
    }
    if self_closing {
        kept.push("/".to_string());
    }
    kept.join(" ")
}

pub fn html_to_plain_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    let replaced = out
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"");
    replaced.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn plain_text_to_html(text: &str) -> String {
    let escaped = text
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");
    format!(
        "<div style=\"white-space:pre-wrap;line-height:1.65;\">{}</div>",
        escaped
    )
}

pub fn strip_invisible_unicode(input: &str) -> String {
    input
        .chars()
        .filter(|&c| {
            !matches!(
                c,
                '\u{200B}'
                    | '\u{200C}'
                    | '\u{200D}'
                    | '\u{2060}'
                    | '\u{FEFF}'
                    | '\u{00AD}'
                    | '\u{202A}'..='\u{202E}'
            )
        })
        .collect()
}

pub fn extract_links_from_text(text: &str) -> Vec<ExtractedLink> {
    let mut out = Vec::new();
    for token in text.split(|c: char| {
        c.is_whitespace() || matches!(c, '<' | '>' | '"' | '\'' | '(' | ')' | '[' | ']')
    }) {
        let trimmed = token.trim_end_matches(['.', ',', ';', ':']);
        if !(trimmed.starts_with("https://") || trimmed.starts_with("http://")) {
            continue;
        }
        let domain = extract_url_domain(trimmed);
        if domain.is_empty() {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        let is_unsubscribe = lower.contains("unsubscribe")
            || lower.contains("opt-out")
            || lower.contains("optout")
            || lower.contains("preferences");
        let is_verification = !is_unsubscribe
            && (lower.contains("verify")
                || lower.contains("confirm")
                || lower.contains("login")
                || lower.contains("auth")
                || lower.contains("magic")
                || lower.contains("token=")
                || lower.contains("code="));
        if !out.iter().any(|e: &ExtractedLink| e.url == trimmed) {
            out.push(ExtractedLink {
                url: trimmed.to_string(),
                domain,
                is_verification,
                is_unsubscribe,
            });
        }
    }
    out
}

pub fn extract_url_domain(url: &str) -> String {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let authority = without_scheme.split(['/', '?', '#']).next().unwrap_or("");
    let host_port = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let host = host_port.split(':').next().unwrap_or("");
    host.trim_start_matches("www.").to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_message_date_and_falls_back_when_invalid() {
        let message = "From: sender@example.com\r\nTo: receiver@example.com\r\nDate: Wed, 15 Nov 2023 05:13:20 +0700\r\nSubject: Verification code 123456\r\n\r\nCode: 123456";
        let item = parse_raw_email("account", "1", message, vec![], 1_700_001_000);
        assert_eq!(item.received_at, 1_700_000_000);
        assert_eq!(item.recipient, "receiver@example.com");
        let invalid = message.replace("Wed, 15 Nov 2023 05:13:20 +0700", "invalid");
        assert_eq!(
            parse_raw_email("account", "1", &invalid, vec![], 1_700_001_000).received_at,
            1_700_001_000
        );
    }

    #[test]
    fn preserves_full_html_with_nested_hidden_preheader_and_white_buttons() {
        let html = r#"<div style="display:none"><span>Preheader snippet</span></div><div style="background:#ffffff"><p>Mới đăng nhập trên Windows noname.dt500@gmail.com Chúng tôi phát hiện thấy có một yêu cầu đăng nhập mới vào Tài khoản Google của bạn trên một thiết bị Windows. Nếu đây là yêu cầu của bạn, thì bạn không cần làm gì thêm. Nếu không phải, hãy kiểm tra hoạt động ngay.</p><a href="https://myaccount.google.com/notifications" style="color:#ffffff;background:#1a73e8">Kiểm tra hoạt động</a></div>"#;
        let clean = sanitize_html_safe(html);
        assert!(!clean.contains("Preheader snippet"));
        assert!(clean.contains("thì bạn không cần làm gì thêm"));
        assert!(clean.contains("Kiểm tra hoạt động"));
    }
}
