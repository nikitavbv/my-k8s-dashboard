import React, { useEffect, useState } from 'react';

import { Namespace } from '../types';

type NamespacesResponse = {
    namespaces: Namespace[],
};

export const Home = () => {
    const [namespaces, setNamespaces] = useState<Namespace[]>([]);

    useEffect(() => {
        fetch('http://localhost:8080/api/v1/namespaces')
            .then(r => r.json() as unknown as NamespacesResponse)
            .then(r => setNamespaces(r.namespaces));
    }, []);

    console.log('namespaces are', namespaces);    

    return (
        <React.Fragment>
            <div>Home</div>
            { namespaces.map(namespace => (<div>namespace: {namespace.name}</div>)) }
        </React.Fragment>
    );
};