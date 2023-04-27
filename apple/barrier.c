// Heavy barrier implementation for MacOS
// using inter processor interrupt(IPI) mechanism.

#include <mach/vm_types.h>
#include <mach/vm_param.h>
#include <mach/mach_port.h>
#include <mach/mach_host.h>
#include <mach/thread_state.h>

// Check if a given expression which involves a system call
// is executed successfully.
#define ASSERT_SUCCESS(expr, ret_on_err) do {   \
    if ((expr) != KERN_SUCCESS) {               \
        return (ret_on_err);                    \
    }                                           \
} while (0)

// Check if the heavy membarrier using an inter processor interrupt
// mechanism is supported on the host environment.
//
// An inter processor interrupt mechanism on Apple environments
// is implementable for only x64 and ARM64.
int is_supported() {
#if defined(__x86_64__) || defined(__aarch64__)
    return 1;
#else
    return 0;
#endif
}

// Issue a heavy memory barrier.
//
// It flushes write buffers of executing threads of the current process,
// and is equivalent to `membarrier` on latest Linux and `FlushProcessWriteBuffers` on Windows.
int flush_process_write_buffers() {
    mach_msg_type_number_t thread_count;
    thread_act_t* thread_acts;

    ASSERT_SUCCESS(task_threads(mach_task_self(), &thread_acts, &thread_count), -1);

    // Iterate through each of the threads in the list.
    for (mach_msg_type_number_t i = 0; i < thread_count; ++i) {
        kern_return_t syscall_succ;

        if (__builtin_available (macOS 10.14, iOS 12, *)) {
            // Request the threads pointer values to force the thread to emit a memory barrier
            size_t registers = 128;
            uintptr_t sp, register_values[128];
            syscall_succ = thread_get_register_pointer_values(thread_acts[i], &sp, &registers, register_values);
        } else {
            // Fallback implementation for older OS versions
#if defined(__x86_64__)
            x86_thread_state64_t thread_state;
            mach_msg_type_number_t count = x86_THREAD_STATE64_COUNT;
            syscall_succ = thread_get_state(thread_acts[i], x86_THREAD_STATE64, (thread_state_t)&thread_state, &count);
#elif defined(__aarch64__)
            arm_thread_state64_t thread_state;
            mach_msg_type_number_t count = ARM_THREAD_STATE64_COUNT;
            syscall_succ = thread_get_state(thread_acts[i], ARM_THREAD_STATE64, (thread_state_t)&thread_state, &count);
#else
            // This path should not be reached!
            // On Rust side, we must check if the heavy barrier is supported
            // by `is_supported` function before using `flush_process_write_buffers`.
            return -2;
#endif
        }

        ASSERT_SUCCESS(syscall_succ, -3);
        ASSERT_SUCCESS(mach_port_deallocate(mach_task_self(), thread_acts[i]), -4);
    }

    // Deallocate the thread list now we're done with it.
    ASSERT_SUCCESS(vm_deallocate(mach_task_self(), (vm_address_t)thread_acts, thread_count * sizeof(thread_act_t)), -5);
    return 0;
}
