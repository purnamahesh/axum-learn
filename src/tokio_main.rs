#[tokio::main]
async fn main() {
    // let players = vec![]
    let racer_1 = F1Racer::new("Mahesh".to_string()); 
    let racer_2 = F1Racer::new("Purna".to_string()); 
    let racer_3 = F1Racer::new("John".to_string()); 
    let h1 = tokio::task::spawn(async {racer_1.await} );
    let h2 = tokio::spawn(async {racer_3.await} );
    let h3 = tokio::spawn(async {racer_2.await} );
    
    let (r1, r2, r3) = tokio::join!(h1, h2, h3);
    println!("Results: {:?}, {:?}, {:?}", r1, r2, r3);
}

struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}

impl F1Racer {
    fn new(name: String) -> F1Racer {
        F1Racer {
            name: name,
            completed_laps: 0,
            laps: 5,
            best_lap_time: 255,
            lap_times: vec![143,165,95,156,234],
        }
    }

    fn do_lap(&mut self) {

        println!("Racer {} started doing lap {}..", self.name, self.completed_laps);
        
        if let Some(lap_time) = self.lap_times.pop() {
            if lap_time < self.best_lap_time {
                self.best_lap_time = lap_time;
            }
        }
        
        self.completed_laps += 1;
    }
}

impl std::future::Future for F1Racer {
    type Output = u8;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed_laps < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        println!("Best lap time of player {} is {}", self.name, self.best_lap_time);
        std::task::Poll::Ready(self.best_lap_time)
    }
}

