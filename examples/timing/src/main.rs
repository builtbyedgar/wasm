use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::{ Duration, Instant };

struct Clock<F> {
    interval: Duration,
    // callback: F,
    // Envoltura en Arc<Mutex> para compartirlo de manera segura entre hilos y permitir
    // que se llame desde el bucle sin preocuparte por la movilidad.
    callback: Arc<Mutex<F>>,
    should_stop: Arc<Mutex<bool>>,
}

// FnMut
impl<F> Clock<F> where F: FnMut() + Send + Sync + 'static {
    fn new(interval: Duration, callback: F) -> Self {
        Clock {
            interval,
            callback: Arc::new(Mutex::new(callback)),
            should_stop: Arc::new(Mutex::new(false)),
        }
    }

    // Inicia un hilo que ejecuta el bucle principal.
    fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let callback = self.callback.clone();
        let should_stop = self.should_stop.clone();

        // Se encarga de crear un nuevo hilo de ejecución independiente del hilo principal.
        let handle = thread::spawn(move || {
            let mut expected = Instant::now() + self.interval;

            loop {
                // Verifica si se debe detener el bucle.
                if *should_stop.lock().unwrap() {
                    break;
                }

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

        match handle.join() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Thread join error: {:?}", e).into()),
        }
    }

    fn stop(&self) {
        *self.should_stop.lock().unwrap() = true;
    }

    fn change_interval(&mut self, new_interval: Duration) {
        self.interval = new_interval;
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
