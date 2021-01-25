import React from 'react';

import {Pod} from '../types';
import {ContainerRow} from './ContainerRow';
import {Collapsible} from './Collapsible';

type PodRowProps = {
    pod: Pod,
};

export const PodRow = (props: PodRowProps) => {
    return (
        <Collapsible category='Pod' name={props.pod.name}>
            { props.pod.containers.map(container => (<ContainerRow container={container} />)) }
        </Collapsible>
    );
};