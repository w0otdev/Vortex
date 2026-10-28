pub mod ai;

use crate::ai::one_line_summary;
use vortex_core::config::UserRuleConfig;
use vortex_core::model::{ExtractedEntity, Item, ItemCategory, ItemState};
use vortex_otp::detect_fast_path;

pub const CLASSIFIER_VERSION: &str = "0.1.0";

const SECURITY_KEYWORDS: &[&str] = &[
    "security alert",
    "new sign-in",
    "new login",
    "unusual activity",
    "password changed",
    "password reset",
    "suspicious login",
    "recovery email",
    "cảnh báo bảo mật",
    "đăng nhập mới",
    "thay đổi mật khẩu",
    "thiết bị lạ",
    "khôi phục mật khẩu",
];

const BILLING_KEYWORDS: &[&str] = &[
    "invoice",
    "receipt",
    "payment received",
    "payment due",
    "amount due",
    "billing statement",
    "subscription renewal",
    "charged",
    "biên lai",
    "hóa đơn",
    "thanh toán",
    "sao kê",
    "biến động số dư",
    "chuyển khoản thành công",
];

const SHIPPING_KEYWORDS: &[&str] = &[
    "has shipped",
    "out for delivery",
    "tracking number",
    "package delivered",
    "shipment update",
    "your order #",
    "order confirmed",
    "đang giao hàng",
    "đơn hàng",
    "mã vận đơn",
    "giao hàng thành công",
    "vận chuyển",
];

const SOCIAL_KEYWORDS: &[&str] = &[
    "friend request",
    "new follower",
    "mentioned you",
    "liked your post",
    "commented on",
    "sent you a message",
    "lời mời kết bạn",
    " đã bình luận",
    "nhắc đến bạn",
];

const WORK_KEYWORDS: &[&str] = &[
    "pull request",
    "merge request",
    "ci/cd",
    "build failed",
    "calendar invite",
    "meeting agenda",
    "sprint review",
    "issue assigned",
    "lịch họp",
    "biên bản họp",
    "báo cáo tuần",
];

const PROMO_KEYWORDS: &[&str] = &[
    "% off",
    "flash sale",
    "limited time offer",
    "special offer",
    "deal of the day",
    "unsubscribe",
    "view in browser",
    "khuyến mãi",
    "giảm giá",
    "ưu đãi",
    "hủy đăng ký",
];

const KNOWN_BRANDS: &[(&str, &[&str])] = &[
    ("paypal", &["paypal.com"]),
    (
        "google",
        &["google.com", "accounts.google.com", "youtube.com"],
    ),
    ("github", &["github.com"]),
    ("apple", &["apple.com", "icloud.com"]),
    ("microsoft", &["microsoft.com", "live.com", "office.com"]),
    ("stripe", &["stripe.com"]),
    ("vietcombank", &["vcb.com.vn", "vietcombank.com.vn"]),
    ("amazon", &["amazon.com", "amazon.co.uk", "amazon.de"]),
];

#[derive(Debug, Clone)]
pub struct ClassificationOutcome {
    pub category: ItemCategory,
    pub confidence: u8,
    pub labels: Vec<String>,
    pub uncertain: bool,
    pub phishing_score: u8,
    pub phishing_warning: Option<String>,
    pub entities: Vec<ExtractedEntity>,
    pub priority_score: u8,
    pub summary: String,
}

