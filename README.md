# README File
# Project Video Link
https://drive.google.com/file/d/1uJ7IkFz-ojXyYiBGlHEAN1D5GDx_6mfm/view?usp=sharing
## Documentation
To run this program and Rust on your computer, first download Rust from rustup.rs. Then, download the rust-analyzer extension from VSCode. Next, 
check the download and run the project by running the following commands after navigating to the project folder.
```
rustc --version
rustup update
cargo build
cargo run
```
Cargo comes installed when you install/update Rust. 

This project has 4 scheduling algorithms. First in first out (FIFO), shortest job first (SJF), shortest time to completion first (STCF), and
round robin (RR). Additionally, the average turnaround time and average response time for each scheduling algorithm is also calculated. 
## Design
The design choices for this project are pretty straight forward. There are 6 methods total. 4 for each of the scheduling algorithms and 2 for
the calculation of average turnaround and response time. There is one struct that I created, which is a Process. The Process struct has 4
properties. These are arrival, duration, first_run, and and completion. There are two implementation functions for the struct Process named 
**PartialOrd** and **Ord**. These two functions order the Processes by the duration of each respective process. This is my way of utilizing a
priority queue in this project since I saw the need for one. Additionally, I have also created Vector<Process> that acts like a list. Using
this, I feed in and take the output of functions. A variable named complete makes a regular appearance in each method which takes completed
processes. Workload and jobs/job_queue are always emptied by the end. I use a lot of while loops to loop through the lists and access each Process.
