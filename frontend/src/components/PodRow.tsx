import React from 'react';

import {Pod} from '../types';
import {ContainerRow} from './ContainerRow';

type PodRowProps = {
    pod: Pod,
};

export const PodRow = (props: PodRowProps) => {
    return (
        <div>
            ---- Pod: { props.pod.name }
            { props.pod.containers.map(container => (<ContainerRow container={container} />)) }
        </div>
    );
};