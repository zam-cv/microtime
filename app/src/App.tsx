import "./App.css";
import { useState, useEffect } from "react";
import { SOCKET_URL } from "./constants";

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { faker } from "@faker-js/faker";
import { Line } from "react-chartjs-2";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

ChartJS.defaults.elements.point.pointStyle = false;

const labels = ["1", "2", "3", "4", "5", "6", "7"];

export const options = {
  responsive: true,
  maintainAspectRatio: true,
  aspectRatio: 5,
  scales: {
    x: {
      display: false,
    },
    y: {
      ticks: {
        color: "white",
        stepSize: 110,
        font: {
          size: 8,
        }
      },
      grid: {
        display: false,
      },
      min: -220,
      max: 220,
    },
  },
  plugins: {
    legend: {
      display: false,
    },
    title: {
      display: false,
    },
  },
};

export const data = {
  labels,
  datasets: [
    {
      data: labels.map(() => faker.number.int({ min: -220, max: 220 })),
      borderColor: "rgb(255, 99, 132)",
      backgroundColor: "rgba(255, 99, 132, 0.5)",
    },
  ],
};

function App() {
  let [message, setMessage] = useState("");
  let [values, setData] = useState(data);
  let socket: WebSocket = new WebSocket(SOCKET_URL);

  useEffect(() => {
    setInterval(() => {
      setData((prev) => {
        return {
          ...prev,
          datasets: [
            {
              ...prev.datasets[0],
              data: prev.datasets[0].data.map(() =>
                faker.number.int({ min: -220, max: 220 })
              ),
            },
          ],
        };
      });
    }, 2000);
  }, []);

  useEffect(() => {
    socket.onopen = function (_) {
      socket.send("max3010x");
    };

    socket.onmessage = function (event) {
      setMessage(event.data);
    };
  }, [socket]);

  return (
    <div className="w-screen h-screen bg-gradient-to-b from-blue-500 to-purple-500 p-5 grid grid-rows-[1fr_3fr_6fr_18fr] grid-cols-2 gap-5">
      <div className="flex col-span-2">
        <div className="flex items-center justify-center mr-1">
          <div className="p-2 flex items-center justify-center">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={2.5}
              stroke="currentColor"
              className="w-6 h-6 text-white"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"
              />
            </svg>
          </div>
        </div>
        <div className="flex items-center">
          <h1 className="font-bold text-white font-sans text-lg">
            Rafael Soto
          </h1>
        </div>
      </div>
      <div className="bg-blue-950 rounded-lg col-span-2 p-3">
        <Line data={values} options={options} />
      </div>
      <div className="bg-blue-950 rounded-lg"></div>
      <div className=" bg-blue-950 rounded-lg"></div>
      <div className="bg-blue-950 rounded-lg col-span-2 text-white">
        {message}
      </div>
    </div>
  );
}

export default App;
