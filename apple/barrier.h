// Heavy barrier implementation for MacOS
// using inter processor interrupt(IPI) mechanism.

// Check if the heavy membarrier using an inter processor interrupt
// mechanism is supported on the host environment.
//
// An inter processor interrupt mechanism on Apple environments
// is implementable for only x64 and ARM64.
int is_supported();

// Issue a heavy memory barrier.
//
// It flushes write buffers of executing threads of the current process,
// and is equivalent to `membarrier` on latest Linux and `FlushProcessWriteBuffers` on Windows.
int flush_process_write_buffers();