pub fn classify_and_enrich(
    item: &mut Item,
    now_epoch: i64,
    blocked_otp_patterns: &[String],
    user_rules: &[UserRuleConfig],
) -> ClassificationOutcome {
    if item.extracted_codes.is_empty()
        && let Some((code, _popup)) = detect_fast_path(item, now_epoch, blocked_otp_patterns)
    {
        item.extracted_codes.push(code);
    }

    let (category, confidence) = score_category(item);
    let mut labels = vec![category.as_str().to_string()];
    if !item.sender_domain.is_empty()
        && let Some(brand) = item.sender_domain.split('.').next()
        && !brand.is_empty()
    {
        labels.push(brand.to_string());
    }

    let (phishing_score, phishing_warning) = evaluate_phishing(item, category);
    if phishing_score >= 60 {
        labels.push("phishing_risk".into());
    }

    let entities = extract_entities(item);
    let priority_score = compute_priority(category, phishing_score, !entities.is_empty());
    let summary = one_line_summary(item);

    item.category = category;
    item.phishing_score = phishing_score;
    item.phishing_warning = phishing_warning.clone();
    item.extracted_entities = entities.clone();
    item.priority_score = priority_score;
    item.summary = Some(summary.clone());

    for l in &labels {
        if !item.labels.contains(l) {
            item.labels.push(l.clone());
        }
    }

    apply_user_rules(item, user_rules, now_epoch);

    ClassificationOutcome {
        category,
        confidence,
        labels: item.labels.clone(),
        uncertain: confidence < 65,
        phishing_score,
        phishing_warning,
        entities,
        priority_score,
        summary,
    }
}

fn score_category(item: &Item) -> (ItemCategory, u8) {
    if !item.extracted_codes.is_empty() {
        return (ItemCategory::CodeOtp, 98);
    }

    let combined = format!("{}\n{}", item.subject, item.body_text).to_lowercase();
    let domain = item.sender_domain.as_str();
    let gm_joined = item.gmail_labels.join(" ").to_ascii_lowercase();

    if SECURITY_KEYWORDS.iter().any(|k| combined.contains(k)) {
        return (ItemCategory::SecurityAlert, 90);
    }

    if SHIPPING_KEYWORDS.iter().any(|k| combined.contains(k)) {
        return (ItemCategory::Shipping, 88);
    }

    if BILLING_KEYWORDS.iter().any(|k| combined.contains(k)) {
        return (ItemCategory::Billing, 86);
    }

    if WORK_KEYWORDS.iter().any(|k| combined.contains(k))
        || matches!(
            domain,
            "github.com" | "gitlab.com" | "linear.app" | "atlassian.net" | "slack.com"
        )
    {
        return (ItemCategory::Work, 84);
    }

    if gm_joined.contains("smartlabel_social")
        || gm_joined.contains("category social")
        || SOCIAL_KEYWORDS.iter().any(|k| combined.contains(k))
        || matches!(
            domain,
            "facebookmail.com" | "linkedin.com" | "x.com" | "discord.com" | "zalo.me"
        )
    {
        return (ItemCategory::Social, 82);
    }

    let has_bulk_header = item.list_unsubscribe_header.is_some()
        || item.precedence_header.as_deref().is_some_and(|p| {
            let pl = p.to_ascii_lowercase();
            pl.contains("bulk") || pl.contains("list")
        });

    if gm_joined.contains("smartlabel_promo")
        || gm_joined.contains("category promotions")
        || PROMO_KEYWORDS.iter().any(|k| combined.contains(k))
        || has_bulk_header
    {
        return (ItemCategory::PromoNewsletter, 80);
    }

    (ItemCategory::Other, 50)
}

