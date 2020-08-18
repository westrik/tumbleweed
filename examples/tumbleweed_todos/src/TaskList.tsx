import { h } from 'preact';
// @ts-ignore TODO: load TS types
// eslint-disable-next-line @typescript-eslint/camelcase
import get_value from '../todos/todos_client/src/lib.rs';

type Task = string;

export default function TaskList(): h.JSX.Element {
    const tasks: Array<Task> = ['my first task', 'my second task'];
    const resp = get_value();
    console.log(resp);
    return (
        <div>
            <h1>Tasks</h1>
            <ul>
                {tasks.map((task, key) => (
                    <li key={key}>{task}</li>
                ))}
            </ul>
        </div>
    );
}
