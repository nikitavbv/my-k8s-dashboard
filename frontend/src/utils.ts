import {Container, Namespace, Pod, Resources, Stats} from './types';

const emptyResources: Resources = {
    cpu: 0,
    memory: 0,
};

const containerToStats = (container: Container): Stats => ({
    usage: container.usage || emptyResources,
    requests: container.requests || emptyResources,
    limits: container.limits || emptyResources,
});

const sumStats = (a: Stats, b: Stats): Stats => ({
    usage: sumResources(a.usage, b.usage),
    requests: sumResources(a.requests, b.requests),
    limits: sumResources(a.limits, b.limits),
});

const sumResources = (a: Resources, b: Resources) => ({
    cpu: a.cpu + b.cpu,
    memory: a.memory + b.memory,
});

export const podStats = (pod: Pod): Stats => pod.containers.map(containerToStats).reduce(sumStats);
export const namespaceStats = (namespace: Namespace): Stats => namespace.pods.map(podStats).reduce(sumStats);

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

const formatMemoryGigabytes = (gigabytes: number) => `${Math.round(gigabytes*10)/10}GB`;

export const formatCPUNanos = (nanocpus: number) => {
    const millis = nanocpus / 100000;
    return `${Math.round(millis * 10)/10}m`;
};