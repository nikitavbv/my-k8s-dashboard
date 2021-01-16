import React, {useState} from 'react';

import {Pod} from '../types';
import {ContainerRow} from './ContainerRow';
import {ChevronRight} from './icons';
import {ChevronDown} from './icons/ChevronDown';

type PodRowProps = {
    pod: Pod,
};

export const PodRow = (props: PodRowProps) => {
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
                Pod: { props.pod.name }
            </span>
            <div style={{
                display: collapsed ? 'none' : 'inherit',
            }}>
                { props.pod.containers.map(container => (<ContainerRow container={container} />)) }
            </div>
        </div>
    );
};