
//! Oggetto Scrittore
use crate::canale_quantistico::CanaleQuantistico;
use crate::canale_pubblico::CanalePubblico;
use crate::LUNG_MSG;
use rand::Rng;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Scrittore {
    messaggio_quantistico: Vec<(char, u8)>, // (polarizzazione, valore)
    chiave_grezza: Vec<u8>,
    chiave_simmetrica: Vec<u8>,
}

impl Scrittore {
    pub fn new(_lung: usize) -> Self {
        Self { messaggio_quantistico: Vec::new(), chiave_grezza: Vec::new(), chiave_simmetrica: Vec::new() }
    }

    /// Inizializzazione
    pub fn inizializzazione(&mut self, cp: &Arc<Mutex<CanalePubblico>>) {
        let mut rng = rand::thread_rng();
        for _ in 0..LUNG_MSG {
            let pol = if rng.gen_bool(0.5) { 'Z' } else { 'X' };
            let val: u8 = if rng.gen_bool(0.5) { 1 } else { 0 };
            self.messaggio_quantistico.push((pol, val));
        }
        // salva per stampa in main
        cp.lock().dbg_set_scrittore_seq(self.messaggio_quantistico.clone());
        println!("[Scrittore]: Scelta polarizzazione e valori fotoni completata, avvio spedizione");
    }

    /// Ciclo invio fotoni sul canale quantistico
    pub fn loop_spedizione_fotoni(&self, cq: &Arc<Mutex<CanaleQuantistico>>) {
        for (pol, val) in self.messaggio_quantistico.iter() {
            {
                let mut cq = cq.lock();
                cq.spedizione_fotone(*pol, *val);
                cq.set_fotone_in();
            }
            // attende che venga rimesso OUT dal lettore
            loop {
                if !cq.lock().fotone_in { break; }
                thread::sleep(Duration::from_millis(2));
            }
        }
    }

    /// Pubblicazione polarizzazioni sul canale pubblico
    pub fn pubblicazione_polarizzazione(&self, cp: &Arc<Mutex<CanalePubblico>>) {
        let pols: Vec<char> = self.messaggio_quantistico.iter().map(|(p, _)| *p).collect();
        {
            let mut cp = cp.lock();
            cp.pubblica_polarizzazioni(pols.clone());
        }
        println!("[Scrittore]: Invio polarizzazioni al canale pubblico completato");
    }

    /// Selezione chiave grezza
    pub fn selezione_chiave_grezza(&mut self, cp: &Arc<Mutex<CanalePubblico>>) {
        let seq = { cp.lock().lettura_sequenza() };
        self.chiave_grezza.clear();
        for (i, ok) in seq.iter().enumerate() {
            if *ok {
                self.chiave_grezza.push(self.messaggio_quantistico[i].1);
            }
        }
    }

    /// Selezione bit test e definizione chiave finale
    pub fn selezione_test_e_chiave_finale(&mut self, cp: &Arc<Mutex<CanalePubblico>>) {
        // bit di test (1 ogni 8, a partire dal primo)
        let mut test_scrittore: Vec<u8> = Vec::new();
        for (idx, bit) in self.chiave_grezza.iter().enumerate() {
            if idx % 8 == 0 { test_scrittore.push(*bit); }
        }
        // attende test del lettore
        loop {
            if cp.lock().test_avversario_pronto { break; }
            thread::sleep(Duration::from_millis(2));
        }
        let test_lettore = { cp.lock().lettura_test_avversario() };
        if test_scrittore != test_lettore {
            println!("[Scrittore]: Test presenza avversario positivo. Chiave scartata");
            // chiave simmetrica vuota
            {
                let mut cp = cp.lock();
                cp.dbg_set_chiave_scrittore(Vec::new());
                // se lo Scrittore rileva avversario, anche il Lettore deve cancellare la sua chiave
                cp.dbg_set_chiave_lettore(Vec::new());
            }
            return;
        } else {
            println!("[Scrittore]: Test presenza avversario negativo");
        }
        // Crea chiave simmetrica eliminando i bit usati per il test
        self.chiave_simmetrica.clear();
        for (idx, bit) in self.chiave_grezza.iter().enumerate() {
            if idx % 8 != 0 { self.chiave_simmetrica.push(*bit); }
        }
        let _stamp = self.chiave_simmetrica.iter().map(|v| if *v==1 {'1'} else {'0'}).collect::<String>();
        // println!("[Scrittore]: chiave simmetrica definita {}", stamp);
        cp.lock().dbg_set_chiave_scrittore(self.chiave_simmetrica.clone());
        // Pubblica OK
        cp.lock().chiave_simmetrica_ok();
    }
}
