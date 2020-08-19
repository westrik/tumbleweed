import { h } from 'preact';
import Client from '../todos_client/Cargo.toml';

type Task = string;

export default function TaskList(): h.JSX.Element {
    const tasks: Array<Task> = ['my first task', 'my second task'];
    const resp = Client.get_value();
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