pub fn evaluate_phishing(item: &Item, category: ItemCategory) -> (u8, Option<String>) {
    let mut score = 0u8;
    let mut reasons = Vec::new();
    let sender_domain = item.sender_domain.to_ascii_lowercase();
    let sender_lower = item.sender.to_ascii_lowercase();

    if sender_domain.contains("xn--") {
        score = score.saturating_add(65);
        reasons.push("sender domain uses punycode lookalike encoding".to_string());
    }

    // Check brand impersonation in display name vs sender domain.
    for (brand, official_domains) in KNOWN_BRANDS {
        if sender_lower.contains(brand)
            && !sender_domain.is_empty()
            && !official_domains
                .iter()
                .any(|d| sender_domain == *d || sender_domain.ends_with(&format!(".{d}")))
        {
            score = score.saturating_add(75);
            reasons.push(format!(
                "display name mentions `{brand}` but sender domain is `{sender_domain}`"
            ));
        }
    }

    // For OTP, SecurityAlert, and Billing mails, verify link domains match sender root domain.
    if matches!(
        category,
        ItemCategory::CodeOtp | ItemCategory::SecurityAlert | ItemCategory::Billing
    ) && !sender_domain.is_empty()
    {
        let sender_root = root_domain(&sender_domain);
        for link in &item.extracted_links {
            let link_root = root_domain(&link.domain);
            if !link_root.is_empty() && link_root != sender_root {
                score = score.saturating_add(65);
                reasons.push(format!(
                    "sender domain `{sender_domain}` does not match link domain `{}`",
                    link.domain
                ));
                break;
            }
        }
    }

    score = score.min(100);
    let warning = if score >= 60 {
        Some(format!("Phishing warning: {}", reasons.join("; ")))
    } else {
        None
    };
    (score, warning)
}

fn root_domain(domain: &str) -> String {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() >= 3 && parts[parts.len() - 1].len() == 2 && parts[parts.len() - 2].len() <= 3 {
        parts[parts.len() - 3..].join(".")
    } else if parts.len() >= 2 {
        parts[parts.len() - 2..].join(".")
    } else {
        domain.to_string()
    }
}

pub fn extract_entities(item: &Item) -> Vec<ExtractedEntity> {
    let mut out = Vec::new();
    let combined = format!("{} {}", item.subject, item.body_text);
    let tokens: Vec<&str> = combined.split_whitespace().collect();

    for (idx, tok) in tokens.iter().enumerate() {
        let clean = tok.trim_matches(|c: char| matches!(c, ',' | '.' | ';' | ')' | '('));
        if (clean.starts_with('$') || clean.ends_with('đ') || clean.ends_with("VND"))
            && clean.chars().any(|c| c.is_ascii_digit())
        {
            out.push(ExtractedEntity {
                kind: "amount".into(),
                value: clean.to_string(),
                label: "Amount".into(),
            });
        }

        if clean.len() == 10
            && clean.chars().nth(4) == Some('-')
            && clean.chars().nth(7) == Some('-')
            && clean.chars().filter(|c| c.is_ascii_digit()).count() == 8
        {
            out.push(ExtractedEntity {
                kind: "due_date".into(),
                value: clean.to_string(),
                label: "Date".into(),
            });
        }

        let lower = clean.to_ascii_lowercase();
        if matches!(lower.as_str(), "tracking" | "vận" | "pnr" | "booking")
            && let Some(next) = tokens.get(idx + 1)
        {
            let val = next.trim_matches(|c: char| !c.is_ascii_alphanumeric());
            if val.len() >= 6 && val.chars().any(|c| c.is_ascii_digit()) {
                let kind = if lower == "pnr" || lower == "booking" {
                    "booking_code"
                } else {
                    "tracking_number"
                };
                out.push(ExtractedEntity {
                    kind: kind.into(),
                    value: val.to_string(),
                    label: kind.replace('_', " "),
                });
            }
        }
    }
    out
}

fn compute_priority(category: ItemCategory, phishing_score: u8, has_entities: bool) -> u8 {
    let base: u8 = match category {
        ItemCategory::CodeOtp => 98,
        ItemCategory::SecurityAlert => 92,
        ItemCategory::Billing => 80,
        ItemCategory::Shipping => 75,
        ItemCategory::Work => 72,
        ItemCategory::Social => 45,
        ItemCategory::Other => 40,
        ItemCategory::PromoNewsletter => 15,
    };
    let mut score = base;
    if has_entities {
        score = score.saturating_add(5);
    }
    if phishing_score >= 60 {
        score = score.max(95);
    }
    score.min(100)
}

