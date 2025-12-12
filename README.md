
# BB84 – Versione concorrente (Rust)

Simulazione concorrente del protocollo **BB84**.
- Concorrenza con **thread separati** per Scrittore, Lettore e (opzionale) Avversario
- Sincronizzazione tramite **flag** su Canale Pubblico/Quantistico (attesa leggera)
- Stampa in `main` di **tre tabelle**:
  1) Sequenze (Scrittore / Avversario se presente / Lettore)
  2) Chiave finale di Scrittore e Lettore
  3) Statistiche relative ai fotoni persi

## Requisiti
- Rust stable
- Visual Studio Code + estensione **Rust Analyzer**

## Esecuzione (un click)
Apri la cartella in VS Code → `Run Task` → **BB84: Esegui**. Il task lancia `cargo run` e mostra l'output in un pannello dedicato.

## Parametri nel file main.rs
- `LUNG_MSG = 64` (8 byte, 64 fotoni)
- `ATTIVA_AVVERSARIO = false` (metti `true` per attivarlo)

## Note
- Il Lettore attende che `letto_da_avversario = true` (se l'avversario è attivo) prima di leggere ciascun fotone, garantendo l'ordine **Avversario → Lettore**.
- Le sequenze e le chiavi vengono salvate anche in campi *debug* del Canale Pubblico per poterle stampare in `main`.
- Il codice è stato scritto fornendo a Microsoft Copilot una specifica piuttosto dettagliata
