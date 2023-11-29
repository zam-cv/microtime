const SOCKET_HOST = import.meta.env.VITE_APP_SOCKET_HOST || "localhost";
const SOCKET_PORT = import.meta.env.VITE_APP_SOCKET_PORT || "4000";
const SERVER_HOST = import.meta.env.VITE_APP_SERVER_HOST || "localhost";
const SERVER_PORT = import.meta.env.VITE_APP_SERVER_PORT || "4000";
export const SOCKET_URL = `ws://${SOCKET_HOST}:${SOCKET_PORT}/ws/`;
export const SERVER_URL = `http://${SERVER_HOST}:${SERVER_PORT}`;
export const DS18B20 = "ds18b20";

export const CHART_OPTIONS = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: {
      ticks: {
        color: "white",
        stepSize: 110,
        font: {
          size: 8,
        },
      },
      grid: {
        display: false,
      },
      display: false,
    },
    y: {
      ticks: {
        color: "white",
        stepSize: 110,
        font: {
          size: 8,
        },
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
      display: true,
      text: "",
      color: "white"
    },
  },
  animation: {
    duration: 200,
  },
};

export interface Options {
  responsive: boolean;
  maintainAspectRatio: boolean;
  scales: {
    x: {
      display: boolean;
    };
    y: {
      ticks: {
        color: string;
        stepSize: number;
        font: {
          size: number;
        };
      };
      grid: {
        display: boolean;
      };
      min: number;
      max: number;
    };
  };
  plugins: {
    legend: {
      display: boolean;
    };
    title: {
      display: boolean;
      text: string;
      color: string;
    };
  };
  animation: {
    duration: number;
  };
}