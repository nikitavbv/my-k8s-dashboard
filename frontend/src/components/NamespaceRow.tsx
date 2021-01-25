import React from 'react';

import { Namespace } from '../types';
import { PodRow } from './PodRow';
import {Collapsible} from './Collapsible';

export type NamespaceRowProps = {
    namespace: Namespace,
};

export const NamespaceRow = (props: NamespaceRowProps) => {
    return (
        <Collapsible category='Namespace' name={props.namespace.name}>
            { props.namespace.pods.map(pod => (<PodRow pod={pod} />) )}
        </Collapsible>
    );
};
