use reccak::{hash, permutations, Digest, Input, PermutationIterator, reverse_hash::{CHARS, DIGESTS}};
use std::{sync::mpsc, thread, time::Instant};

type WorkerPermutationIterator = std::iter::Take<std::iter::Skip<PermutationIterator>>;

fn reverse_hash(
    permutations: impl Iterator<Item = Input>,
    expected_digest: Digest,
) -> Option<Input> {
    for permutation in permutations {
        let calculated_digest = hash(permutation.clone());
        if calculated_digest == expected_digest {
            return Some(permutation);
        }
    }
    None
}

enum WorkerRequest {
    Shutdown,
    Job {
        permutations: WorkerPermutationIterator,
        expected_digest: Digest,
    },
}

struct WorkerResponse {
    input: Input,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    request: mpsc::Sender<WorkerRequest>,
}

impl Worker {
    pub fn new(id: usize, response_sender: mpsc::Sender<WorkerResponse>) -> Self {
        let (request_sender, request_receiver) = mpsc::channel::<WorkerRequest>();

        println!("Spawning worker {}", id);
        let thread = thread::spawn(move || loop {
            match request_receiver.recv().unwrap() {
                WorkerRequest::Job {
                    permutations,
                    expected_digest,
                } => {
                    let result = reverse_hash(permutations, expected_digest);
                    if let Some(input) = result {
                        response_sender.send(WorkerResponse { input }).unwrap();
                    }
                }
                WorkerRequest::Shutdown => return,
            };
        });
        Self {
            id,
            thread: Some(thread),
            request: request_sender,
        }
    }

    pub fn send_reverse_hash(
        &self,
        permutations: WorkerPermutationIterator,
        expected_digest: Digest,
    ) {
        self.request
            .send(WorkerRequest::Job {
                permutations,
                expected_digest,
            })
            .expect("failed sending work");
    }

    pub fn shutdown(&mut self) {
        self.request.send(WorkerRequest::Shutdown).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}

struct WorkerPool {
    workers: Vec<Worker>,
    response_receiver: mpsc::Receiver<WorkerResponse>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (response_sender, response_receiver) = mpsc::channel::<WorkerResponse>();
        for id in 0..size {
            workers.push(Worker::new(id + 1, response_sender.clone()))
        }
        Self {
            workers,
            response_receiver,
        }
    }

    fn distribute_reverse_hash(&self, input_size: usize, expected_digest: Digest) -> Input {
        let chunk_size = CHARS.len().pow(input_size as u32) / self.workers.len();
        let permutations = permutations(CHARS, input_size);
        for (i, worker) in self.workers.iter().enumerate() {
            let skipped = i * chunk_size;
            let permutations = permutations.clone().skip(skipped).take(chunk_size);
            worker.send_reverse_hash(permutations, expected_digest);
        }
        let WorkerResponse { input } = self.response_receiver.recv().unwrap();
        input
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.shutdown();
        }
    }
}

fn main() {
    let workers = match std::env::var_os("WORKERS") {
        Some(value) => str::parse::<usize>(value.to_str().unwrap())
            .expect("invalid `WORKERS` environment variable"),
        None => num_cpus::get(),
    };

    let worker_pool = WorkerPool::new(workers);

    for &(expected_digest, input_size) in DIGESTS {
        let start = Instant::now();
        let input = worker_pool.distribute_reverse_hash(input_size, expected_digest);
        println!(
            "Reversed hash {:X?}, input is: `{}`, took {:?}",
            expected_digest,
            std::str::from_utf8(input.as_slice()).unwrap(),
            start.elapsed()
        )
    }
}
