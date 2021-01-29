import {Container, Namespace, Pod, Resources, Stats, UsageStats} from './types';

const emptyResources: Resources = {
    cpu: 0,
    memory: 0,
};

const emptyTotalResources: UsageStats = {
    total_cpu: 0,
    total_memory: 0,
}

const containerToStats = (container: Container): Stats => ({
    usage: container.usage || emptyResources,
    requests: container.requests || emptyResources,
    limits: container.limits || emptyResources,
    total_resources: container.total_resources || emptyTotalResources,
});

const sumStats = (a: Stats, b: Stats): Stats => ({
    usage: sumResources(a.usage, b.usage),
    requests: sumResources(a.requests, b.requests),
    limits: sumResources(a.limits, b.limits),
    total_resources: sumTotalResources(a.total_resources, b.total_resources),
});

const sumResources = (a: Resources, b: Resources) => ({
    cpu: a.cpu + b.cpu,
    memory: a.memory + b.memory,
});

const sumTotalResources = (a: UsageStats, b: UsageStats): UsageStats => ({
    total_cpu: a.total_cpu + b.total_cpu,
    total_memory: a.total_memory + b.total_memory,
});

export const podStats = (pod: Pod): Stats => pod.containers.map(containerToStats).reduce(sumStats);
export const namespaceStats = (namespace: Namespace): Stats => namespace.pods.map(podStats).reduce(sumStats);

export const compareNamespaces = (first: Namespace, second: Namespace) => compareStats(namespaceStats(first), namespaceStats(second));
export const comparePods = (first: Pod, second: Pod) => compareStats(podStats(first), podStats(second));
export const compareContainers = (first: Container, second: Container) => compareStats(containerToStats(first), containerToStats(second));

const compareStats = (first: Stats, second: Stats) => (statsToScore(second) - statsToScore(first));
const statsToScore = (stats: Stats) => stats.total_resources.total_cpu / Math.pow(10, 9) + (stats.total_resources.total_memory / (4 * 1024 * 1024));

export const formatMemoryKilobytesSeconds = (kilobytesSeconds: number) => formatMemoryKilobytes(kilobytesSeconds) + '-s';

export const formatMemoryKilobytes = (kilobytes: number) => {
    if (kilobytes < 1024) {
        return `${Math.round(kilobytes)}KB`;
    }

    return formatMemoryMegabytes(kilobytes / 1024);
};

const formatMemoryMegabytes = (megabytes: number) => {
    if (megabytes < 1024) {
        return `${Math.round(megabytes*10)/10}MB`;
    }

    return formatMemoryGigabytes(megabytes / 1024);
};

const formatMemoryGigabytes = (gigabytes: number) => {
    if (gigabytes < 1024) {
        return `${Math.round(gigabytes*10)/10}GB`;
    }

    return formatMemoryTerabytes(gigabytes / 1024);
}

const formatMemoryTerabytes = (terabytes: number) => `${Math.round(terabytes * 10)/10}TB`;

export const formatCPUNanosSeconds = (nanocpusSeconds: number) => formatCPUNanos(nanocpusSeconds) + '-s';

export const formatCPUNanos = (nanocpus: number) => {
    const millis = nanocpus / 100000;

    if (millis === 0) {
        return millis.toString();
    }

    if (millis < 100) {
        return `${Math.round(millis * 10) / 10}m`;
    }

    return formatCPUs(millis / 1000);
};

export const formatCPUs = (cpus: number) => {
    if (cpus < 1000) {
        return `${Math.round(cpus * 10) / 10}cpu`;
    }

    return formatKiloCPUs(cpus / 1000);
};

export const formatKiloCPUs = (cpus: number) => {
    return `${Math.round(cpus * 10) / 10}Kcpu`;
};