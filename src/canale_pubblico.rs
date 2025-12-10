
//! Canale Pubblico secondo specifica (condiviso fra thread)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanalePubblico {
    pub canale_pubblico: Vec<char>,              // polarizzazioni pubblicate dallo scrittore
    pub sequenza_ricezione: Vec<bool>,           // esito per ciascun fotone
    pub pubblicazione_pronta: bool,
    pub fine_lettura: bool,
    pub sequenza_polarizzazioni_pronta: bool,
    pub test_avversario_pronto: bool,
    pub chiave_simmetrica_ok: bool,
    pub processo_terminato: bool,
    pub test_avversario: Vec<u8>,                // bit di test del lettore
    // --- Campi debug per stampa in main ---
    pub dbg_scrittore: Vec<(char, u8)>,
    pub dbg_avversario: Vec<(char, u8)>,
    pub dbg_lettore: Vec<(char, u8)>,
    pub dbg_chiave_scrittore: Vec<u8>,
    pub dbg_chiave_lettore: Vec<u8>,
}

impl CanalePubblico {
    pub fn new(lung: usize) -> Self {
        Self {
            canale_pubblico: vec![' '; lung],
            sequenza_ricezione: vec![false; lung],
            pubblicazione_pronta: false,
            fine_lettura: false,
            sequenza_polarizzazioni_pronta: false,
            test_avversario_pronto: false,
            chiave_simmetrica_ok: false,
            processo_terminato: false,
            test_avversario: Vec::new(),
            dbg_scrittore: Vec::new(),
            dbg_avversario: Vec::new(),
            dbg_lettore: Vec::new(),
            dbg_chiave_scrittore: Vec::new(),
            dbg_chiave_lettore: Vec::new(),
        }
    }

    // Pubblicazione polarizzazione dei fotoni trasmessi
    pub fn pubblica_polarizzazioni(&mut self, pol: Vec<char>) {
        self.canale_pubblico = pol;
        self.pubblicazione_pronta = true;
    }

    // Lettura polarizzazioni
    pub fn leggi_polarizzazioni(&mut self) -> Vec<char> {
        self.pubblicazione_pronta = false;
        self.canale_pubblico.clone()
    }

    // Fine lettura
    pub fn fine_lettura(&mut self) { self.fine_lettura = true; }

    // Spedizione sequenza ricezione fotoni
    pub fn spedizione_sequenza(&mut self, seq: Vec<bool>) {
        self.sequenza_ricezione = seq;
        self.sequenza_polarizzazioni_pronta = true;
    }

    // Lettura sequenza ricezione fotoni
    pub fn lettura_sequenza(&mut self) -> Vec<bool> {
        self.sequenza_polarizzazioni_pronta = false;
        self.sequenza_ricezione.clone()
    }

    // Scrittura test avversario
    pub fn scrittura_test_avversario(&mut self, test: Vec<u8>) {
        self.test_avversario = test;
        self.test_avversario_pronto = true;
    }

    // Lettura test avversario
    pub fn lettura_test_avversario(&mut self) -> Vec<u8> {
        self.test_avversario_pronto = false;
        self.test_avversario.clone()
    }

    // Chiave simmetrica ok
    pub fn chiave_simmetrica_ok(&mut self) { self.chiave_simmetrica_ok = true; }

    // Processo terminato
    pub fn processo_terminato(&mut self) { self.processo_terminato = true; }

    // --- Debug API ---
    pub fn dbg_set_scrittore_seq(&mut self, v: Vec<(char, u8)>) { self.dbg_scrittore = v; }
    pub fn dbg_set_avversario_seq(&mut self, v: Vec<(char, u8)>) { self.dbg_avversario = v; }
    pub fn dbg_set_lettore_seq(&mut self, v: Vec<(char, u8)>) { self.dbg_lettore = v; }
    pub fn dbg_set_chiave_scrittore(&mut self, v: Vec<u8>) { self.dbg_chiave_scrittore = v; }
    pub fn dbg_set_chiave_lettore(&mut self, v: Vec<u8>) { self.dbg_chiave_lettore = v; }
}
