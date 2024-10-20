import { createContext, JSXElement, useContext } from "solid-js";

const SocketContext = createContext<WebSocket | null>();

export function SockerProvider({
    children,
    onSocketOpen,
    onSocketClose,
    onSocketError,
    onSocketMessage,
}: {
    children: JSXElement;
    onSocketOpen?: (event: Event) => void;
    onSocketClose?: (event: CloseEvent) => void;
    onSocketError?: (event: Event) => void;
    onSocketMessage?: (event: MessageEvent) => void;
}) {
    const socket = new WebSocket("ws://localhost:3000"); // TODO

    socket.addEventListener("open", onSocketOpen);
    socket.addEventListener("close", onSocketClose);
    socket.addEventListener("error", onSocketError);
    socket.addEventListener("message", onSocketMessage);

    return <SocketContext.Provider value={socket}>{children}</SocketContext.Provider>;
}

export function useSocket() {
    return useContext(SocketContext);
}
