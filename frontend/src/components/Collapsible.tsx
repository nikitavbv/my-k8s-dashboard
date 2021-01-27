import React, {useState} from 'react';

import {ChevronRight, ChevronDown} from './icons';
import {Stats} from '../types';
import {formatCPUNanos, formatMemoryKilobytes} from '../utils';

export type CollapsibleProps = {
    category: string,
    name: string,
    level: number,
    stats: Stats,
    children: React.ReactNode,
};

export const Collapsible = (props: CollapsibleProps) => {
    const [collapsed, setIsCollapsed] = useState<Boolean>(true);

    const paddingLeft = (8 + props.level * 20) + 'px';
    const chevronStyle = {
        display: 'inline',
        cursor: 'pointer',
        lineHeight: '20px',
        verticalAlign: 'middle',
        flex: '1',
        maxWidth: '20px',
        paddingRight: '4px',
    };

    const column = (text: string | undefined) => (
        <div style={{ display: 'flex', textAlign: 'center', width: '100px', maxWidth: '100px', flex: '1' }}>
            <span style={{ margin: '0 auto'}}>
                { text }
            </span>
        </div>
    );

    return (
        <div style={{
            borderTop: '1px solid rgb(224, 224, 224)',
            padding: collapsed ? '8px 0' : '8px 0 0 0',
            userSelect: 'none',
            lineHeight: '20px',
            verticalAlign: 'middle',
            boxSizing: 'border-box',
        }}>
            <span style={{ lineHeight: '20px', padding: '8px 0 8px 4px', verticalAlign: 'middle', display: 'flex', paddingLeft }}>
                { collapsed
                    ? (<ChevronRight onClick={setIsCollapsed.bind(this, false)} style={chevronStyle}/>)
                    : (<ChevronDown onClick={setIsCollapsed.bind(this, true)} style={chevronStyle} />)
                }
                <span style={{ display: 'flex', width: 'auto', flex: '1', verticalAlign: 'middle', lineHeight: '20px' }}>{ props.category }: { props.name }</span>
                { column(formatCPUNanos(props.stats.usage?.cpu || 0)) }
                { column(formatMemoryKilobytes(props.stats.usage?.memory || 0)) }
                { column(formatCPUNanos(props.stats.requests?.cpu || 0)) }
                { column(formatMemoryKilobytes(props.stats.requests?.memory || 0)) }
                { column(formatCPUNanos(props.stats.limits?.cpu || 0)) }
                { column(formatMemoryKilobytes(props.stats.limits?.memory || 0)) }
                { column('todo') }
                { column('todo') }
            </span>
            <div style={{
                display: collapsed ? 'none' : 'inherit',
                marginTop: '8px',
                boxSizing: 'border-box',
            }}>
                { props.children }
            </div>
        </div>
    )
}