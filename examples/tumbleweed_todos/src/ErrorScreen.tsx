import { h } from 'preact';

interface ErrorScreenProps {
    errorDescriptionText?: string;
}

export default function ErrorScreen(props: ErrorScreenProps): h.JSX.Element {
    return (
        <div>
            <h1>{props.errorDescriptionText || '404'}</h1>
            <p>
                <a href="/">go home</a>
            </p>
        </div>
    );
}
