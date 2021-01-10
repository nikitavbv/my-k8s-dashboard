import React, {useState} from 'react';

import { Namespace } from '../types';
import { PodRow } from './PodRow';
import {ChevronRight} from './icons';

export type NamespaceRowProps = {
    namespace: Namespace,
};

export const NamespaceRow = (props: NamespaceRowProps) => {
    const [collapsed, setIsCollapsed] = useState<Boolean>(true);

    return (
        <div style={{
            borderTop: '1px solid rgb(224, 224, 224)',
            padding: '8px',
            userSelect: 'none',
        }}>
            <ChevronRight onClick={() => setIsCollapsed(!collapsed)}  style={{
                display: 'inline',
                cursor: 'pointer',
                lineHeight: '20px',
            }}/>
            <span style={{ lineHeight: '20px' }}>
                Namespace: { props.namespace.name }
            </span>
            <div style={{
                display: collapsed ? 'none' : 'inherit',
            }}>
                { props.namespace.pods.map(pod => (<PodRow pod={pod} />) )}
            </div>
        </div>
    );
};