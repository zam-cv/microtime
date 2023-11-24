const SOCKET_HOST = import.meta.env.VITE_APP_SOCKET_HOST || 'localhost';
const SOCKET_PORT = import.meta.env.VITE_APP_SOCKET_PORT || '4000';
export const SOCKET_URL = `ws://${SOCKET_HOST}:${SOCKET_PORT}/ws/`;