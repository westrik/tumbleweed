import { h, render } from 'preact';
import Router, { Route } from 'preact-router';

export default function App(): h.JSX.Element {
    return (
        <Router>
            <Route path="/login" component={SignInForm} />
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
