import { h } from 'preact';

type Task = string;

export default function TaskList(): h.JSX.Element {
    const tasks: Array<Task> = ['my first task', 'my second task'];
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
