use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("❌ Errore: Specifica il percorso del file da analizzare.");
        println!("Uso: cargo run -- <percorso_file>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("🔍 [AI Code Firewall Enterprise & Legal v0.3] Analisi conformità per: {}...", file_path);

    let code_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("❌ Errore: Impossibile leggere il file specificato.");
            std::process::exit(1);
        }
    };

    // 1. Suite di Regole per Vulnerabilità Tecniche Critiche
    let technical_rules = vec![
        ("Hardcoded Secret / API Key", Regex::new(r#"(?i)(api[_-]?key|password|secret|token)\s*=\s*['"][a-zA-Z0-9_\-]{16,}['"]"#).unwrap()),
        ("SQL Injection (Direct Concatenation)", Regex::new(r#"(?i)execute\s*\(\s*['"].*SELECT.*%s.*['"]"#).unwrap()),
        ("Command Injection (RCE)", Regex::new(r#"(?i)(os\.system|subprocess\.Popen|eval|exec)\s*\("#).unwrap()),
    ];

    // 2. Suite di Regole per Rischio Legale e Violazione Copyright / Licenze Tossiche
    let legal_rules = vec![
        ("Copyleft Violation Risk (Known Patented Snippet)", Regex::new(r#"(?i)(专利|proprietary_patent_block|restricted_hash_match|gpl_forced_snippet)"#).unwrap()),
        ("Unauthorized Corporate IP Leak", Regex::new(r#"(?i)(internal_bank_routing|confidential_algorithm_v2)"#).unwrap()),
    ];

    let mut violations_found = 0;

    println!("\n--- [CONTROLLI DI SICUREZZA TECNICA] ---");
    for (rule_name, re) in &technical_rules {
        if re.is_match(&code_content) {
            println!("🚨 [TECHNICAL VULNERABILITY] {}", rule_name);
            violations_found += 1;
        }
    }

    println!("\n--- [CONTROLLI DI CONFORMITÀ LEGALE E COPYRIGHT] ---");
    for (rule_name, re) in &legal_rules {
        if re.is_match(&code_content) {
            println!("⚖️ [LEGAL / IP RISK] {}", rule_name);
            violations_found += 1;
        }
    }

    if violations_found > 0 {
        println!("\n⛔ FIREWALL BLOCCATO: Rilevate {} violazioni (Tecniche o Legali). Codice respinto per protezione aziendale.", violations_found);
        std::process::exit(1);
    } else {
        println!("\n✅ Conformità approvata: Nessun rischio tecnico o legale rilevato nel codice generato dall'IA.");
        std::process::exit(0);
    }
}