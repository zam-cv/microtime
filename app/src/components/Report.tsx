import { useState, useEffect } from "react";
import { SERVER_URL, SOCKET_URL, REPORT } from "../constants";
import axios from "axios";
import moment from "moment/moment";
import "moment/locale/es";

interface Headers {
  timestamp: number;
}

interface Payload {
  status: string;
  description: string;
}

interface Response {
  headers: Headers;
  payload: Payload;
}

export function Header() {
  return (
    <div className="bg-blue-900 grid-cols-[3fr_7fr] rounded-lg text-white font-medium grid p-3 gap-3">
      <p className="flex items-center justify-center">Fecha</p>
      <p className="flex items-center justify-center border-white border-l">
        Descripción
      </p>
    </div>
  );
}

const LIST: any = {
  warning: "bg-yellow-500",
  danger: "bg-red-500",
  normal: "bg-green-500",
};

export function Item({
  date,
  time,
  description,
  status,
}: {
  date: string;
  time: string;
  description: string;
  status: string;
}) {
  let [color, _] = useState(LIST[status] || "bg-green-500");

  return (
    <div className="bg-blue-900 grid grid-cols-[3fr_7fr_auto] rounded-lg p-3 gap-3 text-white">
      <div className="text-sm">
        <div className="font-bold">{date}</div> <div>{time}</div>
      </div>
      <div className="pl-7 text-sm flex items-center">{description}</div>
      <div className="flex items-center justify-center p-2">
        <div className={`rounded-xl h-2 w-2 ${color}`}></div>
      </div>
    </div>
  );
}

export default function Report() {
  let socket: WebSocket = new WebSocket(SOCKET_URL);
  let [reports, setReports] = useState<Response[]>([]);

  useEffect(() => {
    socket.onopen = function (_) {
      socket.send(REPORT);
    };

    socket.onmessage = function (event) {
      let response: Response = JSON.parse(event.data);
      setReports((reports) => [response, ...reports]);
    };
  }, []);

  useEffect(() => {
    axios.get(`${SERVER_URL}/report`).then((res: any) => {
      if (res.data) {
        setReports(res.data);
      }
    });
  }, []);

  return (
    <div className="bg-blue-950 rounded-lg col-span-3 p-5 grid grid-cols-1 grid-rows-[auto_1fr_6fr] gap-3 h-full overflow-auto">
      <p className="text-white text-center font-bold">Reporte</p>
      <Header />
      <div className="overflow-auto grid-cols-3 flex flex-col gap-3 h-full relative">
        {reports
          .sort((a, b) => b.headers.timestamp - a.headers.timestamp)
          .map((report, index) => {
            let date = moment
              .unix(report.headers.timestamp)
              .format("DD/MMM/YY");
            let list = date.split("/");
            list[1] = list[1].charAt(0).toUpperCase() + list[1].slice(1);
            list[1] = list[1].slice(0, -1);
            date = list.join("/");
            let time = moment.unix(report.headers.timestamp).format("HH:mm");

            return (
              <Item
                key={index}
                date={date}
                time={time}
                description={report.payload.description}
                status={report.payload.status}
              />
            );
          })}
      </div>
    </div>
  );
}