/// Parses a concise rule DSL string:
/// `match sender:github.com keyword:urgent category:work -> label:ops pin snooze:30 auto_archive_days:7`
pub fn parse_user_rule_dsl(name: &str, dsl: &str) -> Result<UserRuleConfig, String> {
    let (lhs, rhs) = dsl
        .split_once("->")
        .ok_or_else(|| "rule DSL must contain `->` separating match and actions".to_string())?;

    let mut rule = UserRuleConfig {
        name: name.to_string(),
        match_sender: None,
        match_keyword: None,
        match_category: None,
        apply_label: None,
        pin: false,
        snooze_minutes: None,
        auto_archive_days: None,
        explicit_delete: false,
    };

    for tok in lhs.split_whitespace() {
        if tok.eq_ignore_ascii_case("match") {
            continue;
        }
        if let Some(v) = tok.strip_prefix("sender:") {
            rule.match_sender = Some(v.to_string());
        } else if let Some(v) = tok.strip_prefix("keyword:") {
            rule.match_keyword = Some(v.to_string());
        } else if let Some(v) = tok.strip_prefix("category:") {
            rule.match_category = Some(ItemCategory::from_str_name(v));
        }
    }

    for tok in rhs.split_whitespace() {
        if let Some(v) = tok.strip_prefix("label:") {
            rule.apply_label = Some(v.to_string());
        } else if tok == "pin" {
            rule.pin = true;
        } else if let Some(v) = tok.strip_prefix("snooze:") {
            rule.snooze_minutes = v.parse().ok();
        } else if let Some(v) = tok.strip_prefix("auto_archive_days:") {
            rule.auto_archive_days = v.parse().ok();
        } else if tok == "delete" {
            rule.explicit_delete = true;
        }
    }

    Ok(rule)
}

