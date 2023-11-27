const SOCKET_HOST = import.meta.env.VITE_APP_SOCKET_HOST || "localhost";
const SOCKET_PORT = import.meta.env.VITE_APP_SOCKET_PORT || "4000";
export const SOCKET_URL = `ws://${SOCKET_HOST}:${SOCKET_PORT}/ws/`;
export const DS18B20 = "ds18b20";

export const CHART_OPTIONS = {
  responsive: true,
  maintainAspectRatio: false,
  // aspectRatio: 5,
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
      display: false,
    },
  },
  animation: {
    duration: 200,
  },
};
