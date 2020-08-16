import { h, render } from 'preact';
import Router, { Route } from 'preact-router';

import ErrorScreen from '~ErrorScreen';
import TaskList from '~TaskList';

export default function App(): h.JSX.Element {
    return (
        <Router>
            <Route path="/" component={TaskList} />
            <Route default component={ErrorScreen} />
        </Router>
    );
}

render(<App />, document.getElementById('root')!);

// @ts-ignore
if (module.hot) {
    // @ts-ignore
    module.hot.accept();
}
