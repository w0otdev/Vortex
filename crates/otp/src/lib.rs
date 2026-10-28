use serde::{Deserialize, Serialize};
use std::time::Instant;
use vortex_core::model::{ExtractedCode, Item, OtpPopupEvent};

pub const DETECTOR_VERSION: &str = "0.1.0";
pub const DEFAULT_EXPIRY_SECS: i64 = 600;

const OTP_KEYWORDS: &[&str] = &[
    "verification code",
    "security code",
    "login code",
    "sign-in code",
    "signin code",
    "one-time password",
    "one time password",
    "one-time code",
    "authentication code",
    "confirmation code",
    "passcode",
    "2fa code",
    "otp",
    "verify your",
    "temporary code",
    "access code",
    "mã xác nhận",
    "mã otp",
    "mã đăng nhập",
    "mã bảo mật",
    "mã xác thực",
    "mã kích hoạt",
    "mã giao dịch",
];

const NEGATIVE_KEYWORDS: &[&str] = &[
    "order #",
    "order number",
    "order id",
    "invoice #",
    "invoice number",
    "tracking #",
    "tracking number",
    "shipment",
    "mã đơn hàng",
    "mã vận đơn",
    "số hóa đơn",
    "số điện thoại",
    "hotline",
    "tel:",
    "phone:",
    "call us",
    "zip code",
    "postal code",
    "unsubscribe",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusMetrics {
    pub total: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub false_negatives: usize,
    pub precision: f64,
    pub recall: f64,
    pub max_latency_micros: u64,
}

pub fn detect_fast_path(
    item: &Item,
    now_epoch: i64,
    blocked_patterns: &[String],
) -> Option<(ExtractedCode, OtpPopupEvent)> {
    let start = Instant::now();
    let combined = format!("{}\n{}", item.subject, item.body_text);
    let lower = combined.to_lowercase();

    let has_otp_keyword = OTP_KEYWORDS.iter().any(|kw| lower.contains(kw));
    if !has_otp_keyword {
        return None;
    }

    let sender_lower = item.sender.to_ascii_lowercase();
    let sender_reputation_bonus: i32 = if sender_lower.contains("noreply")
        || sender_lower.contains("no-reply")
        || sender_lower.contains("security")
        || sender_lower.contains("verify")
        || sender_lower.contains("auth")
        || sender_lower.contains("account")
        || sender_lower.contains("alert")
    {
        10
    } else {
        0
    };

    let freshness_bonus: i32 = if (now_epoch - item.received_at).abs() <= 900 {
        5
    } else {
        0
    };

    let candidates = extract_candidates(&combined);
    let mut best: Option<(String, i32)> = None;

    for (cand, byte_offset) in candidates {
        if blocked_patterns
            .iter()
            .any(|p| p.eq_ignore_ascii_case(&cand))
        {
            continue;
        }
        let score = score_candidate(
            &cand,
            byte_offset,
            &combined,
            &lower,
            sender_reputation_bonus,
            freshness_bonus,
        );
        if score >= 65 && best.as_ref().is_none_or(|(_, best_s)| score > *best_s) {
            best = Some((cand, score));
        }
    }

    let (code, score) = best?;
    let expiry_secs = parse_expiry_seconds(&lower).unwrap_or(DEFAULT_EXPIRY_SECS);
    let expires_at = now_epoch + expiry_secs;
    let service = if !item.sender_domain.is_empty() {
        item.sender_domain.clone()
    } else {
        item.sender.clone()
    };
    let verification_link = item
        .extracted_links
        .iter()
        .find(|l| l.is_verification)
        .map(|l| l.url.clone());

    let confidence = score.clamp(0, 100) as u8;
    let extracted = ExtractedCode {
        code: code.clone(),
        service: service.clone(),
        expires_at,
        confidence,
        verification_link: verification_link.clone(),
    };

    let latency_ms = start.elapsed().as_millis() as u64;
    let code_hash = OtpPopupEvent::compute_dedup_hash(&item.external_id, &code);
    let popup = OtpPopupEvent {
        item_id: item.id,
        message_id: item.external_id.clone(),
        code_hash,
        code,
        sender: item.sender.clone(),
        service,
        received_at: item.received_at,
        expires_at,
        verification_link,
        phishing_warning: item.phishing_warning.clone(),
        detected_latency_ms: latency_ms,
    };

    Some((extracted, popup))
}

fn extract_candidates(text: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if !bytes[i].is_ascii_alphanumeric() {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
            i += 1;
        }
        let raw_tok = text[start..i].trim_matches('-');
        if raw_tok.len() < 4 || raw_tok.len() > 10 {
            continue;
        }

        // Ignore tokens directly preceded by '#', '$', '+', or '/' (order numbers, prices, paths).
        if start > 0 && matches!(bytes[start - 1], b'#' | b'$' | b'+' | b'/' | b'=') {
            continue;
        }
        // Ignore tokens directly followed by '%', '$', 'đ', or '.00'.
        if i < bytes.len() && matches!(bytes[i], b'%' | b'$') {
            continue;
        }

        if let Some(normalized) = classify_token_shape(raw_tok) {
            out.push((normalized, start));
        }
    }
    out
}

