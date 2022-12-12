# Rust Universal Machine
Authored by Christian Tropeano and Nick Tremblay
Special thanks to TA Isaac and TA Connor for their help with the assignment 

## About 
Together, these modules *successfully implement* an emulation of a universal machine- referred to herein as rum. The machine accepts 14 different instructions, in any sequence, which it can use to compute any computable sequence, per the criteria for computability Alan Turing specified when he conceptualized the universal machine. This implementation of rum follows our design for the most part. 

While we based our design on the specification provided to us by Prof. Daniels, so as to run benchmarks successfully, we did not fully honor the methods listed in the design. We predicted some helper functions being useful, but later discovered it was easier to abandon some degree of abstraction to write operations inline, for the abstractions didn’t meet the design’s criteria for what makes an abstraction worthwhile. Some functions and singletons were also better suited to modules other than what was specified in the design. Namely, the program counter. Additionally, we decided to merge the registers and operations modules into one, misleadingly named registers. 

## Architecture
First of the modules that we used is main.rs, referred to in the design as the rum module, calls the functions to create a new UM, boot it, and execute instructions through the operations module. It includes the instantation of the UM struct, as well as some runtime information. It also includes a debug feature, enabled through a flag, which is the core of our testing strategy. The debug feature allows rum a level of verbosity that gives developers an in-depth understanding of what the machine state looks like through each frame of the main application call-stack. 

The second module rum implements is binary.rs, which initializes the UM struct and defines some of its behavior. All binary needs to know is what a UM is, how it should be initialized, and some details about its behavior: how it could instantiate itself, boot, and fetch or run instructions. This module is also responsible for parsing binary words and appropriately type-casting the information to be loaded into rum’s 0th memory segment, as well as declaring the UM struct such that it can be instantiated by UM and passed-by-reference to all modules lower in heirarchy. 

The third and last module is operations.rs. This is where a parsed word is matched to an instruction and executed. This module needs to know what an instruction is, how to derive the Opcode, among other parameters, and how to match it to the correct function representation of the operation. Additionally, registers needs access to the instance of the UM struct, so as to be able to access and overwrite registers and memory.


## Statistics 
Rum took 11.98 seconds seconds real-time to run 50 million instructions and 11.62 seconds user-time. We know this thorugh linux's timing feature, as timing at the host OS level brings another level of accuracy to our metrics. This linux feature was applied to midmark.um, used in this case a benchmark, containing 87 million instructions. And so, some basic arithmetic gave us the figures above. 

We spent 5 hours analyzing the assignment.

We spent 10 hours preparing our design.

We spent 25 hours solving the problem after our analysis, with 20 hours of stand-up development and an additional 5 hours of debugging.