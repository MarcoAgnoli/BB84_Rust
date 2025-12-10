
//! Oggetto Avversario (opzionale) — legge sempre prima del Lettore
use crate::canale_quantistico::CanaleQuantistico;
use crate::LUNG_MSG;
use rand::Rng;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Avversario {
    msg: Vec<(char, u8)>,
}

impl Avversario {
    pub fn new(_lung: usize) -> Self { Self { msg: vec![(' ', 0); LUNG_MSG] } }

    pub fn leggi_fotoni(&mut self, cq: &Arc<Mutex<CanaleQuantistico>>) {
        let mut rng = rand::thread_rng();
        let mut letti = 0usize;
        while letti < LUNG_MSG {
            // attende nuovo fotone non ancora letto dall'avversario
            loop {
                {
                    let cq_guard = cq.lock();
                    if cq_guard.fotone_in && !cq_guard.letto_da_avversario {
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(1));
            }
            let pol = if rng.gen_bool(0.5) { 'Z' } else { 'X' };
            let val = { cq.lock().lettura_fotone(pol) };
            {
                let mut cq = cq.lock();
                // marca che l'avversario ha letto: ora il Lettore può procedere
                cq.mark_letto_da_avversario();
            }
            self.msg[letti] = (pol, val);
            letti += 1;
        }
        println!("[Avversario]: lettura terminata");
    }

    pub fn debug_pubblica(&self, cp: &Arc<Mutex<crate::canale_pubblico::CanalePubblico>>) {
        cp.lock().dbg_set_avversario_seq(self.msg.clone());
    }
}
