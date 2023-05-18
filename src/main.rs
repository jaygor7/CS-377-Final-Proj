use std::collections::BinaryHeap;
use std::cmp::Reverse;

//defining the struct and deriving items to later be used
#[derive(Debug, Eq, PartialEq, Clone)]
struct Process {
    arrival: i32,
    duration: i32,
    first_run: i32,
    completion: i32,
}

//ordering the queues by duration - mainly used for job queues in sjf and stcf
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Process) -> Option<std::cmp::Ordering> {
        Some(self.duration.cmp(&other.duration))
    }
}

impl Ord for Process {
    fn cmp(&self, other: &Process) -> std::cmp::Ordering {
        self.duration.cmp(&other.duration)
    }
}

//FIFO scheduling algorithm 
fn fifo(workload: &mut Vec<Process>) -> Vec<Process> {
    let mut complete: Vec<Process> = Vec::new();
    let mut runtime = 0;
    runtime += workload[0].arrival;
    while !workload.is_empty() {
        let mut process = workload.remove(0);
        process.first_run = runtime;
        process.completion = runtime + process.duration;
        runtime = process.completion;
        complete.push(process);
    }

    complete
}

//SJF scheduling algorithm
fn sjf(workload: &mut Vec<Process>) -> Vec<Process> {
    let mut complete: Vec<Process> = Vec::new();
    let mut runtime = 0;
    let mut job_queue: BinaryHeap<Reverse<Process>> = BinaryHeap::new();
    runtime += workload[0].arrival;
    //while the workload or job queue are not empty 
    while !(workload.is_empty() && job_queue.is_empty()) {
        while !workload.is_empty() && workload[0].arrival <= runtime {
            job_queue.push(Reverse(workload.remove(0)));   
        }
        if !workload.is_empty() && job_queue.is_empty() {
            runtime = workload[0].arrival;
            job_queue.push(Reverse(workload.remove(0)));
        }
        if let Some(mut process) = job_queue.pop().map(|r| r.0) {
            process.first_run = runtime;
            process.completion = process.duration + runtime;
            runtime = process.completion;
            complete.push(process)
        }
    }
    complete
}

//STCF scheduling algorithm
fn stcf(workload: &mut Vec<Process>) -> Vec<Process> {
    let mut complete: Vec<Process> = Vec::new();
    let mut runtime = 0;
    let mut job_queue: BinaryHeap<Reverse<Process>> = BinaryHeap::new();
    runtime += workload[0].arrival;
    //add processes that have same, lowest arrival time
    while !workload.is_empty() && workload[0].arrival == runtime {
            let mut process = workload.remove(0);
            process.first_run = -1;
            job_queue.push(Reverse(process));
    }
    loop {
        if let Some(Reverse(mut p)) = job_queue.pop() {
            if p.first_run == -1 {
                p.first_run = runtime;
            }
            runtime += 1;
            p.duration -= 1;
            if p.duration == 0 {
                p.completion = runtime;
                if p.arrival == 0 && complete.is_empty() {
                    p.first_run = 0;
                }
                complete.push(p);
            } else {
                job_queue.push(Reverse(p));
            }
        }
        //add all the processes that might have arrived
        while !workload.is_empty() && workload[0].arrival == runtime {
            let mut process = workload.remove(0);
            process.first_run = -1;
            job_queue.push(Reverse(process));
        }
        if job_queue.is_empty() {
            break;
        }
    }
    complete
}

