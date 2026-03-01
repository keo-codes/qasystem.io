use std::collections::HashMap;
use std::fs;
use docx_rs::*;

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub date: String,
    pub event: String,
}

#[derive(Debug, Clone)]
pub struct CalendarMonth {
    pub name: String,
    pub events: Vec<CalendarEvent>,
}

#[derive(Debug, Clone)]
pub struct CalendarYear {
    pub year: i32,
    pub months: HashMap<String, CalendarMonth>,
}

pub fn build_calendar(doc_dir: &str, year: i32) -> CalendarYear {
    let mut months: HashMap<String, CalendarMonth> = HashMap::new();

    let month_names = [
        "January","February","March","April","May","June",
        "July","August","September","October","November","December"
    ];

    for m in &month_names {
        months.insert(m.to_string(), CalendarMonth {
            name: m.to_string(),
            events: Vec::new(),
        });
    }

    for entry in fs::read_dir(doc_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().unwrap() == "docx" {
            let bytes = fs::read(&path).unwrap();
            let doc = read_docx(&bytes).unwrap();

            let mut current_month = None;

            for c in doc.document.children {
                if let DocumentChild::Paragraph(p) = c {
                    let mut line = String::new();

                    for r in p.children {
                        if let ParagraphChild::Run(run) = r {
                            for rc in run.children {
                                if let RunChild::Text(t) = rc {
                                    line.push_str(&t.text);
                                }
                            }
                        }
                    }

                    let clean = line.trim().to_string();
                    if clean.is_empty() {
                        continue;
                    }

                    // Month detection
                    for m in &month_names {
                        if clean.to_lowercase().contains(&m.to_lowercase()) {
                            current_month = Some(m.to_string());
                        }
                    }

                    // Event detection (simple heuristic)
                    if let Some(m) = &current_month {
                        if clean.chars().any(|c| c.is_numeric()) && clean.len() > 6 {
                            let event = CalendarEvent {
                                date: format!("{} {}", year, clean),
                                event: clean.clone(),
                            };

                            if let Some(month_block) = months.get_mut(m) {
                                month_block.events.push(event);
                            }
                        }
                    }
                }
            }
        }
    }

    CalendarYear { year, months }
}