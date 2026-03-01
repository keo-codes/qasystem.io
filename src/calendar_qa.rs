use crate::calendar::CalendarYear;
use std::collections::HashSet;

pub fn calendar_answer(cal: &CalendarYear, question: &str) -> String {
    let q_norm = normalize(question);
    let q_tokens = tokenize(&q_norm);

    let mut best_score = 0i32;
    let mut best_match: Option<(String, String)> = None;

    // ---------------- EVENT MATCHING ----------------
    for (_month_name, month) in &cal.months {
        for ev in &month.events {
            let ev_text_norm = normalize(&ev.event);
            let ev_date_norm = normalize(&ev.date);

            let ev_tokens = tokenize(&ev_text_norm);
            let date_tokens = tokenize(&ev_date_norm);

            let mut score = 0;

            // Token overlap scoring
            score += overlap_score(&q_tokens, &ev_tokens) * 5;
            score += overlap_score(&q_tokens, &date_tokens) * 5;

            // Month relevance
            let m_norm = normalize(&month.name);
            if q_norm.contains(&m_norm) {
                score += 5;
            }

            // Date intent
            if q_norm.contains("when") || q_norm.contains("date") {
                if ev.date.chars().any(|c| c.is_numeric()) {
                    score += 4;
                }
            }

            // Year relevance
            if q_norm.contains("2026") && ev_date_norm.contains("2026") {
                score += 4;
            }

            if score > best_score {
                best_score = score;
                best_match = Some((ev.date.clone(), ev.event.clone()));
            }
        }
    }

    // -------- RETURN BEST EVENT --------
    if let Some((date, event)) = best_match {
        if best_score > 0 {
            return format!("{} - {}", date, event);
        }
    }

    // ---------------- MONTH QUERIES ----------------
    for (month_name, month) in &cal.months {
        let m_norm = normalize(month_name);
        if q_norm.contains(&m_norm) {
            if month.events.is_empty() {
                return format!("No events found for {}", month_name);
            }

            let mut out = String::new();
            for ev in &month.events {
                out.push_str(&format!("{} - {}\n", ev.date, ev.event));
            }
            return out.trim().to_string();
        }
    }

    // ---------------- YEAR QUERIES ----------------
    if q_norm.contains("2026") || q_norm.contains("year") {
        let mut out = String::new();
        for (_m, month) in &cal.months {
            for ev in &month.events {
                out.push_str(&format!("{} - {}\n", ev.date, ev.event));
            }
        }
        return out.trim().to_string();
    }

    "No matching information found in the document.".to_string()
}

/* ---------------- UTILITIES ---------------- */

fn normalize(text: &str) -> String {
    text.to_lowercase()
        .replace("’", "")
        .replace("'", "")
        .replace(",", "")
        .replace(".", "")
        .replace("|", "")
        .replace("?", "")
        .replace("!", "")
}

fn tokenize(text: &str) -> HashSet<String> {
    text.split_whitespace()
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() > 2)
        .collect()
}

fn overlap_score(a: &HashSet<String>, b: &HashSet<String>) -> i32 {
    a.intersection(b).count() as i32
}