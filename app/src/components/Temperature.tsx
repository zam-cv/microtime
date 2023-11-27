import { useState, useEffect } from "react";
import { SOCKET_URL, DS18B20 } from "../constants";

interface Headers {
  timestamp: number,
}

interface Payload {
  temperature: number
}

interface Response {
  headers: Headers,
  payload: Payload
}

export default function Temperature() {
  let socket: WebSocket = new WebSocket(SOCKET_URL);
  let [temperature, setTemperature] = useState(0);

  useEffect(() => {
    socket.onopen = function (_) {
      socket.send(DS18B20);
    };

    socket.onmessage = function (event) {
      let response: Response = JSON.parse(event.data);
      setTemperature(Math.trunc(response.payload.temperature));
    };
  }, [socket]);

  return (
    <div className=" bg-blue-950 rounded-lg flex items-center justify-center">
      <div className=" flex gap-1">
        <p className="text-white text-4xl font-normal">{temperature}</p>
        <p className="text-white font-bold text-sm">°C</p>
      </div>
    </div>
  );
}