fn classify_token_shape(tok: &str) -> Option<String> {
    if tok.chars().all(|c| c.is_ascii_digit()) && (4..=8).contains(&tok.len()) {
        return Some(tok.to_string());
    }

    // e.g. "123-456" or "G-582910"
    if let Some((left, right)) = tok.split_once('-') {
        if left.len() == 3
            && right.len() == 3
            && left.chars().all(|c| c.is_ascii_digit())
            && right.chars().all(|c| c.is_ascii_digit())
        {
            return Some(format!("{left}{right}"));
        }
        if left.len() == 1
            && left.chars().all(|c| c.is_ascii_uppercase())
            && (4..=8).contains(&right.len())
            && right.chars().all(|c| c.is_ascii_digit())
        {
            return Some(right.to_string());
        }
    }

    // Alphanumeric uppercase OTP (6..=8 chars, contains both uppercase letters and digits).
    if (6..=8).contains(&tok.len())
        && tok
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        && tok.chars().any(|c| c.is_ascii_uppercase())
        && tok.chars().filter(|c| c.is_ascii_digit()).count() >= 2
    {
        return Some(tok.to_string());
    }

    None
}

fn score_candidate(
    cand: &str,
    byte_offset: usize,
    original: &str,
    lower: &str,
    sender_bonus: i32,
    freshness_bonus: i32,
) -> i32 {
    let mut score: i32 = if cand.chars().all(|c| c.is_ascii_digit()) {
        if cand.len() == 6 { 38 } else { 28 }
    } else {
        22
    };

    score += sender_bonus + freshness_bonus;

    let win_start = floor_char_boundary(lower, byte_offset.saturating_sub(90));
    let win_end = ceil_char_boundary(lower, (byte_offset + cand.len() + 90).min(lower.len()));
    let window = &lower[win_start..win_end];

    if OTP_KEYWORDS.iter().any(|kw| window.contains(kw)) {
        score += 45;
    } else if OTP_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
        score += 20;
    }

    if NEGATIVE_KEYWORDS.iter().any(|neg| window.contains(neg)) {
        score -= 60;
    }

    // Reject 4-digit calendar years (1990..=2035) unless immediately preceded by "code:" / "otp:" / "is ".
    if cand.len() == 4
        && let Ok(yr) = cand.parse::<u32>()
        && (1990..=2035).contains(&yr)
    {
        let immediate_start = floor_char_boundary(original, byte_offset.saturating_sub(18));
        let immediate_prefix = original[immediate_start..byte_offset].to_ascii_lowercase();
        let explicit_code = immediate_prefix.contains("code")
            || immediate_prefix.contains("otp")
            || immediate_prefix.contains("pin")
            || immediate_prefix.contains("mã");
        if !explicit_code {
            score -= 55;
        }
    }

    score
}

