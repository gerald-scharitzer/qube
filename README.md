# qube

**Quantum Operating System**

Schedule quantum computing workload (processes) on computers (processors).

# Processor

Each quantum computing service that can run quantum programs is a processor. The API of the service is the instruction set of the processor.

# Process

Each quantum program and its data constitutes a process that can run on a compatible and available processor.

**Quantum Orchestrator**

Schedule quantum computing workload (pods) on computers (nodes).

Kubernetes can be considered an operating system for OCI containers.

# Processor (Node)

Accept workload that

    1. is encoded in a readable data structure (image)
    2. can be processed
    3. will be processed.

# Process (Pod)

Defines an interface that supports the following states.

    1. READY to run on a processor
    2. RUNNING on a processor
    3. DONE
