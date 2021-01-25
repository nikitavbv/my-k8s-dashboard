import React from 'react';
import { ConnectedRouter } from 'connected-react-router';
import { Route, Switch } from 'react-router-dom';
import { history } from './store';

import { Header, Home } from './components';

const App = () => (
  <ConnectedRouter history={history}>
    <Header />
    <main style={{
        padding: '32px'
    }}>
      <Switch>
        <Route exact path='/' component={Home} />
      </Switch>
    </main>
  </ConnectedRouter>
);

export default App;
