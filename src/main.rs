
//! Esecuzione concorrente del protocollo BB84 secondo specifica
mod canale_pubblico;
mod canale_quantistico;
mod scrittore;
mod lettore;
mod avversario;

use canale_pubblico::CanalePubblico;
use canale_quantistico::CanaleQuantistico;
use scrittore::Scrittore;
use lettore::Lettore;
use avversario::Avversario;
use std::sync::Arc;
use parking_lot::Mutex;
use std::thread;
use std::time::Duration;

/// LUNG_MSG = 64 fotoni (8 byte), come da specifica
pub const LUNG_MSG: usize = 256;
/// Imposta a true per attivare l’avversario
pub const ATTIVA_AVVERSARIO: bool = true;

fn print_ascii_table_rows(rows: &Vec<Vec<String>>) {
    // calcolo larghezze colonne
    let cols = if rows.is_empty() { 0 } else { rows[0].len() };
    let mut widths = vec![0usize; cols];
    for r in rows {
        for (i, c) in r.iter().enumerate() { widths[i] = widths[i].max(c.len()); }
    }
    // stampa
    for r in rows {
        let mut line = String::new();
        for (i, c) in r.iter().enumerate() {
            let pad = widths[i] - c.len();
            line.push_str("| ");
            line.push_str(c);
            line.push_str(&" ".repeat(pad));
            line.push_str(" ");
        }
        line.push('|');
        println!("{}", line);
        // separatore dopo header
    }
}

fn main() {
    println!("[Main]: Avvio simulazione BB84 concorrente");

    let canale_pubblico = Arc::new(Mutex::new(CanalePubblico::new(LUNG_MSG)));
    let canale_quantistico = Arc::new(Mutex::new(CanaleQuantistico::new()));

    let scrittore = Scrittore::new(LUNG_MSG);
    let lettore = Lettore::new(LUNG_MSG);

    // Thread Scrittore
    let cp_s = canale_pubblico.clone();
    let cq_s = canale_quantistico.clone();
    let h_s = thread::spawn(move || {
        let mut s = scrittore;
        s.inizializzazione(&cp_s);
        s.loop_spedizione_fotoni(&cq_s);
        // aspetta fine lettura
        loop {
            if cp_s.lock().fine_lettura { break; }
            thread::sleep(Duration::from_millis(2));
        }
        s.pubblicazione_polarizzazione(&cp_s);
        // aspetta sequenza ricezione pronta
        loop {
            if cp_s.lock().sequenza_polarizzazioni_pronta { break; }
            thread::sleep(Duration::from_millis(2));
        }
        s.selezione_chiave_grezza(&cp_s);
        s.selezione_test_e_chiave_finale(&cp_s);
        cp_s.lock().processo_terminato();
    });

    // Thread Lettore
    let cp_l = canale_pubblico.clone();
    let cq_l = canale_quantistico.clone();
    let h_l = thread::spawn(move || {
        let mut l = lettore;
        l.leggi_fotoni(&cq_l);
        l.fine_lettura(&cp_l);
        l.verifica_polarizzazioni(&cp_l);
        l.prepara_test_e_chiave(&cp_l);
        // attende processo terminato per stampa
        loop {
            if cp_l.lock().processo_terminato { break; }
            thread::sleep(Duration::from_millis(2));
        }
        l.consuntiva(&cp_l);
    });

    // Thread Avversario (opzionale) — deve leggere PRIMA del lettore
    let h_a = if ATTIVA_AVVERSARIO {
        let cq_a = canale_quantistico.clone();
        let cp_a = canale_pubblico.clone();
        Some(thread::spawn(move || {
            let mut a = Avversario::new(LUNG_MSG);
            a.leggi_fotoni(&cq_a);
            a.debug_pubblica(&cp_a);
        }))
    } else { None };

    h_s.join().unwrap();
    h_l.join().unwrap();
    if let Some(h) = h_a { h.join().unwrap(); }

    // --- STAMPE TABELLE RICHIESTE DALLA SPECIFICA ---
    let cp = canale_pubblico.lock();

    // Tabella 1: sequenze di Scrittore / (Avversario) / Lettore
    println!("\n=== Tabella 1: Sequenze fotoni ===");
    let mut header: Vec<String> = vec!["Scrittore (pol,val)".to_string()];
    if ATTIVA_AVVERSARIO { header.push("Avversario (pol,val)".to_string()); }
    header.push("Lettore (pol,val)".to_string());

    let n = LUNG_MSG; // per righe
    let mut rows: Vec<Vec<String>> = Vec::new();
    rows.push(header);
    for i in 0..n {
        let s = if i < cp.dbg_scrittore.len() {
            format!("({},{})", cp.dbg_scrittore[i].0, cp.dbg_scrittore[i].1)
        } else { "".into() };
        let r = if i < cp.dbg_lettore.len() {
            format!("({},{})", cp.dbg_lettore[i].0, cp.dbg_lettore[i].1)
        } else { "".into() };
        if ATTIVA_AVVERSARIO {
            let a = if i < cp.dbg_avversario.len() {
                format!("({},{})", cp.dbg_avversario[i].0, cp.dbg_avversario[i].1)
            } else { "".into() };
            rows.push(vec![s, a, r]);
        } else {
            rows.push(vec![s, r]);
        }
    }
    print_ascii_table_rows(&rows);

    // Tabella 2: Chiavi finali
    println!("\n=== Tabella 2: Chiavi finali ===");
    let ks: String = cp.dbg_chiave_scrittore.iter().map(|b| if *b==1 {'1'} else {'0'}).collect();
    let kl: String = cp.dbg_chiave_lettore.iter().map(|b| if *b==1 {'1'} else {'0'}).collect();
    let rows2 = vec![
        vec!["Scrittore".into(), ks],
        vec!["Lettore".into(), kl],
    ];
    print_ascii_table_rows(&rows2);

    // Tabella 3: Statistiche riassuntive
    println!("\n=== Tabella 3: Statistiche riassuntive ===");
    let total = LUNG_MSG;
    let selezionati = cp.sequenza_ricezione.iter().filter(|&&b| b).count();
    let scartati_polar = total - selezionati;
    let scartati_test = (selezionati + 7) / 8; // uno ogni 8 nella chiave grezza
    let chiave_finale_len = cp.dbg_chiave_scrittore.len();
    let pct = |v: usize| -> String { format!("{:.2}%", (v as f64 * 100.0) / (total as f64)) };
    let rows3 = vec![
        vec!["Fotoni totali (iniziali)".into(), total.to_string(), "100.00%".into()],
        vec!["Fotoni selezionati (chiave grezza)".into(), selezionati.to_string(), pct(selezionati)],
        vec!["Scartati per differenza polarizzazioni".into(), scartati_polar.to_string(), pct(scartati_polar)],
        vec!["Scartati per test presenza avversario".into(), scartati_test.to_string(), pct(scartati_test)],
        vec!["Lunghezza chiave finale".into(), chiave_finale_len.to_string(), pct(chiave_finale_len)],
    ];
    print_ascii_table_rows(&rows3);

    println!("\n[Main]: Simulazione completata\n");
}
