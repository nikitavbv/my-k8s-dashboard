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