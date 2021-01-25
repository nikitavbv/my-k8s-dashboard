import React, {useState} from 'react';

import {ChevronRight, ChevronDown} from './icons';

export type CollapsibleProps = {
    category: string,
    name: string,
    children: React.ReactNode,
};

export const Collapsible = (props: CollapsibleProps) => {
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
                { props.category }: { props.name }
            </span>
            <div style={{
                display: collapsed ? 'none' : 'inherit',
                paddingLeft: '20px',
            }}>
                { props.children }
            </div>
        </div>
    )
}