pub fn apply_user_rules(item: &mut Item, rules: &[UserRuleConfig], now_epoch: i64) {
    let sender_lower = item.sender.to_ascii_lowercase();
    let combined_lower = format!("{}\n{}", item.subject, item.body_text).to_lowercase();

    for rule in rules {
        if let Some(ref s) = rule.match_sender
            && !sender_lower.contains(&s.to_ascii_lowercase())
        {
            continue;
        }
        if let Some(ref kw) = rule.match_keyword
            && !combined_lower.contains(&kw.to_lowercase())
        {
            continue;
        }
        if let Some(cat) = rule.match_category
            && item.category != cat
        {
            continue;
        }

        if let Some(ref label) = rule.apply_label
            && !item.labels.contains(label)
        {
            item.labels.push(label.clone());
        }
        if rule.pin {
            item.pinned = true;
        }
        if let Some(mins) = rule.snooze_minutes {
            item.state = ItemState::Snoozed;
            item.snoozed_until = Some(now_epoch + i64::from(mins) * 60);
        }
        if let Some(days) = rule.auto_archive_days
            && now_epoch - item.received_at >= i64::from(days) * 86_400
        {
            item.state = ItemState::Archived;
        }
        // Never delete automatically unless the user explicitly created a delete rule.
        if rule.explicit_delete {
            item.state = ItemState::Trash;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{
        answer_mailbox_qa, build_ai_preview, compute_local_embedding, redact_item_for_ai,
        validate_ai_proposal_json,
    };
    use vortex_core::config::AiConfig;
    use vortex_core::model::{ExtractedLink, SourceKind};

    #[test]
    fn classifies_all_categories_and_detects_phishing() {
        let mut phishing_mail = Item::new(
            SourceKind::Email,
            "me@example.com",
            "p-1",
            "PayPal Security <security@evil-login-check.xyz>",
            "Security Alert: Your verification code is 554433",
            "Please verify your account at https://evil-login-check.xyz/verify",
            1_700_000_000,
        );
        phishing_mail.extracted_links.push(ExtractedLink {
            url: "https://evil-login-check.xyz/verify".into(),
            domain: "evil-login-check.xyz".into(),
            is_verification: true,
            is_unsubscribe: false,
        });

        let out = classify_and_enrich(&mut phishing_mail, 1_700_000_000, &[], &[]);
        assert_eq!(out.category, ItemCategory::CodeOtp);
        assert!(out.phishing_score >= 60);
        assert!(out.phishing_warning.is_some());

        let mut billing = Item::new(
            SourceKind::Email,
            "me@example.com",
            "b-1",
            "Vietcombank <info@vcb.com.vn>",
            "Thông báo biến động số dư và hóa đơn",
            "Thanh toán thành công 250.000đ vào ngày 2026-10-28.",
            1_700_000_000,
        );
        let b_out = classify_and_enrich(&mut billing, 1_700_000_000, &[], &[]);
        assert_eq!(b_out.category, ItemCategory::Billing);
        assert!(b_out.entities.iter().any(|e| e.kind == "amount"));
        assert!(b_out.entities.iter().any(|e| e.kind == "due_date"));
    }

    #[test]
    fn user_rules_dsl_applies_without_unrequested_delete() {
        let rule = parse_user_rule_dsl(
            "dev-pin",
            "match sender:github.com category:work -> label:engineering pin",
        )
        .unwrap();
        let mut item = Item::new(
            SourceKind::Email,
            "me@example.com",
            "w-1",
            "GitHub <notifications@github.com>",
            "Pull request #42 merged",
            "All checks passed on pull request.",
            1_700_000_000,
        );
        classify_and_enrich(&mut item, 1_700_000_000, &[], &[rule]);
        assert_eq!(item.category, ItemCategory::Work);
        assert!(item.pinned);
        assert!(item.labels.contains(&"engineering".to_string()));
        assert_ne!(item.state, ItemState::Trash);
    }

    #[test]
    fn ai_redaction_and_sensitive_privacy_gate() {
        let mut item = Item::new(
            SourceKind::Email,
            "me@example.com",
            "ai-1",
            "Bank <alert@vcb.com.vn>",
            "Your OTP is 884920",
            "Card 4532015112830366 charged. Password: supersecret123\n> old quoted line",
            1_700_000_000,
        );
        classify_and_enrich(&mut item, 1_700_000_000, &[], &[]);
        let redacted = redact_item_for_ai(&item);
        assert!(!redacted.contains("884920"));
        assert!(!redacted.contains("4532015112830366"));
        assert!(!redacted.contains("supersecret123"));
        assert!(!redacted.contains("old quoted line"));

        let ext_cfg = AiConfig {
            enabled: true,
            provider: "openai".into(),
            endpoint: "https://api.openai.com".into(),
            allow_sensitive_external: false,
            ..Default::default()
        };
        let preview = build_ai_preview(&item, "classify", &ext_cfg);
        assert!(preview.blocked_reason.is_some());

        let parsed = validate_ai_proposal_json(
            r#"{"category":"billing","summary":"Monthly bill","priority_score":80,"entities":[]}"#,
        )
        .unwrap();
        assert!(parsed.requires_user_confirmation);
    }

    #[test]
    fn embeddings_and_mailbox_qa() {
        let v1 = compute_local_embedding("invoice payment due tomorrow");
        let v2 = compute_local_embedding("invoice payment due soon");
        let sim = vortex_core::store::cosine_similarity(&v1, &v2);
        assert!(sim > 0.5);

        let mut work_item = Item::new(
            SourceKind::Email,
            "me@example.com",
            "qa-1",
            "Alice <alice@company.com>",
            "Project roadmap review",
            "Can we review the Q4 roadmap this week?",
            1_700_000_000,
        );
        work_item.category = ItemCategory::Work;
        let ans = answer_mailbox_qa("who needs a reply this week?", &[work_item]);
        assert!(ans.contains("Alice"));
    }
}