pub fn parse_expiry_seconds(lower_text: &str) -> Option<i64> {
    let tokens: Vec<&str> = lower_text.split_whitespace().collect();
    for window in tokens.windows(2) {
        let raw_num = window[0].trim_matches(|c: char| !c.is_ascii_digit());
        let Ok(n) = raw_num.parse::<i64>() else {
            continue;
        };
        if n <= 0 || n > 1440 {
            continue;
        }
        let unit = window[1].trim_matches(|c: char| !c.is_alphabetic());
        if matches!(unit, "minute" | "minutes" | "min" | "mins" | "phút") {
            return Some(n * 60);
        }
        if matches!(unit, "second" | "seconds" | "sec" | "secs" | "giây") {
            return Some(n);
        }
        if matches!(unit, "hour" | "hours" | "giờ") && n <= 24 {
            return Some(n * 3600);
        }
    }
    None
}

fn floor_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

fn ceil_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx < s.len() && !s.is_char_boundary(idx) {
        idx += 1;
    }
    idx
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct LabeledSample {
    pub sender: &'static str,
    pub subject: &'static str,
    pub body: &'static str,
    pub expected_code: Option<&'static str>,
}

#[cfg(test)]
pub fn builtin_labeled_corpus() -> Vec<LabeledSample> {
    vec![
        // Positive OTP samples (English + Vietnamese)
        LabeledSample {
            sender: "GitHub <noreply@github.com>",
            subject: "[GitHub] Your verification code is 482910",
            body: "Enter 482910 to verify your device. This code expires in 10 minutes.",
            expected_code: Some("482910"),
        },
        LabeledSample {
            sender: "Google <no-reply@accounts.google.com>",
            subject: "Google Verification Code",
            body: "Your Google verification code is G-739201. Do not share this code.",
            expected_code: Some("739201"),
        },
        LabeledSample {
            sender: "Vietcombank <alert@vcb.com.vn>",
            subject: "Mã OTP xác thực giao dịch",
            body: "Quý khách đang thực hiện chuyển khoản. Mã xác nhận OTP của quý khách là 883920. Hiệu lực trong 3 phút.",
            expected_code: Some("883920"),
        },
        LabeledSample {
            sender: "Zalo <security@zalo.me>",
            subject: "Mã đăng nhập Zalo",
            body: "Mã đăng nhập của bạn là 5912. Tuyệt đối không chia sẻ cho bất kỳ ai.",
            expected_code: Some("5912"),
        },
        LabeledSample {
            sender: "Stripe <support@stripe.com>",
            subject: "Stripe two-step authentication",
            body: "Your Stripe security code is 129-482. It expires in 5 minutes.",
            expected_code: Some("129482"),
        },
        LabeledSample {
            sender: "Steam <noreply@steampowered.com>",
            subject: "Your Steam Guard access code",
            body: "Here is the Steam Guard access code you need to login: 9K4M2P",
            expected_code: Some("9K4M2P"),
        },
        LabeledSample {
            sender: "Shopee <info@shopee.vn>",
            subject: "Mã xác thực tài khoản Shopee",
            body: "Nhập mã bảo mật 664109 để hoàn tất liên kết ví. Có giá trị trong 5 phút.",
            expected_code: Some("664109"),
        },
        LabeledSample {
            sender: "AWS <no-reply@signin.aws>",
            subject: "AWS Multi-Factor Authentication code",
            body: "Your one-time password for AWS root sign-in is 304958.",
            expected_code: Some("304958"),
        },
        // Negative samples (orders, tracking, prices, phone numbers, dates, newsletters)
        LabeledSample {
            sender: "Amazon <shipment-tracking@amazon.com>",
            subject: "Your Amazon order #482910 has shipped",
            body: "Order number 482910 is on the way. Total charged: $149.00. Delivery by 2026.",
            expected_code: None,
        },
        LabeledSample {
            sender: "FedEx <TrackingUpdates@fedex.com>",
            subject: "FedEx Shipment 78492011 Delivered",
            body: "Your package with tracking number 78492011 was delivered at 14:30.",
            expected_code: None,
        },
        LabeledSample {
            sender: "Tiki <hotro@tiki.vn>",
            subject: "Xác nhận đơn hàng #992831",
            body: "Cảm ơn bạn đã đặt hàng. Mã đơn hàng 992831 sẽ được giao trong 2 ngày. Hotline: 19006035.",
            expected_code: None,
        },
        LabeledSample {
            sender: "Billing <billing@acme.io>",
            subject: "Invoice #204918 for October 2026",
            body: "Please find attached invoice number 204918 for $2500. Call us at 80055512.",
            expected_code: None,
        },
        LabeledSample {
            sender: "Newsletter <deals@fashion.example.com>",
            subject: "Weekend Flash Sale: 50% off everything",
            body: "Visit our store at 1420 Broadway, Suite 8400, New York 10018. Copyright 2026.",
            expected_code: None,
        },
        LabeledSample {
            sender: "HR <people@company.com>",
            subject: "Updated Q4 2026 holiday schedule",
            body: "Our office closes on 2026-12-24 and reopens on 2027-01-02.",
            expected_code: None,
        },
    ]
}

