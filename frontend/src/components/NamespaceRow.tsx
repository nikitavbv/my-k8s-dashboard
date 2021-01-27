import React from 'react';

import { Namespace } from '../types';
import { PodRow } from './PodRow';
import {Collapsible} from './Collapsible';
import {namespaceStats} from '../utils';

export type NamespaceRowProps = {
    namespace: Namespace,
};

export const NamespaceRow = (props: NamespaceRowProps) => {
    return (
        <Collapsible category='Namespace' name={props.namespace.name} level={0} stats={namespaceStats(props.namespace)}>
            { props.namespace.pods.map(pod => (<PodRow pod={pod} />) )}
        </Collapsible>
    );
};
