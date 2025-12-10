
//! Oggetto Lettore
use crate::canale_quantistico::CanaleQuantistico;
use crate::canale_pubblico::CanalePubblico;
use crate::LUNG_MSG;
use rand::Rng;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::ATTIVA_AVVERSARIO;

pub struct Lettore {
    messaggio_quantistico_ricevuto: Vec<(char, u8)>,
    esito_letture: Vec<bool>,
    chiave_grezza: Vec<u8>,
    chiave_simmetrica: Vec<u8>,
}

impl Lettore {
    pub fn new(_lung: usize) -> Self {
        Self {
            messaggio_quantistico_ricevuto: vec![(' ', 0); LUNG_MSG],
            esito_letture: vec![false; LUNG_MSG],
            chiave_grezza: Vec::new(),
            chiave_simmetrica: Vec::new(),
        }
    }

    pub fn leggi_fotoni(&mut self, cq: &Arc<Mutex<CanaleQuantistico>>) {
        println!("[Lettore]: Lettura fotoni avviata");
        let mut rng = rand::thread_rng();
        let mut letti = 0usize;
        while letti < LUNG_MSG {
            // attende fotone
            loop {
                let cq_guard = cq.lock();
                if cq_guard.fotone_in {
                    // se avversario attivo, deve aver letto prima
                    if !ATTIVA_AVVERSARIO || cq_guard.letto_da_avversario {
                        break;
                    }
                }
                drop(cq_guard);
                thread::sleep(Duration::from_millis(1));
            }
            let pol = if rng.gen_bool(0.5) { 'Z' } else { 'X' };
            let val = { cq.lock().lettura_fotone(pol) };
            {
                let mut cq = cq.lock();
                cq.set_fotone_out();
                // pronto per il prossimo fotone
            }
            self.messaggio_quantistico_ricevuto[letti] = (pol, val);
            letti += 1;
        }
        println!("[Lettore]: Lettura fotoni terminata");
    }

    pub fn fine_lettura(&self, cp: &Arc<Mutex<CanalePubblico>>) {
        let mut cp = cp.lock();
        cp.fine_lettura();
    }

    pub fn verifica_polarizzazioni(&mut self, cp: &Arc<Mutex<CanalePubblico>>) {
        // attende pubblicazione pronta
        loop {
            if cp.lock().pubblicazione_pronta { break; }
            thread::sleep(Duration::from_millis(2));
        }
        let pol_tx = { cp.lock().leggi_polarizzazioni() };
        for i in 0..LUNG_MSG {
            self.esito_letture[i] = self.messaggio_quantistico_ricevuto[i].0 == pol_tx[i];
        }
        {
            let mut cp = cp.lock();
            cp.spedizione_sequenza(self.esito_letture.clone());
        }
        println!("[Lettore]: Invio polarizzazioni al canale pubblico completato");
    }

    pub fn prepara_test_e_chiave(&mut self, cp: &Arc<Mutex<CanalePubblico>>) {
        // costruisce chiave_grezza
        self.chiave_grezza.clear();
        for i in 0..LUNG_MSG {
            if self.esito_letture[i] {
                self.chiave_grezza.push(self.messaggio_quantistico_ricevuto[i].1);
            }
        }
        // bit di test: uno ogni 8 partendo dal primo
        let mut test: Vec<u8> = Vec::new();
        self.chiave_simmetrica.clear();
        for (idx, bit) in self.chiave_grezza.iter().enumerate() {
            if idx % 8 == 0 { test.push(*bit); } else { self.chiave_simmetrica.push(*bit); }
        }
        // pubblica test
        {
            let mut cp = cp.lock();
            cp.scrittura_test_avversario(test);
            // Scrive sul terminale che ha spedito il test
            println!("[Lettore]: Selezione valori di test presenza avversario e spedizione al canale pubblico completata");
            // salva per stampa
            cp.dbg_set_lettore_seq(self.messaggio_quantistico_ricevuto.clone());
            cp.dbg_set_chiave_lettore(self.chiave_simmetrica.clone());
        }
    }

    pub fn consuntiva(&mut self, _cp: &Arc<Mutex<CanalePubblico>>) {
        // nessuna azione extra: stampa già gestita in main
    }
}