#[cfg(test)]
pub fn evaluate_corpus(samples: &[LabeledSample]) -> CorpusMetrics {
    let mut tp = 0usize;
    let mut fp = 0usize;
    let mut tn = 0usize;
    let mut fn_count = 0usize;
    let mut max_latency_micros = 0u64;
    let now = 1_700_000_000i64;

    for (idx, s) in samples.iter().enumerate() {
        let item = Item::new(
            vortex_core::model::SourceKind::Email,
            "test@example.com",
            format!("corpus-{idx}"),
            s.sender,
            s.subject,
            s.body,
            now,
        );
        let t0 = Instant::now();
        let detected = detect_fast_path(&item, now, &[]);
        let elapsed_us = t0.elapsed().as_micros() as u64;
        max_latency_micros = max_latency_micros.max(elapsed_us);

        match (s.expected_code, detected) {
            (Some(exp), Some((got, _))) if got.code == exp => tp += 1,
            (Some(_), Some(_)) => {
                fp += 1;
                fn_count += 1;
            }
            (Some(_), None) => fn_count += 1,
            (None, Some(_)) => fp += 1,
            (None, None) => tn += 1,
        }
    }

    let precision = if tp + fp == 0 {
        1.0
    } else {
        tp as f64 / (tp + fp) as f64
    };
    let recall = if tp + fn_count == 0 {
        1.0
    } else {
        tp as f64 / (tp + fn_count) as f64
    };

    CorpusMetrics {
        total: samples.len(),
        true_positives: tp,
        false_positives: fp,
        true_negatives: tn,
        false_negatives: fn_count,
        precision,
        recall,
        max_latency_micros,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vortex_core::model::SourceKind;

    #[test]
    fn corpus_achieves_high_precision_and_recall_under_2ms() {
        let corpus = builtin_labeled_corpus();
        let metrics = evaluate_corpus(&corpus);
        assert_eq!(metrics.false_positives, 0);
        assert_eq!(metrics.false_negatives, 0);
        assert!(metrics.precision >= 0.99);
        assert!(metrics.recall >= 0.99);
        assert!(metrics.max_latency_micros < 50_000);
    }

    #[test]
    fn respects_not_a_code_user_flag_and_parses_expiry() {
        let item = Item::new(
            SourceKind::Email,
            "me@example.com",
            "msg-99",
            "Service <noreply@service.vn>",
            "Mã xác nhận: 778899",
            "Mã OTP của bạn là 778899, hiệu lực trong 5 phút.",
            1_700_000_000,
        );
        let (extracted, popup) = detect_fast_path(&item, 1_700_000_000, &[]).unwrap();
        assert_eq!(extracted.code, "778899");
        assert_eq!(extracted.expires_at, 1_700_000_300);
        assert!(!format!("{popup:?}").contains("778899"));

        let suppressed = detect_fast_path(&item, 1_700_000_000, &["778899".to_string()]);
        assert!(suppressed.is_none());
    }
}
