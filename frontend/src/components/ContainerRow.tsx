import React from 'react';
import {Container} from '../types';
import {formatCPUNanos, formatCPUNanosSeconds, formatMemoryKilobytes, formatMemoryKilobytesSeconds} from '../utils';

export type ContainerRowProps = {
    container: Container,
};

export const ContainerRow = (props: ContainerRowProps) => {
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
            padding: '8px 0 8px 52px',
            width: '100%',
            display: 'flex',
            boxSizing: 'border-box',
        }}>
            <span style={{ display: 'flex', width: 'auto', flex: '1' }}>Container: {props.container.name}</span>
            { column(formatCPUNanos(props.container.usage?.cpu || 0)) }
            { column(formatMemoryKilobytes(props.container.usage?.memory || 0)) }
            { column(formatCPUNanos(props.container.requests?.cpu || 0)) }
            { column(formatMemoryKilobytes(props.container.requests?.memory || 0)) }
            { column(formatCPUNanos(props.container.limits?.cpu || 0)) }
            { column(formatMemoryKilobytes(props.container.limits?.memory || 0)) }
            { column(formatCPUNanosSeconds(props.container.total_resources?.total_cpu || 0)) }
            { column(formatMemoryKilobytesSeconds(props.container.total_resources?.total_memory || 0)) }
        </div>
    );
};