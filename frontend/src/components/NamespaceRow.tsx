import React, {useState} from 'react';

import { Namespace } from '../types';
import { PodRow } from './PodRow';
import {ChevronRight} from './icons';
import {ChevronDown} from './icons/ChevronDown';

export type NamespaceRowProps = {
    namespace: Namespace,
};

export const NamespaceRow = (props: NamespaceRowProps) => {
    const [collapsed, setIsCollapsed] = useState<Boolean>(true);

    const chevronStyle = {
        display: 'inline',
        cursor: 'pointer',
        lineHeight: '20px',
    };

    return (
        <div style={{
            borderTop: '1px solid rgb(224, 224, 224)',
            padding: '8px',
            userSelect: 'none',
        }}>
            { collapsed
                ? (<ChevronRight onClick={setIsCollapsed.bind(this, false)} style={chevronStyle}/>)
                : (<ChevronDown onClick={setIsCollapsed.bind(this, true)} style={chevronStyle} />)
            }
            <span style={{ lineHeight: '20px' }}>
                Namespace: { props.namespace.name }
            </span>
            <div style={{
                display: collapsed ? 'none' : 'inherit',
                paddingLeft: '20px',
            }}>
                { props.namespace.pods.map(pod => (<PodRow pod={pod} />) )}
            </div>
        </div>
    );
};
