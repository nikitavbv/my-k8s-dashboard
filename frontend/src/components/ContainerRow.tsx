import React from 'react';
import {Container} from '../types';

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
            padding: '8px 0 8px 48px',
            width: '100%',
            display: 'flex',
            boxSizing: 'border-box',
        }}>
            <span style={{ display: 'flex', width: 'auto', flex: '1' }}>Container: {props.container.name}</span>
            { column(props.container.usage?.cpu.toString()) }
            { column(props.container.usage?.memory.toString()) }
        </div>
    );
};