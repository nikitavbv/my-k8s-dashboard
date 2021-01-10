import { connectRouter } from 'connected-react-router';
import { combineReducers } from 'redux';
import { History } from 'history';


export const rootReducer = (history: History) => combineReducers({
    router: connectRouter(history),
});