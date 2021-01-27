import React, {useState} from 'react';

import {ChevronRight, ChevronDown} from './icons';

export type CollapsibleProps = {
    category: string,
    name: string,
    level: number,
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
        paddingLeft,
    };

    return (
        <div style={{
            borderTop: '1px solid rgb(224, 224, 224)',
            padding: collapsed ? '8px 0' : '8px 0 0 0',
            userSelect: 'none',
            lineHeight: '20px',
            verticalAlign: 'middle',
            boxSizing: 'border-box',
        }}>
            { collapsed
                ? (<ChevronRight onClick={setIsCollapsed.bind(this, false)} style={chevronStyle}/>)
                : (<ChevronDown onClick={setIsCollapsed.bind(this, true)} style={chevronStyle} />)
            }
            <span style={{ lineHeight: '20px', padding: '8px 0 8px 4px', verticalAlign: 'middle' }}>
                { props.category }: { props.name }
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