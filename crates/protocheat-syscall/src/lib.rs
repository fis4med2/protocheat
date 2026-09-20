use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtMapping {
    pub nt_call: &'static str,
    pub linux_equivalent: &'static str,
    pub note: &'static str,
}

pub fn table() -> Vec<NtMapping> {
    vec![
        NtMapping {
            nt_call: "NtCreateEvent",
            linux_equivalent: "NTSYNC_IOC_CREATE_EVENT",
            note: "Manual and auto reset events map to the ntsync event object.",
        },
        NtMapping {
            nt_call: "NtSetEvent / NtResetEvent / NtPulseEvent",
            linux_equivalent: "NTSYNC_IOC_EVENT_SET / RESET / PULSE",
            note:
                "Pulse needs direct wait queue control, which is why it lives in the kernel driver.",
        },
        NtMapping {
            nt_call: "NtCreateMutant",
            linux_equivalent: "NTSYNC_IOC_CREATE_MUTEX",
            note: "Owner id is passed from user space. Abandoned state uses NTSYNC_IOC_MUTEX_KILL.",
        },
        NtMapping {
            nt_call: "NtCreateSemaphore",
            linux_equivalent: "NTSYNC_IOC_CREATE_SEM",
            note: "Post maps to NTSYNC_IOC_SEM_POST with a 64 object wait limit.",
        },
        NtMapping {
            nt_call: "NtWaitForSingleObject",
            linux_equivalent: "NTSYNC_IOC_WAIT_ANY with count 1",
            note: "Fast path, optimized for wait for any.",
        },
        NtMapping {
            nt_call: "NtWaitForMultipleObjects",
            linux_equivalent: "NTSYNC_IOC_WAIT_ANY / WAIT_ALL",
            note: "Wait for all takes a device wide lock and is rare in games.",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_covers_wait() {
        assert!(table().iter().any(|m| m.nt_call.contains("Wait")));
    }
}
