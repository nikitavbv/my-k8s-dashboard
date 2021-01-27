import React from 'react';

import {Pod} from '../types';
import {ContainerRow} from './ContainerRow';
import {Collapsible} from './Collapsible';
import {podStats} from '../utils';

type PodRowProps = {
    pod: Pod,
};

export const PodRow = (props: PodRowProps) => {
    return (
        <Collapsible category='Pod' name={props.pod.name} level={1} stats={podStats(props.pod)}>
            { props.pod.containers.map(container => (<ContainerRow container={container} />)) }
        </Collapsible>
    );
};