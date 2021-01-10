import React from 'react';

import { Namespace } from '../types';
import { PodRow } from './PodRow';

export type NamespaceRowProps = {
    namespace: Namespace,
};

export const NamespaceRow = (props: NamespaceRowProps) => {
    return (
        <div>
            - Namespace: { props.namespace.name }<br />
            { props.namespace.pods.map(pod => (<PodRow pod={pod} />) )}
        </div>
    );
};