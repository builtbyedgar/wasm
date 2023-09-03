use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{ Duration, Instant };

struct Clock<F> {
    interval: Duration,
    // callback: F,
    // Envoltura en Arc<Mutex> para compartirlo de manera segura entre hilos y permitir 
    // que se llame desde el bucle sin preocuparte por la movilidad.
    callback: Arc<Mutex<F>>,
}

// FnMut
impl<F> Clock<F> where F: FnMut() + Send + Sync + 'static {
    fn new(interval: Duration, callback: F) -> Self {
        Clock { interval, callback: Arc::new(Mutex::new(callback)) }
    }

    // Inicia un hilo que ejecuta el bucle principal.
    fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let callback = self.callback.clone();

        let handle = thread::spawn(move || {
            let mut expected = Instant::now() + self.interval;

            
            loop {
                let drift = Instant::now() - expected;
                expected += self.interval;

                // Bloquear el mutex para acceder a la función de devolución de llamada.
                let mut callback = callback.lock().unwrap();
                callback();

                thread::park_timeout(
                    Duration::from_millis((self.interval - drift).as_millis() as u64)
                );
            }
        });

        handle.join()?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn callback_fn() {
        println!("Callback function called");
    }

    let t = Clock::new(Duration::from_secs(1), callback_fn);
    t.start()?;

    thread::sleep(Duration::from_secs(10));
    Ok(())
}
