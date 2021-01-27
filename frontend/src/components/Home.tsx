import React, { useEffect, useState } from 'react';

import { Namespace } from '../types';
import { NamespaceRow } from './NamespaceRow';

type NamespacesResponse = {
    namespaces: Namespace[],
};

export const Home = () => {
    const [namespaces, setNamespaces] = useState<Namespace[]>([]);

    const column = (text: string | undefined) => (
        <div style={{ display: 'flex', textAlign: 'center', width: '100px', maxWidth: '100px', flex: '1', fontWeight: 600 }}>
            <span style={{ margin: '0 auto'}}>
                { text }
            </span>
        </div>
    );

    useEffect(() => {
        fetch('http://localhost:8080/api/v1/namespaces')
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
                { column('cpu') }
                { column('memory') }
            </div>
            { namespaces.map((namespace, i) => (<NamespaceRow namespace={namespace}/>)) }
        </React.Fragment>
    );
};