export type Namespace = {
    name: string,
    pods: Pod[],
};

export type Pod = {
    name: string,
    containers: Container[],
};

export type Container = {
    name: string,
    usage: Resources | null,
    requests: Resources | null,
    limits: Resources | null,
    total_resources: UsageStats | null,
};

export type Resources = {
    cpu: number, // nanocpus
    memory: number, // kilobytes
};

export type Stats = {
    usage: Resources,
    requests: Resources,
    limits: Resources,
    total_resources: UsageStats,
};

export type UsageStats = {
    total_cpu: number, // nanocpu-seconds
    total_memory: number, // kilobyte-seconds
};