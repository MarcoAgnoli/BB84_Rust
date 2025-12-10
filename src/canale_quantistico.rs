
//! Canale Quantistico persistente con tupla (polarizzazione, valore) e flag Fotone_IN
use rand::Rng;

#[derive(Debug, Clone)]
pub struct CanaleQuantistico {
    pub canale_quantistico: (char, u8),
    pub fotone_in: bool,
    pub letto_da_avversario: bool,
}

impl CanaleQuantistico {
    pub fn new() -> Self {
        Self { canale_quantistico: (' ', 0), fotone_in: false, letto_da_avversario: false }
    }

    /// Spedizione del fotone (solo scrittore)
    pub fn spedizione_fotone(&mut self, pol: char, val: u8) {
        self.canale_quantistico = (pol, val);
        self.letto_da_avversario = false; // nuovo fotone: l'avversario non ha ancora letto
    }

    /// Lettura del fotone (lettore/avversario)
    pub fn lettura_fotone(&mut self, pol_lettura: char) -> u8 {
        let (pol_canale, val_corrente) = self.canale_quantistico;
        if pol_canale == pol_lettura {
            val_corrente
        } else {
            // polarizzazioni diverse: collasso casuale
            let nuovo = rand::thread_rng().gen_range(0..=1);
            self.canale_quantistico.1 = nuovo;
            nuovo
        }
    }

    pub fn set_fotone_in(&mut self) { self.fotone_in = true; }
    pub fn set_fotone_out(&mut self) { self.fotone_in = false; }
    pub fn mark_letto_da_avversario(&mut self) { self.letto_da_avversario = true; }
}
