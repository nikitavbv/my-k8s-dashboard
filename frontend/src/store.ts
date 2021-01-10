import { createStore } from 'redux';
import createHistory from 'history/createBrowserHistory';

import { rootReducer } from './reducer';

export const history = createHistory();

export const store = createStore(
    rootReducer(history), JSON.parse(localStorage.state || '{}')
);

store.subscribe(() => {
    localStorage.state = JSON.stringify(store.getState());
});