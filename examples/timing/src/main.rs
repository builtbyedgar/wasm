use std::sync::mpsc;
use std::thread;
use std::time::{ Duration, Instant };

mod sequencer;

// Opciones del secuenciador
struct Options {
    steps: u8,
    tempo: u8,
}

// Definimos un evento custom
struct Event {
    // Tiempo en milisegundos
    t: u128,
}

// Main function
fn main() {
    // Crear un canal de comunicación entre hilos
    // (multiple-producer, single-consumer).
    // tx es el extremo de envío (sender)
    // rx es el extremo de recepción (receiver).
    // tx se utiliza para enviar datos al canal, y rx se utiliza para recibir.
    let (tx, rx) = mpsc::channel();

    // Esta marcianada es una práctica bastante común en Rust y se utiliza para limitar el
    // alcance de la variable `tx` y controlorar su duración de vida. Esto garantiza que
    // los recursos sean liberados de forma adecuada y no se retengan mas tiempo del necesario.
    {
        // Crea un clon del extremo de envío.
        // Util para tener múltiples propietarios que puedan enviar datos al canal.
        // Al clonarlo puedo pasarlo a diferentes hilos o partes del programa, y cada uno
        // podrá enviar datos al mismo canal de forma independiente.
        let tx_clone = mpsc::Sender::clone(&tx);

        let mut seq = sequencer::new(120.0);
        // Crea un nuevo subproceso y comienza a ejecutar el código dentro del bloque de cierre.
        // El subproceso se inicia con un nuevo hilo y se le asigna a la variable sub.
        let sub = thread::spawn(move || {
            let mut tick = Instant::now();

            loop {
                // spera hasta que el tiempo actual sea mayor que el valor en tick.
                while Instant::now() < tick {
                    // Mientras tanto, se usa thread::yield_now() para permitir
                    // que otros hilos se ejecuten.
                    thread::yield_now();
                }

                // Enviar evento al canal de comunicación entre subprocesos.
                let _ = tx.send(Event {
                    t: (Instant::now() - tick).as_micros(),
                });

                let accuracy = Duration::from_millis(1);
                // Incrementar en la instancia e igualarlo
                seq.tick();
                tick += seq.tick;

                // Establece el subproceso en un estado de espera.
                thread::park_timeout(tick - accuracy - Instant::now());
            }
        });



        // TODO: dev only
        // Espera durante un tiempo para que el subproceso tenga tiempo de ejecutarse.
        thread::sleep(Duration::from_secs(3));
        // Asegúra que el subproceso termine antes de salir.
        let _ = sub.join();
    }
}
