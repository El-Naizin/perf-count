
pub enum Event {
    /// Total cycles.
    CPUCycles,
    /// Retired instructions.  Be careful, these can be affected by various issues, most notably
    /// hardware interrupt counts.
    Instructions,
    /// Cache accesses. Usually this indicates Last Level Cache accesses but this may vary depending
    /// on your CPU. This may include prefetches and coherency messages; again this depends on the
    /// design of your CPU.
    CacheReferences,

    /// Cache misses. Usually this indicates Last Level Cache misses; this is intended to be used in
    /// conjunction with the [CacheReferences] event to calculate cache miss rates.
    CacheMisses,

    /// Retired branch instructions.  Prior to Linux 2.6.34, this used the wrong event on AMD
    /// processors.
    BranchInstructions,

    /// Mispredicted branch instructions.
    BranchMisses,
}