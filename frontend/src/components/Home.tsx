import React, { useEffect, useState } from 'react';

import { Namespace } from '../types';
import { NamespaceRow } from './NamespaceRow';
import {compareNamespaces} from '../utils';

type NamespacesResponse = {
    namespaces: Namespace[],
};

export const Home = () => {
    const [namespaces, setNamespaces] = useState<Namespace[]>([]);

    const column = (width: number, fontWeight: number) => (text: string | undefined) => (
        <div style={{ display: 'flex', textAlign: 'center', width: '100px', maxWidth: `${width}px`, flex: '1', fontWeight }}>
            <span style={{ margin: '0 auto'}}>
                { text }
            </span>
        </div>
    );

    const column100 = column(100, 400);
    const column200 = column(200, 600);

    useEffect(() => {
        fetch('/api/v1/namespaces')
            .then(r => r.json() as unknown as NamespacesResponse)
            .then(r => setNamespaces(r.namespaces));
    }, []);

    return (
        <React.Fragment>
            <div style={{
                padding: '0px 0 8px 48px',
                width: '100%',
                display: 'flex',
                boxSizing: 'border-box',
            }}>
                <span style={{ display: 'flex', width: 'auto', flex: '1' }}></span>
                { column200('usage') }
                { column200('requests') }
                { column200('limits') }
                { column200('total resources') }
            </div>
            <div style={{
                padding: '0px 0 8px 48px',
                width: '100%',
                display: 'flex',
                boxSizing: 'border-box',
            }}>
                <span style={{ display: 'flex', width: 'auto', flex: '1' }}></span>
                { column100('cpu') }
                { column100('memory') }
                { column100('cpu') }
                { column100('memory') }
                { column100('cpu') }
                { column100('memory') }
                { column100('cpu') }
                { column100('memory') }
            </div>
            { namespaces.sort(compareNamespaces).map((namespace, i) => (<NamespaceRow namespace={namespace}/>)) }
        </React.Fragment>
    );
};