//RR scheduling algorithm
fn rr(workload: &mut Vec<Process>, time_slice: i32) -> Vec<Process> {
    let mut complete: Vec<Process> = Vec::new();
    let mut jobs: Vec<Process> = Vec::new();
    let mut runtime = 0;
    runtime += workload[0].arrival;
    //add processes that have lowest, same arrival time
    while !workload.is_empty() && workload[0].arrival == runtime {
        let mut process = workload.remove(0);
        process.first_run = -1;
        jobs.push(process);
        jobs.sort_by(|a, b| {
            a.arrival.cmp(&b.arrival).then_with(|| a.duration.cmp(&b.duration))
        });
    }
    loop {
        let mut p = jobs.remove(0);
        if p.first_run == -1 {
            p.first_run = runtime;
        }
        //if duration is less than time slice
        if time_slice > p.duration {
            runtime += p.duration;
            p.duration = 0;
        } else {
            runtime += time_slice;
            p.duration -= time_slice;
        }
        if p.duration == 0 {
            //once the process is completed
            p.completion = runtime;
            complete.push(p);
            while !workload.is_empty() && workload[0].arrival <= runtime {
                let mut process = workload.remove(0);
                process.first_run = -1;
                jobs.push(process);
            }
        } else {
            //add process back to the list
            while !workload.is_empty() && workload[0].arrival <= runtime {
                let mut process = workload.remove(0);
                process.first_run = -1;
                jobs.push(process);
            }
            jobs.push(p);
        }   
        if jobs.is_empty() {
            break;
        }
    }
    complete
}

//Calculates the average turnaround time
fn avg_turnaround(processes: &mut Vec<Process>) -> f64 {
    let mut sum: f64 = 0.0;
    let proc_num: f64 = processes.len() as f64;
    //for all processes
    while !processes.is_empty() {
        let p = processes.remove(0);
        let turnaround: f64 = (p.completion - p.arrival).into();
        sum += turnaround;
    }
    sum/proc_num
}

//Calculates the average response time
fn avg_response(processes: &mut Vec<Process>) -> f64 {
    let mut sum: f64 = 0.0;
    let proc_num: f64 = processes.len() as f64;
    //for all processes
    while !processes.is_empty() {
        let p = processes.remove(0);
        let resp: f64 = (p.first_run - p.arrival).into();
        sum += resp;
    }
    sum/proc_num
}

fn main() {
    //Creates workloads to test from
    let mut workload1: Vec<Process> = Vec::new();
    workload1.push(Process {
        arrival: 0,
        duration: 6,
        first_run: 0,
        completion: 0,
    });
    workload1.push(Process {
        arrival: 1,
        duration: 4,
        first_run: 0,
        completion: 0,
    });
    workload1.push(Process {
        arrival: 2,
        duration: 3,
        first_run: 0,
        completion: 0,
    });

    //sorts workload by arrival time
    workload1.sort_by(|a, b| {
        a.arrival.cmp(&b.arrival).then_with(|| a.duration.cmp(&b.duration))
    });

    let fifo_processes = fifo(&mut workload1.clone());
    let sjf_processes = sjf(&mut workload1.clone());
    let stcf_processes = stcf(&mut workload1.clone());
    let rr_processes = rr(&mut workload1.clone(), 2);

    //output to user
    println!("FIFO Scheduling:");
    for process in fifo_processes.clone() {
        println!("{:?}", process);
    }
    println!("Average Response Time: {}", avg_response(&mut fifo_processes.clone()));
    println!("Average Turnaround Time: {}", avg_turnaround(&mut fifo_processes.clone()));

    println!("SJF Scheduling:");
    for process in sjf_processes.clone() {
        println!("{:?}", process);
    }
    println!("Average Response Time: {}", avg_response(&mut sjf_processes.clone()));
    println!("Average Turnaround Time: {}", avg_turnaround(&mut sjf_processes.clone()));

    println!("STCF Scheduling:");
    for process in stcf_processes.clone() {
        println!("{:?}", process);
    }
    println!("Average Response Time: {}", avg_response(&mut stcf_processes.clone()));
    println!("Average Turnaround Time: {}", avg_turnaround(&mut stcf_processes.clone()));

    println!("RR Scheduling:");
    for process in rr_processes.clone() {
        println!("{:?}", process);
    }
    println!("Average Response Time: {}", avg_response(&mut rr_processes.clone()));
    println!("Average Turnaround Time: {}", avg_turnaround(&mut rr_processes.clone()));
}