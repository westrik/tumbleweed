declare module '*.toml' {
    const value: typeof import('../todos_client/pkg/tumbleweed_todos_client');
    export default value;
}
